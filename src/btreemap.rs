/// Create a `BTreeMap` containing the arguments
/// 
/// `btreemap!` allows `BTreeMap`s to be constructed using minimal syntax:
/// 
/// - Create a `BTreeMap` using a list of `key => value` pairs:
/// 
/// ```
/// # #[macro_use] extern crate collection_macros;
/// # fn main() {
/// let m = btreemap!{
///     1 => "foo",
///     2 => "bar",
///     3 => "baz",
///     5 => "quux",
/// };
/// assert_eq!(m.get(&1), Some(&"foo"));
/// assert_eq!(m.get(&2), Some(&"bar"));
/// assert_eq!(m.get(&3), Some(&"baz"));
/// assert_eq!(m.get(&5), Some(&"quux"));
/// # }
/// ```
/// 
#[macro_export]
macro_rules! btreemap {
    ( $($x:expr => $y:expr),* ) => ({
        use std::collections::BTreeMap;
        let mut temp_map = BTreeMap::new();
        $(
            temp_map.insert($x, $y);
        )*
        temp_map
    });
    ( $($x:expr => $y:expr,)* ) => (
        btreemap!{$($x => $y),*}
    );
}

#[cfg(test)] mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn test_btreemap() {
        let map = btreemap! {
            1 => "hello",
            3 => "blah",
        };

        let mut should_be = BTreeMap::new();
        should_be.insert(1, "hello");
        should_be.insert(3, "blah");

        assert_eq!(map, should_be);

    }
}


