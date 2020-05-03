//! My Crate  mod说明
//!
//! 'my_crate' is a collection of utilites to mak performing certain calcuations more convenient
//!

/// Add one to the number given  方法说明
/// #Example
///
/// ```
/// let file = 5;
///
/// assert_qe!(6,mylib::add_one(file));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}