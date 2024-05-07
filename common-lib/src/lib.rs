pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod stdsync;
pub mod strings;

pub use self::{
    strings::print_hello,
    stdsync::test_arc_strong_count,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
