use std::path::PathBuf;

fn main() {
    let meshopt_dir = PathBuf::from("meshoptimizer");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .define("GLTFFUZZ", None) // Skip main() function in gltfpack.cpp
        .include(&meshopt_dir)
        .include(meshopt_dir.join("src"))
        .include(meshopt_dir.join("extern"))
        // meshoptimizer core
        .file(meshopt_dir.join("src/allocator.cpp"))
        .file(meshopt_dir.join("src/clusterizer.cpp"))
        .file(meshopt_dir.join("src/indexanalyzer.cpp"))
        .file(meshopt_dir.join("src/indexcodec.cpp"))
        .file(meshopt_dir.join("src/indexgenerator.cpp"))
        .file(meshopt_dir.join("src/overdrawoptimizer.cpp"))
        .file(meshopt_dir.join("src/partition.cpp"))
        .file(meshopt_dir.join("src/quantization.cpp"))
        .file(meshopt_dir.join("src/rasterizer.cpp"))
        .file(meshopt_dir.join("src/simplifier.cpp"))
        .file(meshopt_dir.join("src/spatialorder.cpp"))
        .file(meshopt_dir.join("src/stripifier.cpp"))
        .file(meshopt_dir.join("src/vcacheoptimizer.cpp"))
        .file(meshopt_dir.join("src/vertexcodec.cpp"))
        .file(meshopt_dir.join("src/vertexfilter.cpp"))
        .file(meshopt_dir.join("src/vfetchoptimizer.cpp"))
        // gltfpack
        .file(meshopt_dir.join("gltf/animation.cpp"))
        .file(meshopt_dir.join("gltf/encodebasis.cpp"))
        .file(meshopt_dir.join("gltf/encodewebp.cpp"))
        .file(meshopt_dir.join("gltf/fileio.cpp"))
        .file(meshopt_dir.join("gltf/gltfpack.cpp"))
        .file(meshopt_dir.join("gltf/image.cpp"))
        .file(meshopt_dir.join("gltf/json.cpp"))
        .file(meshopt_dir.join("gltf/material.cpp"))
        .file(meshopt_dir.join("gltf/mesh.cpp"))
        .file(meshopt_dir.join("gltf/node.cpp"))
        .file(meshopt_dir.join("gltf/parseobj.cpp"))
        .file(meshopt_dir.join("gltf/parselib.cpp"))
        .file(meshopt_dir.join("gltf/parsegltf.cpp"))
        .file(meshopt_dir.join("gltf/stream.cpp"))
        .file(meshopt_dir.join("gltf/write.cpp"))
        .file("wrapper.cpp");

    // Suppress warnings from third-party code
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        build.flag("-Wno-missing-field-initializers");
        build.flag("-Wno-unused-parameter");
    }

    build.compile("gltfpack");

    // Rerun if wrapper changes
    println!("cargo:rerun-if-changed=wrapper.cpp");
}
