fn main() {
    tauri_build::build();

    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mupdf_dir = if cfg!(target_os = "macos") {
        let ci = manifest_dir.join("libs").join("macos");
        if ci.join("libmupdf.a").exists() {
            ci
        } else {
            manifest_dir.join("mupdf").join("build").join("release-tofu")
        }
    } else if cfg!(target_os = "windows") {
        let ci = manifest_dir.join("libs").join("windows");
        if ci.join("mupdf.lib").exists() {
            ci
        } else {
            manifest_dir.join("mupdf").join("build").join("release-tofu")
        }
    } else {
        manifest_dir.join("mupdf").join("build").join("release-tofu")
    };

    println!("cargo:rustc-link-search=native={}", mupdf_dir.display());
    println!("cargo:rustc-link-lib=static=mupdf");
    println!("cargo:rustc-link-lib=static=mupdf-third");

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
    }
}