use std::env;

fn main() {
    let platforms = ["\"gtk4\"", "\"android\""];

    println!(
        "cargo::rustc-check-cfg=cfg(platform, values({platforms}))",
        platforms = platforms.join(", "),
    );

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let platform = match target_os.as_str() {
        "linux" => "gtk4",
        "android" => "android",
        _ => panic!("unsupported OS `{target_os}`"),
    };

    println!("cargo::rustc-cfg=platform=\"{platform}\"");
}
