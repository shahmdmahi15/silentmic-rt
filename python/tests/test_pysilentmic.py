"""
Unit and integration tests for pysilentmic.
"""

import unittest
import math
from pysilentmic import SilentMic

class TestPySilentMic(unittest.TestCase):
    def setUp(self):
        self.mic = SilentMic(sample_rate=48000, attenuation_db=-48.0)

    def test_version(self):
        self.assertEqual(self.mic.version, "0.1.0")

    def test_process_frame(self):
        in_frame = [0.1] * 64
        out_frame = self.mic.process_frame(in_frame)
        self.assertEqual(len(out_frame), 64)
        for val in out_frame:
            self.assertTrue(math.isfinite(val))

    def test_process_stream(self):
        # 1000 samples of synthetic audio at 48kHz
        audio = [
            0.5 * math.sin(2 * math.pi * 440 * (i / 48000.0)) + 0.05 * ((i % 13) / 13.0)
            for i in range(1280)
        ]

        cleaned = self.mic.process(audio)
        self.assertEqual(len(cleaned), len(audio))
        for val in cleaned:
            self.assertTrue(math.isfinite(val))

        stats = self.mic.get_stats()
        self.assertGreater(stats["frames_processed"], 15)
        self.assertGreater(stats["avg_frame_micros"], 0.0)
        print(f"\n[PySilentMic Verified] Telemetry: {stats}")

if __name__ == "__main__":
    unittest.main()
