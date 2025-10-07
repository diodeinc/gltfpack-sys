#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// Compress with maximum compression (equivalent to gltfpack -cc)
int compress(const char* input, const char* output);

#ifdef __cplusplus
}
#endif
