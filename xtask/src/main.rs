use std::{fs, process::Command};

fn main() {
    let status = Command::new("cargo")
        .args([
            "build",
            "-p",
            "hello_rust_wasm",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()
        .unwrap();

    assert!(status.success());

    let wasm = fs::read("target/wasm32-unknown-unknown/release/hello_rust_wasm.wasm").unwrap();

    let wasm_base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, wasm);

    let template = fs::read_to_string("template.html").unwrap();

    let html = template.replace("{{WASM}}", &wasm_base64);

    fs::create_dir_all("dist").unwrap();
    fs::write("target/index.html", html).unwrap();
}
