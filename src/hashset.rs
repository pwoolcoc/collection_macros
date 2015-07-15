/// Create a `HashSet` containing the arguments
/// 
/// `hashset!` allows `HashSet`s to be constructed using minimal syntax:
/// 
/// ```
/// # #[cfg_attr(feature = "nightly", feature(collections))]
/// # #[macro_use] extern crate collection_macros;
/// use std::collections::HashSet;
/// # fn main() {
/// let m = hashset![
///     "a",
///     "b",
///     "c",
/// ];
/// assert_eq!(m.contains("a"));
/// assert_eq!(m.contains("b"));
/// assert_eq!(m.contains("c"));
/// # }
/// ```
/// 
#[macro_export]
macro_rules! hashset {
    ( $($x:expr),* ) => ({
        let mut temp_set = HashSet::new();
        $(
            temp_set.insert($x);
        )*
        temp_set
    });
    ( $($x:expr,)* ) => (
        hashset![$($x),*]
    );
}

#[cfg(test)] mod tests {
    use std::collections::{HashSet};

    #[test]
    fn test_hashset() {
        let set_ = hashset![
            "foo",
            "bar",
            "baz",
        ];

        let should_be = HashSet::new();
        should_be.insert("foo");
        should_be.insert("bar");
        should_be.insert("baz");

        assert_eq!(set_, should_be);
    }
}
