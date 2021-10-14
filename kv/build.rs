use std::process::Command;

fn main() {
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);
    if !build_enabled {
        println!("=== Skip compiling protos ===");
        return;
    }

    prost_build::Config::new()
        .bytes(&["."])
        .type_attribute(".", "#[derive(PartialOrd)]")
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
    Command::new("cargo")
        .args(&["fmt", "--", "src/pb/*.rs"])
        .status()
        .expect("cargo fmt failed");
}
