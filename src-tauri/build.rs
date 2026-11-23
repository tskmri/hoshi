use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    load_sync_credentials();
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        build_swift();
    }
    tauri_build::build();
}

/// Reads the Google Drive OAuth client from the gitignored `.env` at the
/// project root, so the credentials never live in the source tree.
fn load_sync_credentials() {
    for path in ["../.env", "../.env.local"] {
        let Ok(contents) = fs::read_to_string(path) else {
            continue;
        };
        println!("cargo:rerun-if-changed={path}");
        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            if !matches!(key, "HOSHI_GOOGLE_CLIENT_ID" | "HOSHI_GOOGLE_CLIENT_SECRET") {
                continue;
            }
            // `.env` wins over an inherited environment: `tauri dev` exports
            // the values it read at startup, which would otherwise pin stale
            // credentials across rebuilds.
            let value = value.trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                println!("cargo:rustc-env={key}={value}");
            }
        }
    }
}

fn build_swift() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "aarch64" => "arm64",
        arch => arch,
    }
    .to_string();
    let deployment = env::var("MACOSX_DEPLOYMENT_TARGET").unwrap_or("15.0".into());
    let target = format!("{arch}-apple-macosx{deployment}");
    let status = Command::new("xcrun")
        .args([
            "swiftc",
            "-emit-library",
            "-static",
            "-parse-as-library",
            "-O",
        ])
        .args(["-module-name", "HoshiSpeech", "-swift-version", "5"])
        .args(["-target", &target])
        .arg("-o")
        .arg(out.join("libhoshispeech.a"))
        .args(
            fs::read_dir("swift")
                .unwrap()
                .map(|entry| entry.unwrap().path()),
        )
        .status()
        .unwrap();
    assert!(status.success());

    let info = Command::new("xcrun")
        .args(["swiftc", "-print-target-info", "-target", &target])
        .output()
        .unwrap();
    let info: serde_json::Value = serde_json::from_slice(&info.stdout).unwrap();
    for path in info["paths"]["runtimeLibraryPaths"].as_array().unwrap() {
        println!("cargo:rustc-link-search=native={}", path.as_str().unwrap());
    }
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=hoshispeech");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    println!("cargo:rerun-if-changed=swift");
}
