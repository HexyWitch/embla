use std::any::TypeId;
use std::collections::HashSet;

use super::component::{ComponentMap, ComponentSet};

pub struct Entity {
    id: usize,
    components: ComponentMap,
}

impl Entity {
    pub fn new(id: usize) -> Entity {
        Entity {
            id,
            components: ComponentMap::new(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn get<'a, T: 'static>(&'a self) -> Option<&T> {
        self.components.get::<T>()
    }

    pub fn insert<T: 'static>(&mut self, c: T) -> &mut Self {
        self.components.insert::<T>(c);
        self
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.components.remove::<T>()
    }

    // unsafe to allow unchecked interior mutability
    // only one components borrow at a time is allowed
    pub unsafe fn components<'a, T: ComponentSet<'a>>(&'a self) -> Option<T::MutRefs> {
        T::mut_refs(&self.components)
    }

    pub fn component_set<'a>(&self) -> HashSet<TypeId> {
        self.components.type_set()
    }
}
