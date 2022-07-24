fn main() {
    pkg_config::Config::new().probe("x11").unwrap();
    pkg_config::Config::new().probe("libwacom").unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
