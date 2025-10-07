#include "../meshoptimizer/gltf/gltfpack.h"

// Forward declarations
extern Settings defaults();
extern int gltfpack(const char* input, const char* output, const char* report, Settings settings);

extern "C" {

// Compress with maximum compression (equivalent to gltfpack -cc)
int compress(const char* input, const char* output) {
    Settings settings = defaults();
    settings.compress = true;
    settings.compressmore = true;
    return gltfpack(input, output, NULL, settings);
}

}
