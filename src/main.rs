use clap::Parser;
use std::process;

use gltfpack_sys::compress;

#[derive(Parser, Debug)]
#[command(name = "gltfpack-rs")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Compress glTF files (equivalent to gltfpack -cc)", long_about = None)]
struct Args {
    /// Input file to process (.obj/.gltf/.glb)
    #[arg(short, long)]
    input: String,

    /// Output file path (.gltf/.glb)
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    if let Err(code) = compress(&args.input, &args.output) {
        eprintln!("gltfpack failed with error code: {}", code);
        process::exit(code);
    }
}
