use approx::assert_relative_eq;
use realfft::num_complex::Complex32;
use silentmic::{ErbFilterbank, StftProcessor, FFT_BINS, HOP_SIZE, NUM_ERB_BANDS, SAMPLE_RATE};

#[test]
fn test_stft_perfect_reconstruction() {
    let mut stft = StftProcessor::new();

    // Generate 20 frames (1280 samples) of multi-frequency audio
    let num_frames = 20;
    let total_samples = num_frames * HOP_SIZE;
    let mut original = vec![0.0f32; total_samples];

    for (i, sample) in original.iter_mut().enumerate() {
        let t = i as f32 / SAMPLE_RATE as f32;
        // Superposition of 250 Hz, 1000 Hz, and 4000 Hz
        *sample = 0.3 * (2.0 * std::f32::consts::PI * 250.0 * t).sin()
            + 0.2 * (2.0 * std::f32::consts::PI * 1000.0 * t).sin()
            + 0.1 * (2.0 * std::f32::consts::PI * 4000.0 * t).sin();
    }

    let mut reconstructed = vec![0.0f32; total_samples];
    let mut in_frame = [0.0f32; HOP_SIZE];
    let mut out_frame = [0.0f32; HOP_SIZE];

    // Identity pass-through (no spectral modification)
    for f in 0..num_frames {
        let start = f * HOP_SIZE;
        let end = start + HOP_SIZE;
        in_frame.copy_from_slice(&original[start..end]);

        stft.process_frame(&in_frame, &mut out_frame, |_in_spec, out_spec| {
            // Identity pass-through
            out_spec.copy_from_slice(_in_spec);
        });

        reconstructed[start..end].copy_from_slice(&out_frame);
    }

    // Skip the first frame (hop 0) due to initial overlap delay line fill
    let eval_start = HOP_SIZE * 2;
    let eval_end = total_samples - HOP_SIZE;

    let mut max_err = 0.0f32;
    let mut sum_sq_err = 0.0f32;
    let eval_count = eval_end - eval_start;

    for i in eval_start..eval_end {
        let err = (original[i - HOP_SIZE] - reconstructed[i]).abs();
        if err > max_err {
            max_err = err;
        }
        sum_sq_err += err * err;
    }

    let rmse = (sum_sq_err / eval_count as f32).sqrt();
    println!("STFT Reconstruction RMSE: {:.8}, Max Error: {:.8}", rmse, max_err);

    // Overlap-add with sine window achieves bit-accurate reconstruction within float precision
    assert!(rmse < 1e-4, "STFT reconstruction error exceeds threshold: {}", rmse);
    assert!(max_err < 1e-3, "STFT max reconstruction error exceeds threshold: {}", max_err);
}

#[test]
fn test_erb_filterbank_geometry() {
    let erb = ErbFilterbank::new();
    let freqs = erb.center_frequencies();

    assert_eq!(freqs.len(), NUM_ERB_BANDS);

    // Center frequencies must be strictly increasing
    for i in 1..NUM_ERB_BANDS {
        assert!(
            freqs[i] > freqs[i - 1],
            "ERB band {} ({} Hz) not strictly greater than band {} ({} Hz)",
            i, freqs[i], i - 1, freqs[i - 1]
        );
    }

    // Frequency boundaries
    assert!(freqs[0] >= 50.0);
    assert!(freqs[NUM_ERB_BANDS - 1] <= 20000.0);

    // Test log energy extraction
    let mut spectrum = [Complex32::new(0.0, 0.0); FFT_BINS];
    spectrum[4] = Complex32::new(10.0, 5.0);

    let mut log_energies = [0.0f32; NUM_ERB_BANDS];
    erb.extract_log_energies(&spectrum, &mut log_energies);

    for (i, &energy) in log_energies.iter().enumerate() {
        assert!(energy.is_finite(), "ERB band {} log energy is not finite", i);
    }

    // Test gain synthesis
    let unity_gains = [1.0f32; NUM_ERB_BANDS];
    let mut bin_gains = [0.0f32; FFT_BINS];
    erb.synthesize_bin_gains(&unity_gains, &mut bin_gains);

    for &_gain in bin_gains.iter() {
        assert_relative_eq!(_gain, 1.0, epsilon = 1e-2);
    }
}
