
/// Create a `HashMap` containing the arguments
/// 
/// `hashmap!` allows `HashMap`s to be constructed using minimal syntax:
/// 
/// ```
/// # #[cfg_attr(feature = "nightly", feature(collections))]
/// # #[macro_use] extern crate collection_macros;
/// use std::collections::HashMap;
/// # fn main() {
/// let m = hashmap!{
///     "a" => "foo",
///     "b" => "bar",
///     "c" => "baz",
/// };
/// assert_eq!(m.get("a"), Some(&"foo"));
/// assert_eq!(m.get("b"), Some(&"bar"));
/// assert_eq!(m.get("c"), Some(&"baz"));
/// # }
/// ```
/// 
#[macro_export]
macro_rules! hashmap {
    ( $($x:expr => $y:expr),* ) => ({
        let mut temp_map = HashMap::new();
        $(
            temp_map.insert($x, $y);
        )*
        temp_map
    });
    ( $($x:expr => $y:expr,)* ) => (
        hashmap!{$($x => $y),*}
    );
}

#[cfg(test)] mod tests {
    use std::collections::{HashMap};

    #[test]
    fn test_hashmap() {
        let map = hashmap!{
            "one" => "two",
            "three" => "four",
        };

        let mut should_be = HashMap::new();
        should_be.insert("one", "two");
        should_be.insert("three", "four");

        assert_eq!(map, should_be);
    }
}


