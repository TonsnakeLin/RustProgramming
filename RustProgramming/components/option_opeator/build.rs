use std::{env,time};

fn main() {
    println!("option_opertator build.rs begin");
    if option_env!("CFGTEST1").map_or(false, |x| {x == "1"}) {
        println!("cargo:rustc-cfg=enable_cfgtest1");
    }

    if option_env!("CFGTEST2").map_or(false, |x| {x == "1"}) {
        println!("cargo:rustc-cfg=enable_cfgtest2");
    }

    if let Ok(version) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        println!("DEP_OPENSSL_VERSION_NUMBER:{}", version);
    } else {
        println!("env DEP_OPENSSL_VERSION_NUMBER is not set");
    }

    print!("cargo:rustc-env=RustProgramming_Build_Time={:?}", 
    time::SystemTime::now());
}
