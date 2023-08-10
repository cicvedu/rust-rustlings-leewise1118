use std::env;
fn main() {
    println!("cargo:rustc-cfg=feature=\"pass\"");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp.to_string());
}
