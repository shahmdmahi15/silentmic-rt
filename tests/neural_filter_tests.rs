use realfft::num_complex::Complex32;
use silentmic::{NeuralFilter, FFT_BINS};

#[test]
fn test_neural_filter_deterministic_execution() {
    let mut filter1 = NeuralFilter::new();
    let mut filter2 = NeuralFilter::new();

    let mut spec1 = [Complex32::new(0.5, -0.3); FFT_BINS];
    let mut spec2 = [Complex32::new(0.5, -0.3); FFT_BINS];

    for _ in 0..10 {
        filter1.process_spectrum(&mut spec1);
        filter2.process_spectrum(&mut spec2);

        for b in 0..FFT_BINS {
            assert_eq!(
                spec1[b].re, spec2[b].re,
                "Non-deterministic real component at bin {}", b
            );
            assert_eq!(
                spec1[b].im, spec2[b].im,
                "Non-deterministic imaginary component at bin {}", b
            );
        }
    }
}

#[test]
fn test_neural_filter_silence_preservation() {
    let mut filter = NeuralFilter::new();
    let mut spec = [Complex32::new(0.0, 0.0); FFT_BINS];

    for _ in 0..20 {
        filter.process_spectrum(&mut spec);
        for b in 0..FFT_BINS {
            assert_eq!(spec[b].re, 0.0);
            assert_eq!(spec[b].im, 0.0);
        }
    }
}

#[test]
fn test_neural_filter_noise_attenuation() {
    let mut filter = NeuralFilter::new();
    filter.set_attenuation_db(-40.0);

    // Feed high-frequency noise spikes (keyboard click / hiss) at bin 55 (> 20 kHz)
    let mut spec = [Complex32::new(0.0, 0.0); FFT_BINS];
    spec[55] = Complex32::new(5.0, 5.0);

    let initial_mag = (spec[55].re * spec[55].re + spec[55].im * spec[55].im).sqrt();

    // Process several frames
    for _ in 0..5 {
        filter.process_spectrum(&mut spec);
    }

    let final_mag = (spec[55].re * spec[55].re + spec[55].im * spec[55].im).sqrt();
    println!("Initial Noise Mag: {:.4}, Final Suppressed Mag: {:.4}", initial_mag, final_mag);

    assert!(
        final_mag < initial_mag * 0.5,
        "Noise at 20 kHz was not attenuated: initial {}, final {}",
        initial_mag, final_mag
    );
}
