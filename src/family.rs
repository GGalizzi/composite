use std::collections::HashMap;

pub type ReqTuple  = (Vec<&'static str>, Vec<&'static str>);
pub type FamilyMap = HashMap<&'static str, ReqTuple>;

pub trait FamilyDataHolder {
    fn new() -> Self;
    fn all_families(&self) -> &FamilyMap;
}

pub fn matcher(reqs: &ReqTuple, components: &Vec<&str>) -> bool {
    for forb in reqs.1.iter() {
        if components.contains(&forb) { return false; }
    }
    for req in reqs.0.iter() {
        if !components.contains(&req) { return false; }
    }
    true
}

#[macro_export]
macro_rules! families {
    ($([$family:ident: {$($req:ident),*} - {$($forb:ident),*}]),+) => {
        use std::collections::HashMap;
        use $crate::family::FamilyDataHolder;
        struct FamilyData {
            families: FamilyMap,
        }

        impl FamilyData {
            pub fn filled_new() -> FamilyData {
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
                FamilyData {
                    families: map,
                }
            }
        }

        impl FamilyDataHolder for FamilyData {
            fn new() -> FamilyData {
                FamilyData::filled_new()
            }
            
            fn all_families(&self) -> &FamilyMap {
                &self.families
            }
        }
    }
}
