use std::collections::HashMap;

/// Has two vecs: The first one is a list of must have components,
/// the second is a list of forbidden components.
pub type ReqTuple  = (Vec<&'static str>, Vec<&'static str>);

/// Matches each family to its list of required and forbidden components.
pub type FamilyMap = HashMap<&'static str, ReqTuple>;


/// Implemented by the `families!` macro, has the knowledge of all families requirements.
pub trait FamilyDataHolder {
    fn new() -> Self;
    fn all_families(&self) -> &FamilyMap;
}

/// Checks if the given `components` meet the requirements of a family.
pub fn matcher(reqs: &ReqTuple, components: &Vec<&str>) -> bool {
    for forb in reqs.1.iter() {
        if components.contains(&forb) { return false; }
    }
    for req in reqs.0.iter() {
        if !components.contains(&req) { return false; }
    }
    true
}

/// defines the families and which components an entity must (or musn't) have to belong to it.
///
/// # Example
///
/// ```
/// families!(
///     // An entity to belong to the "mob" family MUST have both `health` and `stats` components.
///     [mob: health, stats -],
///     // And for one to belong to the "tile" family, it MUST have `position` and `glyph` but MUST NOT
///     // have either `stats` NOR `health`
///     [tile: position, glyph - stats, health]
/// );
/// ```
#[macro_export]
macro_rules! families {
    ($data:ident:
     $([$family:ident: $($req:ident),* - $($forb:ident),*]),+) => {
        use std::collections::HashMap;
        use $crate::family::FamilyDataHolder;
        pub struct $data {
            families: $crate::family::FamilyMap,
        }

        impl $data {
            pub fn filled_new() -> $data {
                let mut map = HashMap::new();
                $(
                    let mut family_tuple = (Vec::new(), Vec::new());
                    $(
                        family_tuple.0.push(stringify!($req));
                    )*
                        
                    $(
                        family_tuple.1.push(stringify!($forb));
                    )*

                    map.insert(stringify!($family), family_tuple);
                 )+
                $data {
                    families: map,
                }
            }
        }

        impl FamilyDataHolder for $data {
            fn new() -> $data {
                $data::filled_new()
            }
            
            fn all_families(&self) -> &$crate::family::FamilyMap {
                &self.families
            }
        }
    }
}
