use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

type ComponentMap = HashMap<TypeId, RefCell<Box<Any>>>;

pub trait ComponentSetMutRefs<'a> {
    type MutRefs;
}
pub trait ComponentSet {
    type RefKind: for<'a> ComponentSetMutRefs<'a>;

    fn mut_refs<'a>(
        component_map: &'a mut ComponentMap,
    ) -> Option<<Self::RefKind as ComponentSetMutRefs<'a>>::MutRefs>;
}

macro_rules! implement_tuple_set {
    ($( $x:ident ),* ) => {
        impl<'a, $($x: 'static, )*> ComponentSetMutRefs<'a> for ($($x),*,) {
            type MutRefs = ($(RefMut<'a, $x>),*,);
        }
        impl<$($x: 'static, )*> ComponentSet for ($($x),*,) {
            type RefKind = Self;

            fn mut_refs<'a>(component_map: &'a mut ComponentMap) -> Option<<Self as ComponentSetMutRefs<'a>>::MutRefs> {
                Some((
                    $(
                        component_map
                            .get(&TypeId::of::<$x>())
                            .map(|c| RefMut::map(c.borrow_mut(), |c| c.downcast_mut::<$x>().unwrap()))?
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

pub struct Entity {
    id: usize,
    components: ComponentMap,
}

#[derive(Clone, Copy)]
pub struct EntityId(pub usize);

impl Entity {
    pub fn new(id: usize) -> Entity {
        let mut e = Entity {
            id,
            components: ComponentMap::new(),
        };
        e.insert(EntityId(id));
        e
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn get<'a, T: 'static>(&'a mut self) -> Option<Ref<'a, T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|c| Ref::map(c.borrow(), |c| c.downcast_ref::<T>().unwrap()))
    }

    pub fn insert<T: 'static>(&mut self, c: T) -> &mut Self {
        self.components
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(c)));
        self
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.components
            .remove(&TypeId::of::<T>())
            .map(|c| *c.into_inner().downcast::<T>().unwrap())
    }

    pub fn components<'a, T: ComponentSet>(
        &'a mut self,
    ) -> Option<<T::RefKind as ComponentSetMutRefs<'a>>::MutRefs> {
        T::mut_refs(&mut self.components)
    }
}

pub struct World {
    entities: Vec<Option<Entity>>,
    dead: Vec<usize>,
}

impl World {
    pub fn new() -> World {
        World {
            entities: Vec::new(),
            dead: Vec::new(),
        }
    }

    pub fn entity(&mut self, id: usize) -> Option<&mut Entity> {
        self.entities.get_mut(id).and_then(|e| e.as_mut())
    }

    pub fn add_entity(&mut self) -> &mut Entity {
        let id = if let Some(id) = self.dead.pop() {
            self.entities[id] = Some(Entity::new(id));
            id
        } else {
            let id = self.entities.len();
            self.entities.push(Some(Entity::new(id)));
            id
        };
        self.entities[id].as_mut().unwrap()
    }

    pub fn remove_entity(&mut self, id: usize) {
        if id < self.entities.len() {
            self.entities[id] = None;
        }
        self.dead.push(id);
    }

    pub fn with_components<'a, T: ComponentSet>(
        &'a mut self,
    ) -> impl Iterator<Item = <T::RefKind as ComponentSetMutRefs<'a>>::MutRefs> + 'a {
        self.entities.iter_mut().filter_map(|e| {
            if let Some(ref mut e) = e {
                e.components::<T>()
            } else {
                None
            }
        })
    }
}

#[test]
fn ecs() {
    struct Position(i32, i32);
    struct Velocity(i32, i32);

    let mut world = World::new();

    let e1 = world
        .add_entity()
        .insert::<Position>(Position(0, 0))
        .insert::<Velocity>(Velocity(5, 5))
        .id();
    let e2 = world
        .add_entity()
        .insert::<Position>(Position(0, 0))
        .insert::<Velocity>(Velocity(3, 4))
        .id();
    let e3 = world.add_entity().insert::<Position>(Position(0, 0)).id();

    for (mut pos,) in world.with_components::<(Position,)>() {
        pos.0 = 10;
    }

    for (mut pos, vel) in world.with_components::<(Position, Velocity)>() {
        pos.0 += vel.0;
        pos.1 += vel.1;
    }

    assert_eq!(world.entity(e1).unwrap().get::<Position>().unwrap().0, 15);
    assert_eq!(world.entity(e2).unwrap().get::<Position>().unwrap().0, 13);
    assert_eq!(world.entity(e3).unwrap().get::<Position>().unwrap().0, 10);
}
