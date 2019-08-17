fn main() {
    println!("Hello, world!");
}

// 通常コメント

/// RustDocコメント マークダウン記法に対応している

/// 与えられた数値に1加算して返す
/// 
/// # Examples
/// 
/// ```
/// let five = 5;
/// 
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
