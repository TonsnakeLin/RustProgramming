#[cfg(feature = "myfeatures")]
pub fn print_my_feature() {
    println!("myfeatures is enabled");
}

#[cfg(feature = "myfeatures2")]
pub fn print_my_feature2() {
    println!("myfeatures2 is enabled");
}