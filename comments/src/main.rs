fn main() {
// Line comments are anything after '//'
    {
        let x = 5; // This is also a line comment

    }
}

/// Doc comment, supports Markdown notation

/// Adds one to the number give
///
/// # Examples
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #   x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}