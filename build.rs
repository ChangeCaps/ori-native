fn main() {
    println!("cargo::rustc-check-cfg=cfg(platform, values(\"gtk4\", \"android\"))");

    #[cfg(target_os = "linux")]
    println!("cargo::rustc-cfg=platform=\"gtk4\"");

    #[cfg(target_os = "android")]
    println!("cargo::rustc-cfg=platform=\"android\"")
}
