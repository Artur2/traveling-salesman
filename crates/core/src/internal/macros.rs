#![macro_use]

/// Call upgrade and panic if cant
 #[macro_export]
 macro_rules! upgrade_conditionally {
     ($value:expr) => {{
        let upgraded = $value.upgrade();
         match upgraded {
             Some(upgraded) => {
                 upgraded
             },
             None => panic!("Cant reach value"),
         }
     }};
 }