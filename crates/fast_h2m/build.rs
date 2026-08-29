use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");

    if asm_tl_is_enabled_and_supported() {
        generate_asm_backend();
    }

    let Ok(output) = Command::new("rustc").arg("-Vv").output() else {
        return;
    };
    if !output.status.success() {
        return;
    }

    let version = String::from_utf8_lossy(&output.stdout);
    if version.lines().any(|line| {
        line.strip_prefix("release: ")
            .is_some_and(|release| release.contains("nightly"))
    }) {
        println!("cargo:rustc-cfg=nightly");
    }
}

fn asm_tl_is_enabled_and_supported() -> bool {
    if std::env::var_os("CARGO_FEATURE_ASM_TL").is_none() {
        return false;
    }

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    (os == "linux" && matches!(arch.as_str(), "x86_64" | "aarch64" | "riscv64"))
        || (arch == "x86_64" && os == "windows" && env == "msvc")
}

fn generate_asm_backend() {
    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let source_dir = manifest_dir.join("src");
    let output_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("asm_backend");

    // Both parser crates expose the same API, but their DOM types are nominally
    // distinct. Compile the existing converter modules a second time with
    // paths rewritten to `asm_tl`; this keeps one implementation of the
    // conversion logic while allowing both backends to coexist in one binary.
    copy_transformed_tree(&source_dir.join("converter"), &output_dir.join("converter"));
    copy_transformed_file(
        &source_dir.join("prelude.rs"),
        &output_dir.join("prelude.rs"),
    );
}

fn copy_transformed_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create generated asm backend directory");
    println!("cargo:rerun-if-changed={}", source.display());

    for entry in fs::read_dir(source).expect("read backend source directory") {
        let entry = entry.expect("read backend source entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_transformed_tree(&source_path, &destination_path);
        } else if source_path
            .extension()
            .is_some_and(|extension| extension == "rs")
        {
            copy_transformed_file(&source_path, &destination_path);
        }
    }
}

fn copy_transformed_file(source: &Path, destination: &Path) {
    println!("cargo:rerun-if-changed={}", source.display());
    let contents = fs::read_to_string(source).expect("read backend source file");
    let transformed = contents
        .replace("//!", "//")
        .replace("#![allow(unused_imports)]", "")
        .replace("crate::converter", "crate::asm_backend::converter")
        .replace("crate::tl_types", "crate::asm_backend::tl_types")
        .replace("crate::prelude", "crate::asm_backend::prelude")
        .replace("use tl;", "use asm_tl;")
        .replace("tl::", "asm_tl::");
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).expect("create generated backend parent directory");
    }
    fs::write(destination, transformed).expect("write generated asm backend source");
}
