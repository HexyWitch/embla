use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::collections::{BTreeSet, HashMap};
use std::mem;

pub struct ComponentStorage<T> {
    index: BTreeSet<usize>,
    components: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> ComponentStorage<T> {
    pub fn new() -> ComponentStorage<T> {
        ComponentStorage {
            index: BTreeSet::new(),
            components: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn index(&self) -> &BTreeSet<usize> {
        &self.index
    }

    pub fn insert(&mut self, entity_id: usize, c: T) -> usize {
        self.index.insert(entity_id);
        if let Some(k) = self.free.pop() {
            self.components[k] = Some(c);
            k
        } else {
            self.components.push(Some(c));
            self.components.len() - 1
        }
    }

    pub fn remove(&mut self, k: usize) -> Option<T> {
        self.index.remove(&k);
        if let Some(v) = self.components.get_mut(k).and_then(|v| v.take()) {
            self.free.push(k);
            Some(v)
        } else {
            None
        }
    }

    pub fn get(&self, k: usize) -> Option<&T> {
        self.components.get(k).and_then(|v| v.as_ref())
    }

    pub fn get_mut(&mut self, k: usize) -> Option<&mut T> {
        self.components.get_mut(k).and_then(|v| v.as_mut())
    }
}

pub trait GenericComponentStorage {
    fn remove(&mut self, id: usize);
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

impl<T: 'static> GenericComponentStorage for ComponentStorage<T> {
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
        entities: &'a Vec<Option<HashMap<TypeId, usize>>>,
        storage: &'a HashMap<TypeId, RefCell<Box<GenericComponentStorage>>>,
    ) -> Box<Iterator<Item = Self::MutRefs> + 'a>;
}

macro_rules! implement_tuple_set {
    ($($x:ident:$xn:ident),*) => {
        impl<'a, $($x: 'static,)*> ComponentSet<'a> for ($($x,)*) {
            type MutRefs = ($(&'a mut $x,)*);

            fn iter_mut_refs(
                entities: &'a Vec<Option<HashMap<TypeId, usize>>>,
                storage: &'a HashMap<TypeId, RefCell<Box<GenericComponentStorage>>>
            ) -> Box<Iterator<Item = Self::MutRefs> + 'a> {
                $(
                    let $xn = RefMut::map(
                        storage.get(&TypeId::of::<$x>()).expect("component not registered").borrow_mut(),
                        |s| s.as_any_mut().downcast_mut::<ComponentStorage<$x>>().unwrap()
                    );
                )*
                let index = vec![$($xn.index()),*]
                    .into_iter()
                    .fold(None, |l, r| if let Some(l) = l { Some(&l & r) } else { Some(r.clone()) })
                    .unwrap();

                struct ComponentIterator<'a, I: Iterator<Item = usize>, $($x: 'a),*> {
                    index: I,
                    entities: &'a Vec<Option<HashMap<TypeId, usize>>>,
                    $($xn: (RefMut<'a, ComponentStorage<$x>>)),*
                }
                impl<'a, I: Iterator<Item = usize>, $($x: 'static),*> Iterator for ComponentIterator<'a, I, $($x),*> {
                    type Item = ($(&'a mut $x,)*);

                    fn next(&mut self) -> Option<Self::Item> {
                        if let Some(k) = self.index.next() {
                            let e = self.entities.get(k).expect("entity not found").as_ref().unwrap();
                            $(let $xn = e.get(&TypeId::of::<$x>()).expect("entity index mismatch");)*

                            // we can transmute the lifetime of the references to 'a
                            // because iterating over a set of unique indexes guarantees
                            // that each returned mutable references is unique for the
                            // entire lifetime of the iterator
                            unsafe {
                                Some((
                                    $(mem::transmute(self.$xn.get_mut(*$xn)?),)*
                                ))
                            }
                        } else {
                            None
                        }
                    }
                }

                Box::new(
                    ComponentIterator {
                        index: index.into_iter(),
                        entities,
                        $($xn),*
                    }
                )
            }
        }
    }
}
implement_tuple_set!{A:a}
implement_tuple_set!{A:a, B:b}
implement_tuple_set!{A:a, B:b, C:c}
implement_tuple_set!{A:a, B:b, C:c, D:d}
implement_tuple_set!{A:a, B:b, C:c, D:d, E:e}
implement_tuple_set!{A:a, B:b, C:c, D:d, E:e, F:f}
implement_tuple_set!{A:a, B:b, C:c, D:d, E:e, F:f, G:g}
implement_tuple_set!{A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h}
implement_tuple_set!{A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, J:j}
implement_tuple_set!{A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, J:j, K:k}
implement_tuple_set!{A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, J:j, K:k, L:l}
