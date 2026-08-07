use std::rc::{Rc, Weak};

#[derive(Default)]
pub struct StringPool {
    string_references: Vec<Rc<String>>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            string_references: vec![],
        }
    }

    pub fn intern(&mut self, interning_string: String) -> Weak<String> {
        let interned_string = self
            .string_references
            .iter()
            .find(|rc| rc.as_str() == interning_string.as_str());

        if interned_string.is_some() {
            Rc::downgrade(&interned_string.unwrap())
        } else {
            let new_reference = Rc::new(interning_string);
            let weak_reference = Rc::downgrade(&new_reference);
            self.string_references.push(new_reference);
            weak_reference
        }
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn intern_string_should_return_weak_reference() {
        let mut pool = StringPool::new();
        let value = pool.intern("Test".to_owned());
        assert!(value.strong_count() == 1);
        assert!(value.weak_count() == 1);
    }
}
