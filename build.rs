use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Define the output directory for the compiled resources
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Specify the path to your resource files
    let icon_res_file = "res/icon.rc";
    let icon_res_out = out_dir.join("icon.o");
    let version_res_file = "res/version.rc";
    let version_res_out = out_dir.join("version.o");

    // Compile the resource files
    Command::new("x86_64-w64-mingw32-windres")
        .args(&[icon_res_file, "-o"])
        .arg(&icon_res_out)
        .status()
        .unwrap();
    Command::new("x86_64-w64-mingw32-windres")
        .args(&[version_res_file, "-o"])
        .arg(&version_res_out)
        .status()
        .unwrap();

    // Link resource objects
    println!("cargo:rustc-link-arg={}", icon_res_out.display());
    println!("cargo:rustc-link-arg={}", version_res_out.display());
}
