use std::thread;
use silentmic::buffer::{make_spsc_ring_buffer, SpscRingBuffer};

#[test]
fn test_ring_buffer_single_thread() {
    let buf = SpscRingBuffer::new(128);
    assert_eq!(buf.capacity(), 128);
    assert_eq!(buf.available_read(), 0);
    assert_eq!(buf.available_write(), 128);

    // Push 32 samples
    let samples_in: Vec<f32> = (0..32).map(|x| x as f32).collect();
    let written = buf.push_slice(&samples_in);
    assert_eq!(written, 32);
    assert_eq!(buf.available_read(), 32);

    // Pop 32 samples
    let mut samples_out = vec![0.0f32; 32];
    let read = buf.pop_slice(&mut samples_out);
    assert_eq!(read, 32);
    assert_eq!(samples_in, samples_out);
    assert_eq!(buf.available_read(), 0);
}

#[test]
fn test_ring_buffer_lock_free_spsc_concurrency() {
    let (producer, consumer) = make_spsc_ring_buffer(4096);
    let total_samples = 1_000_000;

    let prod_handle = thread::spawn(move || {
        let chunk_size = 64;
        let mut sample_val = 0.0f32;
        let mut buffer = [0.0f32; 64];

        let mut sent = 0;
        while sent < total_samples {
            for item in buffer.iter_mut() {
                *item = sample_val;
                sample_val += 1.0;
            }

            let mut written = 0;
            while written < chunk_size {
                let w = producer.push_slice(&buffer[written..]);
                written += w;
                if written < chunk_size {
                    std::thread::yield_now();
                }
            }
            sent += chunk_size;
        }
    });

    let cons_handle = thread::spawn(move || {
        let chunk_size = 64;
        let mut buffer = [0.0f32; 64];
        let mut received = 0;
        let mut expected_val = 0.0f32;

        while received < total_samples {
            let mut read = 0;
            while read < chunk_size {
                let r = consumer.pop_slice(&mut buffer[read..]);
                read += r;
                if read < chunk_size {
                    std::thread::yield_now();
                }
            }

            for &val in buffer.iter() {
                assert_eq!(
                    val, expected_val,
                    "Data corruption detected in lock-free ring buffer! Expected {}, got {}",
                    expected_val, val
                );
                expected_val += 1.0;
            }
            received += chunk_size;
        }
    });

    prod_handle.join().expect("Producer thread panicked");
    cons_handle.join().expect("Consumer thread panicked");
}
