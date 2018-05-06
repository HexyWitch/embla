use std::any::Any;
use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::marker;
use std::rc::Rc;

use failure::Error;

mod component;
mod entity;

use self::component::ComponentSet;
use self::entity::Entity;

pub type ResourceEntry = Rc<RefCell<Box<Any>>>;

pub struct World {
    systems: HashMap<TypeId, (HashSet<TypeId>, fn(&mut World) -> Result<(), Error>)>,
    system_index: HashMap<TypeId, BTreeSet<usize>>,
    entities: Vec<Option<Entity>>,
    dead: Vec<usize>,
    resources: HashMap<TypeId, ResourceEntry>,
}

impl World {
    pub fn new() -> World {
        World {
            systems: HashMap::new(),
            system_index: HashMap::new(),
            entities: Vec::new(),
            dead: Vec::new(),
            resources: HashMap::new(),
        }
    }

    pub fn register_system<T: System + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();

        let mut entity_index = BTreeSet::new();
        let system_components = <T as System>::Components::type_set();
        for e in self.entities.iter_mut().filter_map(|e| e.as_mut()) {
            if e.component_set().is_superset(&system_components) {
                entity_index.insert(e.id());
            }
        }
        self.system_index.insert(type_id, entity_index);

        fn runner<'a, T: System + 'static>(world: &'a mut World) -> Result<(), Error> {
            let world_cb = WorldCallback {
                system: marker::PhantomData::<T>,
                component_lock: RefCell::new(()),
                world,
            };

            T::run(world_cb)
        }
        self.systems.insert(
            TypeId::of::<T>(),
            (<T as System>::Components::type_set(), runner::<T>),
        );
    }

    pub fn run_system<T: System + 'static>(&mut self) -> Result<(), Error> {
        let type_id = TypeId::of::<T>();

        let runner = self.systems.get(&type_id).expect("system not registered").1;
        let res = (runner)(self);

        res
    }

    pub fn entity<'a>(&'a mut self, id: usize) -> Option<EntityEntry<'a>> {
        if self.entities.get(id).and_then(|e| e.as_ref()).is_some() {
            Some(EntityEntry {
                world: self,
                id: id,
            })
        } else {
            None
        }
    }

    pub fn add_entity<'a>(&'a mut self) -> EntityEntry<'a> {
        let id = if let Some(id) = self.dead.pop() {
            self.entities[id] = Some(Entity::new(id));
            id
        } else {
            let id = self.entities.len();
            self.entities.push(Some(Entity::new(id)));
            id
        };
        EntityEntry {
            world: self,
            id: id,
        }
    }

    pub fn remove_entity(&mut self, id: usize) {
        if id < self.entities.len() {
            self.entities[id] = None;
        }
        self.dead.push(id);
    }

    pub fn register_resource<R: 'static>(&mut self, r: R) {
        self.resources
            .insert(TypeId::of::<R>(), Rc::new(RefCell::new(Box::new(r))));
    }

    pub fn get_resource<R: 'static>(&self) -> Result<RefMut<R>, Error> {
        Ok(RefMut::map(
            self.resources
                .get(&TypeId::of::<R>())
                .ok_or_else(|| format_err!("resource not registered"))?
                .borrow_mut(),
            |r| r.downcast_mut::<R>().unwrap(),
        ))
    }

    fn reindex_entity(&mut self, id: usize) {
        let entity_components = self.entities[id].as_ref().unwrap().component_set();
        for (type_id, (system_components, _)) in self.systems.iter() {
            let index = self.system_index.get_mut(&type_id).unwrap();
            if entity_components.is_superset(&system_components) {
                index.insert(id);
            } else {
                index.remove(&id);
            }
        }
    }

    fn insert_component<T: 'static>(&mut self, id: usize, c: T) {
        self.entities
            .get_mut(id)
            .expect("entity does not exist")
            .as_mut()
            .expect("entity is not alive")
            .insert(c);

        self.reindex_entity(id);
    }

    fn remove_component<T: 'static>(&mut self, id: usize) {
        self.entities
            .get_mut(id)
            .expect("entity does not exist")
            .as_mut()
            .expect("entity is not alive")
            .remove::<T>();

        self.reindex_entity(id);
    }
}

pub trait System: Sized {
    type Components: for<'a> ComponentSet<'a>;

    fn run<'a>(world: WorldCallback<'a, Self>) -> Result<(), Error>;
}

pub struct WorldCallback<'a, T: System + 'static> {
    system: marker::PhantomData<T>,
    component_lock: RefCell<()>,
    world: &'a mut World,
}

impl<'a, T: System + 'static> WorldCallback<'a, T> {
    pub fn components<'b>(
        &'b self,
    ) -> Result<impl Iterator<Item = <T::Components as ComponentSet>::MutRefs> + 'b, Error> {
        let index = self.world.system_index.get(&TypeId::of::<T>()).unwrap();
        let lock = self.component_lock
            .try_borrow_mut()
            .map_err(|_| format_err!("components must not be called recursively"))?;
        Ok(self.world
            .entities
            .iter()
            .enumerate()
            .filter(move |(i, _)| index.contains(i))
            .map(move |(_, e)| {
                let _lock = &lock;

                // Entity::components is unsafe because of unchecked internal mutability
                // Calling it here is safe because:
                // * WorldCallback holds a &mut World
                // * No other immutable methods in WorldCallback access entity components
                // * WorldCallback::components cannot be called recursively
                unsafe {
                    e.as_ref()
                        .expect("indexed entity must be alive")
                        .components::<<T as System>::Components>()
                        .expect("indexed entity must contain all indexed components")
                }
            }))
    }

    pub fn add_entity(&'a mut self) -> EntityEntry<'a> {
        self.world.add_entity()
    }

    pub fn remove_entity(&mut self, id: usize) {
        self.world.remove_entity(id);
    }

    pub fn get_resource<R: 'static>(&self) -> Result<RefMut<R>, Error> {
        self.world.get_resource::<R>()
    }
}

pub struct EntityEntry<'a> {
    world: &'a mut World,
    id: usize,
}

impl<'a> EntityEntry<'a> {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn insert<C: 'static>(&mut self, c: C) -> &mut Self {
        self.world.insert_component(self.id, c);
        self
    }

    pub fn remove<C: 'static>(&mut self) -> &mut Self {
        self.world.remove_component::<C>(self.id);
        self
    }

    pub fn get<'b, C: 'static>(&'b self) -> Option<&'b C> {
        self.world
            .entities
            .get(self.id)
            .unwrap()
            .as_ref()
            .unwrap()
            .get::<C>()
    }
}

#[cfg(test)]
mod tests {
    use super::{System, World, WorldCallback};
    use failure::Error;

    struct Position(i32, i32);
    struct Velocity(i32, i32);

    struct PositioningSystem;
    impl System for PositioningSystem {
        type Components = (Position,);

        fn run(world: WorldCallback<Self>) -> Result<(), Error> {
            for (mut pos,) in world.components()? {
                pos.0 = 10;
            }

            Ok(())
        }
    }

    struct MovementSystem;
    impl System for MovementSystem {
        type Components = (Position, Velocity);

        fn run(world: WorldCallback<Self>) -> Result<(), Error> {
            for (mut pos, vel) in world.components()? {
                pos.0 += vel.0;
                pos.1 += vel.1;
            }

            Ok(())
        }
    }

    #[test]
    fn ecs() {
        let mut world = World::new();
        world.register_system::<PositioningSystem>();
        world.register_system::<MovementSystem>();

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

        world.run_system::<PositioningSystem>().unwrap();
        world.run_system::<MovementSystem>().unwrap();

        assert_eq!(world.entity(e1).unwrap().get::<Position>().unwrap().0, 15);
        assert_eq!(world.entity(e2).unwrap().get::<Position>().unwrap().0, 13);
        assert_eq!(world.entity(e3).unwrap().get::<Position>().unwrap().0, 10);
    }
}
