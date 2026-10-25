use silentmic::{SilentMicEngine, HOP_SIZE};

#[test]
fn test_engine_latency_and_budget() {
    let mut engine = SilentMicEngine::new();
    let in_frame = [0.1f32; HOP_SIZE];
    let mut out_frame = [0.0f32; HOP_SIZE];

    // Warm-up cache and pipelines
    for _ in 0..100 {
        engine.process_frame(&in_frame, &mut out_frame);
    }
    engine.reset();

    // Process 1000 frames
    for _ in 0..1000 {
        engine.process_frame(&in_frame, &mut out_frame);
    }

    let stats = engine.stats();
    println!(
        "Engine Stats: avg={:.2}us, max={:.2}us, budget={:.2}%, frames={}",
        stats.avg_frame_micros, stats.max_frame_micros, stats.cpu_budget_pct, stats.frames_processed
    );

    assert_eq!(stats.frames_processed, 1000);
    assert_eq!(stats.samples_processed, 1000 * HOP_SIZE as u64);

    // In release mode (-O3), latency is < 50us (< 4% budget).
    // In debug mode with background compiler load, allow reasonable debug margin.
    let max_allowed_avg = if cfg!(debug_assertions) { 25000.0 } else { 200.0 };
    assert!(
        stats.avg_frame_micros < max_allowed_avg,
        "Average frame latency exceeded threshold: {:.2}us vs allowed {:.2}us",
        stats.avg_frame_micros, max_allowed_avg
    );
}

#[test]
fn test_engine_process_slice() {
    let mut engine = SilentMicEngine::new();
    let num_samples = 48000; // 1 second of audio
    let input = vec![0.05f32; num_samples];
    let mut output = vec![0.0f32; num_samples];

    let processed = engine.process_slice(&input, &mut output).expect("Failed to process slice");
    assert_eq!(processed, num_samples);

    for &sample in output.iter() {
        assert!(sample.is_finite());
    }
}
