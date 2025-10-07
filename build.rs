use std::path::PathBuf;

fn main() {
    let meshopt_dir = PathBuf::from("meshoptimizer");

    // Build meshoptimizer library
    let meshopt_lib = cmake::Config::new(&meshopt_dir)
        .define("MESHOPT_BUILD_DEMO", "OFF")
        .define("MESHOPT_BUILD_GLTFPACK", "OFF")
        .define("MESHOPT_BUILD_SHARED_LIBS", "OFF")
        .build();

    // Tell cargo to link meshoptimizer
    println!(
        "cargo:rustc-link-search=native={}/lib",
        meshopt_lib.display()
    );
    println!("cargo:rustc-link-lib=static=meshoptimizer");

    // Compile gltf sources
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .include(&meshopt_dir)
        .include(meshopt_dir.join("src"))
        .include(meshopt_dir.join("extern"))
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
