/**
 * @file silentmic.h
 * @brief SilentMic-RT C99 API Header
 *
 * Deterministic, Zero-Allocation Sub-3ms Hard Real-Time Neural Voice Isolator.
 * Designed for embedding into C/C++, OBS plugins, VoIP servers, and audio engines.
 *
 * Author: Shah Md. Mahi <shahmdmahi15@gmail.com>
 * License: MIT
 */

#ifndef SILENTMIC_H
#define SILENTMIC_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>

#define SILENTMIC_SAMPLE_RATE 48000
#define SILENTMIC_FRAME_SIZE  64
#define SILENTMIC_LATENCY_MS  1.333333f

/**
 * @brief Opaque pointer handle to a SilentMic engine instance.
 */
typedef struct SilentMicEngine SilentMicEngine;
typedef SilentMicEngine* silentmic_t;

/**
 * @brief Real-time telemetry metrics per frame.
 */
typedef struct {
    uint64_t frames_processed;
    uint64_t samples_processed;
    double last_frame_micros;
    double max_frame_micros;
    double avg_frame_micros;
    double cpu_budget_pct;
} silentmic_stats_t;

/**
 * @brief Creates a new SilentMic engine instance.
 * @param sample_rate Must be 48000.
 * @return Handle pointer on success, NULL on failure.
 */
silentmic_t silentmic_create(uint32_t sample_rate);

/**
 * @brief Processes an exact 64-sample audio frame in real time.
 * @param handle Valid SilentMic handle.
 * @param in_frame Pointer to 64 input float32 samples.
 * @param out_frame Pointer to 64 output float32 samples.
 * @return 0 on success, negative error code on failure.
 */
int silentmic_process_frame(silentmic_t handle, const float* in_frame, float* out_frame);

/**
 * @brief Configures maximum noise attenuation floor in decibels (e.g. -48.0f).
 * @param handle Valid SilentMic handle.
 * @param db Attenuation limit in decibels.
 * @return 0 on success, negative error code on failure.
 */
int silentmic_set_attenuation_db(silentmic_t handle, float db);

/**
 * @brief Resets all internal delay lines and recurrent states.
 * @param handle Valid SilentMic handle.
 * @return 0 on success, negative on failure.
 */
int silentmic_reset(silentmic_t handle);

/**
 * @brief Retrieves runtime performance statistics.
 * @param handle Valid SilentMic handle.
 * @param stats Pointer to output statistics struct.
 * @return 0 on success, negative on failure.
 */
int silentmic_get_stats(silentmic_t handle, silentmic_stats_t* stats);

/**
 * @brief Frees and destroys a SilentMic engine instance.
 * @param handle Valid SilentMic handle.
 */
void silentmic_destroy(silentmic_t handle);

/**
 * @brief Returns the version string of the SilentMic library.
 * @return NUL-terminated constant string.
 */
const char* silentmic_version(void);

#ifdef __cplusplus
}
#endif

#endif /* SILENTMIC_H */
