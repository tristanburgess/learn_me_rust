//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing calculations of the universe
//! and the meaning of life and death themselves more convenient.

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
