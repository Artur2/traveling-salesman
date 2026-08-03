#![macro_use]

/// Call upgrade and panic if cant
#[macro_export]
macro_rules! upgrade_conditionally {
    ($value:expr) => {{
        let upgraded = $value.upgrade();
        match upgraded {
            Some(upgraded) => upgraded,
            None => panic!("Cant reach {:?}", $value),
        }
    }};
}

#[macro_export]
macro_rules! random_index {
    ($vertex:expr) => {{
        let length = max(0, $vertex.edges.len() - 1);
        let random_value = if length != 0 {
            thread_rng().gen_range(0, length)
        } else {
            0
        };

        random_value
    }};
}
