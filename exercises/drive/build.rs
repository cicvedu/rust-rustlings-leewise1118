use std::env;
fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let time = timestamp + 5;

    println!("cargo:rustc-cfg=feature=\"pass\"");
    env::set_var("TEST_FOO", time.to_string());
}
