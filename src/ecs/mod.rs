use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt;

use failure::Error;

mod component;

use self::component::{ComponentSet, ComponentStorage, GenericComponentStorage};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entity {
    index: usize,
    generation: u32,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.index, self.generation)
    }
}

pub struct World {
    components: HashMap<TypeId, RefCell<Box<GenericComponentStorage>>>,
    entities: Vec<u32>,
    dead: Vec<usize>,
}

impl World {
    pub fn new() -> World {
        World {
            components: HashMap::new(),
            entities: Vec::new(),
            dead: Vec::new(),
        }
    }

    pub fn register_component<C: 'static>(&mut self) {
        self.components.insert(
            TypeId::of::<C>(),
            RefCell::new(Box::new(ComponentStorage::<C>::new())),
        );
    }

    pub fn entity<'a>(&'a mut self, e: Entity) -> Option<EntityEntry<'a>> {
        match self.entities.get(e.index) {
            Some(&gen) if gen == e.generation => Some(EntityEntry { world: self, e: e }),
            _ => None,
        }
    }

    pub fn add_entity<'a>(&'a mut self) -> EntityEntry<'a> {
        let entity_id = if let Some(index) = self.dead.pop() {
            let generation = self.entities[index] + 1;
            self.entities[index] = generation;
            Entity { index, generation }
        } else {
            let index = self.entities.len();
            self.entities.push(0);
            Entity {
                index,
                generation: 0,
            }
        };
        EntityEntry {
            world: self,
            e: entity_id,
        }
    }

    pub fn remove_entity(&mut self, e: Entity) {
        match self.entities.get(e.index) {
            Some(&gen) if gen == e.generation => {
                for (_, storage) in self.components.iter() {
                    storage.borrow_mut().remove(e.index);
                }
                self.dead.push(e.index);
            }
            _ => {}
        }
    }

    pub fn get_component<'b, C: 'static>(&'b self, e: Entity) -> Option<Ref<'b, C>> {
        match self.entities.get(e.index) {
            Some(&gen) if gen == e.generation => {
                let storage = self.get_storage::<C>();
                if storage.contains(e.index) {
                    Some(Ref::map(storage, |s| s.get(e.index).unwrap()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_storage<T: 'static>(&self) -> Ref<ComponentStorage<T>> {
        Ref::map(
            self.components
                .get(&TypeId::of::<T>())
                .expect("component not registered")
                .borrow(),
            |s| s.as_any().downcast_ref::<ComponentStorage<T>>().unwrap(),
        )
    }

    fn get_storage_mut<T: 'static>(&mut self) -> Result<RefMut<ComponentStorage<T>>, Error> {
        Ok(RefMut::map(
            self.components
                .get(&TypeId::of::<T>())
                .expect("component not registered")
                .borrow_mut(),
            |s| {
                s.as_any_mut()
                    .downcast_mut::<ComponentStorage<T>>()
                    .unwrap()
            },
        ))
    }

    fn insert_component<T: 'static>(&mut self, e: Entity, c: T) -> Result<(), Error> {
        if e.index >= self.entities.len() || e.generation != self.entities[e.index] {
            return Err(format_err!("Entity {} is dead", e));
        }
        self.get_storage_mut::<T>()?.insert(e.index, c);
        Ok(())
    }

    fn remove_component<T: 'static>(&mut self, e: Entity) -> Result<Option<T>, Error> {
        if e.index >= self.entities.len() || e.generation != self.entities[e.index] {
            return Err(format_err!("Entity {} is dead", e));
        }
        Ok(self.get_storage_mut::<T>()?.remove(e.index))
    }

    pub fn iter<'b, C: ComponentSet<'b>>(&'b self) -> Box<Iterator<Item = C::IterItem> + 'b> {
        C::iter(&self.components)
    }

    pub fn iter_entities<'a, C: ComponentSet<'a> + 'static>(
        &'a self,
    ) -> Box<Iterator<Item = (Entity, C::IterItem)> + 'a> {
        let entities = &self.entities;
        Box::new(C::indexed(&self.components).map(move |(e, cs)| {
            (
                Entity {
                    index: e,
                    generation: entities[e],
                },
                cs,
            )
        }))
    }
}

pub struct EntityEntry<'a> {
    world: &'a mut World,
    e: Entity,
}

impl<'a> EntityEntry<'a> {
    pub fn entity(&self) -> Entity {
        self.e
    }

    pub fn insert<C: 'static>(&mut self, c: C) -> Result<&mut Self, Error> {
        self.world.insert_component(self.e, c)?;
        Ok(self)
    }

    pub fn remove<C: 'static>(&mut self) -> Result<&mut Self, Error> {
        self.world.remove_component::<C>(self.e)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    struct Position(i32, i32);
    struct Velocity(i32, i32);

    #[test]
    fn ecs() {
        let mut world = World::new();

        world.register_component::<Position>();
        world.register_component::<Velocity>();

        let e1 = world
            .add_entity()
            .insert(Position(0, 0))
            .unwrap()
            .insert(Velocity(5, 5))
            .unwrap()
            .entity();
        let e2 = world
            .add_entity()
            .insert(Position(0, 0))
            .unwrap()
            .insert(Velocity(3, 4))
            .unwrap()
            .entity();
        let e3 = world.add_entity().insert(Position(0, 0)).unwrap().entity();

        let position_entities = &[e1, e2, e3];
        for (entity, (mut pos,)) in world.iter_entities::<(Position,)>() {
            pos.0 = 10;
            assert!(position_entities.contains(&entity));
        }

        let velocity_entities = &[e1, e2];
        for (entity, (mut pos, vel)) in world.iter_entities::<(Position, Velocity)>() {
            pos.0 += vel.0;
            pos.1 += vel.1;
            assert!(velocity_entities.contains(&entity));
        }

        assert_eq!(world.get_component::<Position>(e1).unwrap().0, 15);
        assert_eq!(world.get_component::<Position>(e2).unwrap().0, 13);
        assert_eq!(world.get_component::<Position>(e3).unwrap().0, 10);

        world.remove_component::<Position>(e1).unwrap();

        for (mut pos,) in world.iter::<(Position,)>() {
            pos.0 = 5;
        }

        assert_eq!(world.get_component::<Position>(e2).unwrap().0, 5);
        assert_eq!(world.get_component::<Position>(e3).unwrap().0, 5);

        for (mut pos, vel) in world.iter::<(Position, Velocity)>() {
            pos.0 += vel.0;
            pos.1 += vel.1;
        }

        assert_eq!(world.get_component::<Position>(e2).unwrap().0, 8);

        world.remove_entity(e3);
        let e4 = world.add_entity().entity();
        assert_eq!(e3.index, e4.index);
        assert_ne!(e3, e4);
    }
}
