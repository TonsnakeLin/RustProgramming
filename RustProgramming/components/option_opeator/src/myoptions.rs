pub fn my_option_func_suits() -> bool {
    let x = Some("linpin");
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);
    let y = Some("bieguanyin");
    assert_eq!(x.and(y), Some("bieguanyin"));
    true
}