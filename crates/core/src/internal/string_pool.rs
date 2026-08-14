use std::rc::{Rc, Weak};

#[derive(Default)]
pub struct StringPool {
    string_references: Vec<Rc<str>>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            string_references: vec![],
        }
    }

    pub fn intern(&mut self, interning_string: String) -> Weak<str> {
        let interned_string = self
            .string_references
            .iter()
            .find(|rc| rc.as_ref().eq(&interning_string));

        if interned_string.is_some() {
            Rc::downgrade(&interned_string.unwrap())
        } else {
            let new_reference = Rc::from(interning_string.as_str());
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
        assert_eq!(value.strong_count(), 1);
        assert_eq!(value.weak_count(), 1);
    }
}
