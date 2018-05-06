use std::any::TypeId;
use std::collections::{HashMap, HashSet};

pub struct ComponentMap {
    map: HashMap<TypeId, *mut ()>,
    removers: HashMap<TypeId, Box<Fn(&mut HashMap<TypeId, *mut ()>)>>,
}

impl ComponentMap {
    pub fn new() -> ComponentMap {
        ComponentMap {
            map: HashMap::new(),
            removers: HashMap::new(),
        }
    }

    pub fn get<C: 'static>(&self) -> Option<&C> {
        unsafe { self.map.get(&TypeId::of::<C>()).map(|c| &*(*c as *mut C)) }
    }

    pub fn insert<C: 'static>(&mut self, c: C) {
        let type_id = TypeId::of::<C>();
        self.map
            .insert(type_id.clone(), Box::into_raw(Box::new(c)) as *mut ());
        self.removers.insert(
            type_id,
            Box::new(move |map| unsafe {
                map.remove(&type_id).map(|p| Box::from_raw(p as *mut C));
            }),
        );
    }

    pub fn remove<C: 'static>(&mut self) -> Option<C> {
        let type_id = TypeId::of::<C>();
        self.removers.remove(&type_id);
        unsafe {
            self.map
                .remove(&type_id)
                .map(|c| *Box::from_raw(c as *mut C))
        }
    }

    pub fn type_set(&self) -> HashSet<TypeId> {
        self.map.keys().cloned().collect()
    }
}

impl Drop for ComponentMap {
    fn drop(&mut self) {
        for (_, remover) in self.removers.drain() {
            (*remover)(&mut self.map);
        }
    }
}

pub trait ComponentSet<'a> {
    type MutRefs;

    fn type_set() -> HashSet<TypeId>;

    // unsafe to allow unchecked interior mutability
    // only one mut_refs borrow at a time is allowed
    unsafe fn mut_refs(component_map: &'a ComponentMap) -> Option<Self::MutRefs>;
}

macro_rules! implement_tuple_set {
    ($( $x:ident ),* ) => {
        impl<'a, $($x: 'static, )*> ComponentSet<'a> for ($($x),*,) {
            type MutRefs = ($(&'a mut $x),*,);

            fn type_set() -> HashSet<TypeId> {
                let mut set = HashSet::new();
                $(
                    set.insert(TypeId::of::<$x>());
                )*
                set
            }
            unsafe fn mut_refs(component_map: &'a ComponentMap) -> Option<Self::MutRefs> {
                let mut type_set = Self::type_set();
                $(
                    if !type_set.remove(&TypeId::of::<$x>()) {
                        panic!("encountered duplicate component in component set");
                    }
                )*

                Some((
                    $(
                        &mut *(*component_map.map.get(&TypeId::of::<$x>())? as *mut $x)
                    ),*
                ,))
            }
        }
    }
}
implement_tuple_set!{A}
implement_tuple_set!{A, B}
implement_tuple_set!{A, B, C}
implement_tuple_set!{A, B, C, D}
implement_tuple_set!{A, B, C, D, E}
implement_tuple_set!{A, B, C, D, E, F}
implement_tuple_set!{A, B, C, D, E, F, G}
implement_tuple_set!{A, B, C, D, E, F, G, H}
implement_tuple_set!{A, B, C, D, E, F, G, H, J}
implement_tuple_set!{A, B, C, D, E, F, G, H, J, K}
implement_tuple_set!{A, B, C, D, E, F, G, H, J, K, L}
