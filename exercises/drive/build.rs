
use std::env;

fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
   env::set_var("TEST_FOO", timestamp.to_string());

    // env::set_var("FEATURE_4", "true");
    // println!("cargo:rustc-env=FEATURE_4=true");
    env::set_var("FEATURE_4", "true");
    println!("cargo:rustc-cfg=FEATURE_4");
 



}