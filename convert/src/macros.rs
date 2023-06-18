#[macro_export]
/// Call a given macro for all given pairs of the input.
/// A modified version of the excellent https://stackoverflow.com/a/54552848
/// ```
/// use convert::for_all_pairs;
/// let mut my_pairs: Vec<(&'static str, &'static str)> = Vec::new();
/// macro_rules! add_pair_to_list {
///     ($m: ident, $a: ident, $b: ident) => {
///         $m.push(($a, $b));
///     }   
/// }
/// let a = "a";
/// let b = "b";
/// let c = "c";
/// for_all_pairs!(add_pair_to_list, my_pairs: a b c);
/// assert_eq!(my_pairs , vec![("a", "b"), ("a", "c"), ("b", "a"), ("b", "c"), ("c", "a"), ("c", "b")]);
/// ```
macro_rules! for_all_pairs {
    ($mac:ident, $a:ident: $($x:ident)*) => {
        // Duplicate the list
        for_all_pairs!(@inner $mac, $a: $($x)*; $($x)*);
    };

    // The end of iteration: we exhausted the list
    (@inner $mac:ident, $a:ident: ; $($x:ident)*) => {};

    // The head/tail recursion: pick the first element of the first list
    // and recursively do it for the tail.
    (@inner $mac:ident, $a:ident: $head:ident $($tail:ident)*; $($x:ident)*) => {
        $(
            $mac!($a, $head, $x);
        )*
        for_all_pairs!(@inner $mac, $a: $($tail)* ; $($x)*);
    };
}
