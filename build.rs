fn main() {
    println!("cargo::rerun-if-changed=templates-src/**");

    std::process::Command::new("make")
        .arg("templates")
        .output()
        .unwrap();
}
