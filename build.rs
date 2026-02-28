use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        return;
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    compile_icon(&out_dir, "jif_to_jpg", "assets/icon_jpg.ico");
    compile_icon(&out_dir, "jif_to_png", "assets/icon_png.ico");
}

fn compile_icon(out_dir: &str, bin_name: &str, icon_path: &str) {
    let rc_path = Path::new(out_dir).join(format!("{}.rc", bin_name));
    let o_path = Path::new(out_dir).join(format!("{}.o", bin_name));

    let icon_abs = fs::canonicalize(icon_path).expect("icon file not found");
    let icon_str = icon_abs.to_str().unwrap().replace('\\', "/");

    fs::write(&rc_path, format!("1 ICON \"{}\"", icon_str)).unwrap();

    let status = Command::new("x86_64-w64-mingw32-windres")
        .arg(&rc_path)
        .arg("-o")
        .arg(&o_path)
        .status()
        .expect("failed to run windres — install gcc-mingw-w64-x86-64");

    assert!(status.success(), "windres failed for {}", bin_name);

    println!("cargo:rustc-link-arg-bin={}={}", bin_name, o_path.display());
}
