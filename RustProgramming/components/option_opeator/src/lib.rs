mod macros;
mod mycfg;
mod myfeature;
mod myoptions;
pub use self::macros::option_env_macro;
pub use self::mycfg::maybe_enable_mycfg;
pub use self::myoptions::my_option_func_suits;
#[cfg(feature = "myfeatures")]
pub use self::myfeature::print_my_feature;
#[cfg(feature = "myfeatures2")]
pub use self::myfeature::print_my_feature2;