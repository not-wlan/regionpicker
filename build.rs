extern crate protoc_rust;
use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &[
            "protos/cstrike15_gcmessages.proto",
            "protos/cstrike15_usermessages.proto",
            "protos/engine_gcmessages.proto",
            "protos/steammessages.proto",
        ],
        includes: &["protos"],
        customize: Customize {
            ..Default::default()
        },
    })
    .expect("protoc");

    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-lib=static=steam_api");
    println!("cargo:rustc-link-lib=static=tier0");
}
