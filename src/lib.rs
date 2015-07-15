#![doc(html_root_url = "https://pwoolcoc.github.io/collection_macros/")]
#![cfg_attr(feature = "nightly", feature(collections))]

//! Collection Macros
//! 
//! A collection of macros to construct collections with minimal syntax


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

/// Create a `BTreeMap` containing the arguments
/// 
/// `btreemap!` allows `BTreeMap`s to be constructed using minimal syntax:
/// 
/// - Create a `BTreeMap` using a list of `key => value` pairs:
/// 
/// ```
/// # #[macro_use] extern crate collection_macros;
/// use std::collections::BTreeMap;
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
#[cfg_attr(feature = "nightly", doc = r#"If you are using nightly, there is a second form to this macro:"#)]
#[cfg_attr(feature = "nightly", doc = r#""#)]
#[cfg_attr(feature = "nightly", doc = r#"- Create a `BTreeMap` using the `BTreeMap::with_b` constructor, and a list"#)]
#[cfg_attr(feature = "nightly", doc = r#"  of `key => value` pairs. This is currently only available on nightly"#)]
#[cfg_attr(feature = "nightly", doc = r#""#)]
#[cfg_attr(feature = "nightly", doc = r#"```"#)]
#[cfg_attr(feature = "nightly", doc = r#"# #![feature(collections)]"#)]
#[cfg_attr(feature = "nightly", doc = r#"# #[macro_use] extern crate collection_macros;"#)]
#[cfg_attr(feature = "nightly", doc = r#"use std::collections::BTreeMap;"#)]
#[cfg_attr(feature = "nightly", doc = r#"# fn main() {"#)]
#[cfg_attr(feature = "nightly", doc = r#"let m = btreemap!{"#)]
#[cfg_attr(feature = "nightly", doc = r#"    6;"#)]
#[cfg_attr(feature = "nightly", doc = r#"    1 => "foo","#)]
#[cfg_attr(feature = "nightly", doc = r#"    2 => "bar","#)]
#[cfg_attr(feature = "nightly", doc = r#"    3 => "baz","#)]
#[cfg_attr(feature = "nightly", doc = r#"    5 => "quux","#)]
#[cfg_attr(feature = "nightly", doc = r#"};"#)]
#[cfg_attr(feature = "nightly", doc = r#"assert_eq!(m.get(&1), Some(&"foo"));"#)]
#[cfg_attr(feature = "nightly", doc = r#"assert_eq!(m.get(&2), Some(&"bar"));"#)]
#[cfg_attr(feature = "nightly", doc = r#"assert_eq!(m.get(&3), Some(&"baz"));"#)]
#[cfg_attr(feature = "nightly", doc = r#"assert_eq!(m.get(&5), Some(&"quux"));"#)]
#[cfg_attr(feature = "nightly", doc = r#"# }"#)]
#[cfg_attr(feature = "nightly", doc = "```")]
#[cfg_attr(feature = "nightly", doc = "")]
#[macro_export]
macro_rules! btreemap {
    ( $b:expr; $($x:expr => $y:expr),* ) => ({
        let mut temp_map = BTreeMap::with_b($b);
        $(
            temp_map.insert($x, $y);
        )*
        temp_map
    });
    ( $($x:expr => $y:expr),* ) => ({
        let mut temp_map = BTreeMap::new();
        $(
            temp_map.insert($x, $y);
        )*
        temp_map
    });
    ( $b:expr; $($x:expr => $y:expr,)* ) => (
        btreemap!{$b; $($x => $y),*}
    );
    ( $($x:expr => $y:expr,)* ) => (
        btreemap!{$($x => $y),*}
    );
}

#[cfg(test)] mod tests {
    use std::collections::{BTreeMap, HashMap};

    #[cfg(feature = "nightly")]
    #[test]
    fn test_btreemap_with_b() {
        let bmap = btreemap! {
            6;
            1 => "hello",
            3 => "blah",
        };

        let mut should_be = BTreeMap::new();
        should_be.insert(1, "hello");
        should_be.insert(3, "blah");

        assert_eq!(bmap, should_be);
    }

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

