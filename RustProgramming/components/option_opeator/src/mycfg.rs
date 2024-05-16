fn can_enable_mycfg() -> bool {
    !cfg!(enable_cfgtest1) || !cfg!(enable_cfgtest2)
}

pub fn maybe_enable_mycfg() -> bool {
    if !can_enable_mycfg() {
        println!("it disables cfg");
    }

    #[cfg(enable_cfgtest1)]
    {
        println!("enable_cfgtest1");
    }

    #[cfg(enable_cfgtest2)]
    {
        println!("enable_cfgtest2");
    }

    true
}