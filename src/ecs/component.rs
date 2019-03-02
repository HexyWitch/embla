use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::mem;

pub struct ComponentStorage<T> {
    components: Vec<Option<T>>,
}

impl<T> ComponentStorage<T> {
    pub fn new() -> ComponentStorage<T> {
        ComponentStorage {
            components: Vec::new(),
        }
    }

    pub fn contains(&self, entity_id: usize) -> bool {
        self.components
            .get(entity_id)
            .map(|c| c.is_some())
            .unwrap_or(false)
    }

    pub fn insert(&mut self, entity_id: usize, c: T) {
        if entity_id >= self.components.len() {
            self.components.resize_with(entity_id, || None);
        }
        self.components.insert(entity_id, Some(c));
    }

    pub fn remove(&mut self, entity_id: usize) -> Option<T> {
        self.components.get_mut(entity_id).and_then(|v| v.take())
    }

    pub fn get(&self, entity_id: usize) -> Option<&T> {
        self.components.get(entity_id).and_then(|v| v.as_ref())
    }

    pub fn get_mut(&mut self, entity_id: usize) -> Option<&mut T> {
        self.components.get_mut(entity_id).and_then(|v| v.as_mut())
    }
}

pub trait GenericComponentStorage {
    fn next_entry(&self, start: usize) -> Option<usize>;
    fn remove(&mut self, id: usize);
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

impl<T: 'static> GenericComponentStorage for ComponentStorage<T> {
    fn next_entry(&self, start: usize) -> Option<usize> {
        self.components.get(start..).and_then(|s| {
            s.iter()
                .enumerate()
                .filter_map(|(e, c)| if c.is_some() { Some(start + e) } else { None })
                .next()
        })
    }
    fn remove(&mut self, id: usize) {
        self.remove(id);
    }
    fn as_any(&self) -> &Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

pub trait ComponentSet<'a> {
    type MutRefs;

    fn iter_mut_refs(
        storage: &'a HashMap<TypeId, RefCell<Box<GenericComponentStorage>>>,
    ) -> Box<Iterator<Item = Self::MutRefs> + 'a>;
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! implement_tuple_set {
    ($($x:ident:$xn:ident),*) => {
        impl<'a, $($x: 'static,)*> ComponentSet<'a> for ($($x,)*) {
            type MutRefs = ($(&'a mut $x,)*);

            fn iter_mut_refs(
                storage: &'a HashMap<TypeId, RefCell<Box<GenericComponentStorage>>>
            ) -> Box<Iterator<Item = Self::MutRefs> + 'a> {

                struct ComponentIterator<'a, $($x: 'a),*> {
                    index: usize,
                    $($xn: (RefMut<'a, ComponentStorage<$x>>)),*
                }
                impl<'a, $($x: 'static),*> Iterator for ComponentIterator<'a, $($x),*> {
                    type Item = ($(&'a mut $x,)*);

                    fn next(&mut self) -> Option<Self::Item> {
                        let component_count = 0 $(+ replace_expr!($x 1))*;
                        let mut entity = self.index;
                        let mut entity_count = 0;
                        let next_entity = loop {
                            $(
                                if let Some(e) = self.$xn.next_entry(entity) {
                                    if e != entity {
                                        entity_count = 0;
                                    }
                                    entity_count += 1;
                                    entity = e;
                                } else {
                                    break None;
                                }

                                if entity_count == component_count {
                                    self.index = entity + 1;
                                    break Some(entity);
                                }
                            )*
                            entity += 1;
                        };

                        // we can transmute the lifetime of the references to the lifetime of the iterator because:
                        // * this iterator holds a mutable reference to the component storage, guaranteeing there are no
                        //   other references to the storage or any component entry in the storage
                        // * the iterator can return only one mutable reference to each unique component entry
                        unsafe {
                            next_entity.map(|e| {
                                ($(
                                    mem::transmute::<&mut $x, &'a mut $x>(self.$xn.get_mut(e).unwrap()),
                                )*)
                            })
                        }
                    }
                }

                Box::new(
                    ComponentIterator {
                        index: 0,
                        $($xn: RefMut::map(
                            storage.get(&TypeId::of::<$x>()).expect("component $x not registered").borrow_mut(),
                            |s| s.as_any_mut().downcast_mut::<ComponentStorage<$x>>().unwrap()
                        )),*
                    }
                )
            }
        }
    }
}
implement_tuple_set! {A:a}
implement_tuple_set! {A:a, B:b}
implement_tuple_set! {A:a, B:b, C:c}
implement_tuple_set! {A:a, B:b, C:c, D:d}
implement_tuple_set! {A:a, B:b, C:c, D:d, E:e}
implement_tuple_set! {A:a, B:b, C:c, D:d, E:e, F:f}
implement_tuple_set! {A:a, B:b, C:c, D:d, E:e, F:f, G:g}
implement_tuple_set! {A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h}
implement_tuple_set! {A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, J:j}
implement_tuple_set! {A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, J:j, K:k}
implement_tuple_set! {A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, J:j, K:k, L:l}
