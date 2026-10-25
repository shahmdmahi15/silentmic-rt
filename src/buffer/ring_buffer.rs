//! Zero-allocation lock-free Single-Producer Single-Consumer (SPSC) ring buffer.
//!
//! Designed specifically for low-latency real-time audio threads. Uses 64-byte cacheline
//! padding to eliminate false sharing between the capture thread (producer) and the DSP
//! processing thread (consumer).

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use crate::error::{Result, SilentMicError};

/// Align helper struct to prevent false sharing between atomic cache lines.
#[repr(align(64))]
struct CachelineAligned<T>(pub T);

/// Lock-free SPSC Ring Buffer buffer storage.
pub struct SpscRingBuffer {
    capacity: usize,
    mask: usize,
    buffer: Box<[f32]>,
    // Write index modified exclusively by Producer
    write_head: CachelineAligned<AtomicUsize>,
    // Read index modified exclusively by Consumer
    read_tail: CachelineAligned<AtomicUsize>,
}

// Safety: SpscRingBuffer safely splits into Producer and Consumer or can be shared via Arc.
unsafe impl Send for SpscRingBuffer {}
unsafe impl Sync for SpscRingBuffer {}

impl SpscRingBuffer {
    /// Creates a new ring buffer with the given capacity rounded up to the nearest power of 2.
    pub fn new(requested_capacity: usize) -> Self {
        let capacity = requested_capacity.next_power_of_two().max(64);
        let mask = capacity - 1;
        let mut vec = Vec::with_capacity(capacity);
        vec.resize(capacity, 0.0f32);

        Self {
            capacity,
            mask,
            buffer: vec.into_boxed_slice(),
            write_head: CachelineAligned(AtomicUsize::new(0)),
            read_tail: CachelineAligned(AtomicUsize::new(0)),
        }
    }

    /// Total capacity in float samples.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Number of samples currently available to read without blocking.
    #[inline(always)]
    pub fn available_read(&self) -> usize {
        let head = self.write_head.0.load(Ordering::Acquire);
        let tail = self.read_tail.0.load(Ordering::Relaxed);
        head.wrapping_sub(tail)
    }

    /// Number of samples currently available to write before overflow.
    #[inline(always)]
    pub fn available_write(&self) -> usize {
        self.capacity - self.available_read()
    }

    /// Pushes a single audio sample into the buffer.
    ///
    /// # Errors
    /// Returns `BufferOverflow` if the buffer is currently full.
    #[inline(always)]
    pub fn push(&self, sample: f32) -> Result<()> {
        let head = self.write_head.0.load(Ordering::Relaxed);
        let tail = self.read_tail.0.load(Ordering::Acquire);

        if head.wrapping_sub(tail) >= self.capacity {
            return Err(SilentMicError::BufferOverflow {
                written: 1,
                available: 0,
            });
        }

        let idx = head & self.mask;
        unsafe {
            let ptr = self.buffer.as_ptr() as *mut f32;
            *ptr.add(idx) = sample;
        }

        self.write_head.0.store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Pushes a slice of audio samples. Returns the number of samples actually written.
    #[inline]
    pub fn push_slice(&self, slice: &[f32]) -> usize {
        let to_write = slice.len();
        if to_write == 0 {
            return 0;
        }

        let head = self.write_head.0.load(Ordering::Relaxed);
        let tail = self.read_tail.0.load(Ordering::Acquire);
        let occupied = head.wrapping_sub(tail);
        let free = self.capacity.saturating_sub(occupied);

        let actual = to_write.min(free);
        if actual == 0 {
            return 0;
        }

        let start_idx = head & self.mask;
        let first_chunk = (self.capacity - start_idx).min(actual);
        let second_chunk = actual - first_chunk;

        unsafe {
            let raw_buf = self.buffer.as_ptr() as *mut f32;
            std::ptr::copy_nonoverlapping(slice.as_ptr(), raw_buf.add(start_idx), first_chunk);
            if second_chunk > 0 {
                std::ptr::copy_nonoverlapping(slice.as_ptr().add(first_chunk), raw_buf, second_chunk);
            }
        }

        self.write_head.0.store(head.wrapping_add(actual), Ordering::Release);
        actual
    }

    /// Pops a single audio sample from the buffer.
    #[inline(always)]
    pub fn pop(&self) -> Option<f32> {
        let tail = self.read_tail.0.load(Ordering::Relaxed);
        let head = self.write_head.0.load(Ordering::Acquire);

        if tail == head {
            return None;
        }

        let idx = tail & self.mask;
        let sample = unsafe { *self.buffer.as_ptr().add(idx) };
        self.read_tail.0.store(tail.wrapping_add(1), Ordering::Release);
        Some(sample)
    }

    /// Pops up to `dest.len()` samples into `dest`. Returns the count read.
    #[inline]
    pub fn pop_slice(&self, dest: &mut [f32]) -> usize {
        let to_read = dest.len();
        if to_read == 0 {
            return 0;
        }

        let tail = self.read_tail.0.load(Ordering::Relaxed);
        let head = self.write_head.0.load(Ordering::Acquire);
        let available = head.wrapping_sub(tail);

        let actual = to_read.min(available);
        if actual == 0 {
            return 0;
        }

        let start_idx = tail & self.mask;
        let first_chunk = (self.capacity - start_idx).min(actual);
        let second_chunk = actual - first_chunk;

        unsafe {
            let raw_buf = self.buffer.as_ptr();
            std::ptr::copy_nonoverlapping(raw_buf.add(start_idx), dest.as_mut_ptr(), first_chunk);
            if second_chunk > 0 {
                std::ptr::copy_nonoverlapping(raw_buf, dest.as_mut_ptr().add(first_chunk), second_chunk);
            }
        }

        self.read_tail.0.store(tail.wrapping_add(actual), Ordering::Release);
        actual
    }

    /// Discards all currently queued audio samples and resets indices.
    pub fn reset(&self) {
        let head = self.write_head.0.load(Ordering::Relaxed);
        self.read_tail.0.store(head, Ordering::Release);
    }
}

/// Producer handle for writing samples.
pub struct Producer {
    inner: Arc<SpscRingBuffer>,
}

/// Consumer handle for reading samples.
pub struct Consumer {
    inner: Arc<SpscRingBuffer>,
}

unsafe impl Send for Producer {}
unsafe impl Send for Consumer {}

impl Producer {
    #[inline(always)]
    pub fn push(&self, sample: f32) -> Result<()> {
        self.inner.push(sample)
    }

    #[inline(always)]
    pub fn push_slice(&self, slice: &[f32]) -> usize {
        self.inner.push_slice(slice)
    }

    #[inline(always)]
    pub fn available_write(&self) -> usize {
        self.inner.available_write()
    }
}

impl Consumer {
    #[inline(always)]
    pub fn pop(&self) -> Option<f32> {
        self.inner.pop()
    }

    #[inline(always)]
    pub fn pop_slice(&self, dest: &mut [f32]) -> usize {
        self.inner.pop_slice(dest)
    }

    #[inline(always)]
    pub fn available_read(&self) -> usize {
        self.inner.available_read()
    }
}

/// Creates a split Producer and Consumer pair sharing an underlying zero-allocation ring buffer.
pub fn make_spsc_ring_buffer(capacity: usize) -> (Producer, Consumer) {
    let buf = Arc::new(SpscRingBuffer::new(capacity));
    (
        Producer { inner: Arc::clone(&buf) },
        Consumer { inner: buf },
    )
}
