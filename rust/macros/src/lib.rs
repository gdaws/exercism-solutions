#[macro_export]
macro_rules! hashmap {
    () => { crate::HashMap::new() };
    ($($key:literal => $value:expr),+ $(,)?) => {
        {
            let mut hm = crate::HashMap::new();
            $(hm.insert($key, $value);)*
            hm
        }
    };
}
