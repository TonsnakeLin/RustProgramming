use std::env;

pub fn option_env_macro() {
    if option_env!("LINPIN").map_or(false, |x| {
            println!("{}", x);
            x == "1"
        }) {
        println!("env LINPIN is set to 1");
    } else {
        println!("env LINPIN is not set to 1");
    }

    if let Ok(linqi_var) = env::var("LINQI") {
        println!("{}", linqi_var);
    } else {
        println!("env LINQI is not set");
    }

    if let Ok(var2) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        println!("DEP_OPENSSL_VERSION_NUMBER:{}", var2);
    } else {
        println!("env DEP_OPENSSL_VERSION_NUMBER is not set");
    }
}
