"""
SilentMic-RT Python ctypes binding.
Ultra-low latency sub-3ms neural voice isolator & noise suppressor.
"""

import ctypes
import os
import sys
from typing import Optional, Union, Dict, Any, List

try:
    import numpy as np
    HAS_NUMPY = True
except ImportError:
    HAS_NUMPY = False

class SilentMicStatsC(ctypes.Structure):
    _fields_ = [
        ("frames_processed", ctypes.c_uint64),
        ("samples_processed", ctypes.c_uint64),
        ("last_frame_micros", ctypes.c_double),
        ("max_frame_micros", ctypes.c_double),
        ("avg_frame_micros", ctypes.c_double),
        ("cpu_budget_pct", ctypes.c_double),
    ]

def _find_library() -> str:
    """Locates the compiled SilentMic dynamic library."""
    lib_names = []
    if sys.platform.startswith("linux"):
        lib_names = ["libsilentmic.so"]
    elif sys.platform == "darwin":
        lib_names = ["libsilentmic.dylib"]
    elif sys.platform == "win32":
        lib_names = ["silentmic.dll", "libsilentmic.dll"]
    else:
        lib_names = ["libsilentmic.so"]

    search_dirs = [
        os.path.dirname(os.path.abspath(__file__)),
        os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "target", "release"),
        os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "target", "debug"),
        "/usr/local/lib",
        "/usr/lib",
    ]

    for d in search_dirs:
        for name in lib_names:
            path = os.path.join(d, name)
            if os.path.isfile(path):
                return os.path.abspath(path)

    return lib_names[0]

class SilentMic:
    """
    SilentMic Real-Time Neural Voice Isolator Engine.
    
    Processes 48kHz audio in 64-sample frames with deterministic sub-3ms algorithmic latency.
    """
    FRAME_SIZE = 64
    SAMPLE_RATE = 48000

    def __init__(self, sample_rate: int = 48000, attenuation_db: float = -48.0, lib_path: Optional[str] = None):
        if sample_rate != self.SAMPLE_RATE:
            raise ValueError(f"SilentMic requires sample_rate={self.SAMPLE_RATE} Hz, received {sample_rate} Hz")

        dll_path = lib_path or _find_library()
        self._lib = ctypes.CDLL(dll_path)

        # Declare function signatures
        self._lib.silentmic_create.argtypes = [ctypes.c_uint32]
        self._lib.silentmic_create.restype = ctypes.c_void_p

        self._lib.silentmic_process_frame.argtypes = [
            ctypes.c_void_p,
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
        ]
        self._lib.silentmic_process_frame.restype = ctypes.c_int

        self._lib.silentmic_set_attenuation_db.argtypes = [ctypes.c_void_p, ctypes.c_float]
        self._lib.silentmic_set_attenuation_db.restype = ctypes.c_int

        self._lib.silentmic_reset.argtypes = [ctypes.c_void_p]
        self._lib.silentmic_reset.restype = ctypes.c_int

        self._lib.silentmic_get_stats.argtypes = [ctypes.c_void_p, ctypes.POINTER(SilentMicStatsC)]
        self._lib.silentmic_get_stats.restype = ctypes.c_int

        self._lib.silentmic_destroy.argtypes = [ctypes.c_void_p]
        self._lib.silentmic_destroy.restype = None

        self._lib.silentmic_version.argtypes = []
        self._lib.silentmic_version.restype = ctypes.c_char_p

        self._handle = self._lib.silentmic_create(sample_rate)
        if not self._handle:
            raise RuntimeError("Failed to create native SilentMic-RT engine instance")

        self.set_attenuation_db(attenuation_db)

    def set_attenuation_db(self, db: float) -> None:
        """Sets the maximum noise floor attenuation in decibels (e.g. -48.0 dB)."""
        ret = self._lib.silentmic_set_attenuation_db(self._handle, ctypes.c_float(db))
        if ret != 0:
            raise RuntimeError("Failed to set attenuation level")

    def reset(self) -> None:
        """Resets all internal delay lines and recurrent states."""
        self._lib.silentmic_reset(self._handle)

    def process_frame(self, in_frame: Union[Any, List[float]]) -> Union[Any, List[float]]:
        """
        Processes an exact 64-sample float32 array or list.
        """
        if HAS_NUMPY and isinstance(in_frame, np.ndarray):
            if in_frame.shape != (self.FRAME_SIZE,) or in_frame.dtype != np.float32:
                in_frame = np.ascontiguousarray(in_frame, dtype=np.float32)
                if in_frame.shape != (self.FRAME_SIZE,):
                    raise ValueError(f"in_frame must have shape ({self.FRAME_SIZE},), got {in_frame.shape}")

            out_frame = np.empty(self.FRAME_SIZE, dtype=np.float32)
            in_ptr = in_frame.ctypes.data_as(ctypes.POINTER(ctypes.c_float))
            out_ptr = out_frame.ctypes.data_as(ctypes.POINTER(ctypes.c_float))

            ret = self._lib.silentmic_process_frame(self._handle, in_ptr, out_ptr)
            if ret != 0:
                raise RuntimeError(f"silentmic_process_frame returned error code {ret}")
            return out_frame
        else:
            in_list = list(in_frame)
            if len(in_list) != self.FRAME_SIZE:
                raise ValueError(f"in_frame must have length {self.FRAME_SIZE}, got {len(in_list)}")

            c_in = (ctypes.c_float * self.FRAME_SIZE)(*in_list)
            c_out = (ctypes.c_float * self.FRAME_SIZE)()

            ret = self._lib.silentmic_process_frame(self._handle, c_in, c_out)
            if ret != 0:
                raise RuntimeError(f"silentmic_process_frame returned error code {ret}")
            return [float(x) for x in c_out]

    def process(self, audio: Union[Any, List[float]]) -> Union[Any, List[float]]:
        """
        Denoises an arbitrary length audio array in streaming 64-sample hops.
        """
        if HAS_NUMPY and isinstance(audio, np.ndarray):
            arr = np.ascontiguousarray(audio, dtype=np.float32).flatten()
            total_len = len(arr)
            output = np.empty(total_len, dtype=np.float32)

            full_hops = total_len // self.FRAME_SIZE
            for i in range(full_hops):
                start = i * self.FRAME_SIZE
                end = start + self.FRAME_SIZE
                output[start:end] = self.process_frame(arr[start:end])

            rem = total_len % self.FRAME_SIZE
            if rem > 0:
                pad_frame = np.zeros(self.FRAME_SIZE, dtype=np.float32)
                pad_frame[:rem] = arr[full_hops * self.FRAME_SIZE :]
                cleaned_pad = self.process_frame(pad_frame)
                output[full_hops * self.FRAME_SIZE :] = cleaned_pad[:rem]

            return output
        else:
            arr = list(audio)
            total_len = len(arr)
            output = [0.0] * total_len

            full_hops = total_len // self.FRAME_SIZE
            for i in range(full_hops):
                start = i * self.FRAME_SIZE
                end = start + self.FRAME_SIZE
                cleaned = self.process_frame(arr[start:end])
                output[start:end] = cleaned

            rem = total_len % self.FRAME_SIZE
            if rem > 0:
                pad = arr[full_hops * self.FRAME_SIZE :] + [0.0] * (self.FRAME_SIZE - rem)
                cleaned = self.process_frame(pad)
                output[full_hops * self.FRAME_SIZE :] = cleaned[:rem]

            return output

    def get_stats(self) -> Dict[str, Any]:
        """Returns real-time execution telemetry and latency profiling metrics."""
        stats = SilentMicStatsC()
        ret = self._lib.silentmic_get_stats(self._handle, ctypes.byref(stats))
        if ret != 0:
            return {}
        return {
            "frames_processed": stats.frames_processed,
            "samples_processed": stats.samples_processed,
            "last_frame_micros": stats.last_frame_micros,
            "max_frame_micros": stats.max_frame_micros,
            "avg_frame_micros": stats.avg_frame_micros,
            "cpu_budget_pct": stats.cpu_budget_pct,
        }

    @property
    def version(self) -> str:
        """Returns the native library version string."""
        ver = self._lib.silentmic_version()
        return ver.decode("utf-8") if ver else "unknown"

    def __del__(self):
        if hasattr(self, "_handle") and self._handle:
            self._lib.silentmic_destroy(self._handle)
            self._handle = None
