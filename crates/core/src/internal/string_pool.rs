use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Default)]
pub struct StringPool {
    string_references: HashMap<String, Rc<str>>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            string_references: HashMap::new(),
        }
    }

    pub fn intern(&mut self, interning_string: String) -> Weak<str> {
        let interned_string = self
            .string_references
            .entry(interning_string)
            .or_insert_with_key(|f| Rc::from(f.as_str()));

        Rc::downgrade(interned_string)
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn intern_string_should_return_weak_reference() {
        let mut pool = StringPool::new();
        let value = pool.intern("Test".to_owned());
        assert_eq!(value.strong_count(), 1);
        assert_eq!(value.weak_count(), 1);
    }
}
