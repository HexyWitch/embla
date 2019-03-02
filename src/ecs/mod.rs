use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

use failure::Error;

mod component;

use self::component::{ComponentSet, ComponentStorage, GenericComponentStorage};

pub struct World {
    components: HashMap<TypeId, RefCell<Box<GenericComponentStorage>>>,
    entities: Vec<bool>,
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

    pub fn entity<'a>(&'a mut self, id: usize) -> Option<EntityEntry<'a>> {
        if self.entities.get(id).cloned().unwrap_or(false) {
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
            self.entities[id] = true;
            id
        } else {
            let id = self.entities.len();
            self.entities.push(true);
            id
        };
        EntityEntry {
            world: self,
            id: id,
        }
    }

    pub fn remove_entity(&mut self, id: usize) {
        for (_, storage) in self.components.iter() {
            storage.borrow_mut().remove(id);
        }
        self.entities[id] = false;
        self.dead.push(id);
    }

    pub fn get_component<'b, C: 'static>(&'b self, id: usize) -> Option<Ref<'b, C>> {
        let storage = self.get_storage::<C>();
        if storage.contains(id) {
            Some(Ref::map(storage, |s| s.get(id).unwrap()))
        } else {
            None
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

    fn insert_component<T: 'static>(&mut self, id: usize, c: T) -> Result<(), Error> {
        self.get_storage_mut::<T>()?.insert(id, c);
        Ok(())
    }

    fn remove_component<T: 'static>(&mut self, id: usize) -> Result<Option<T>, Error> {
        Ok(self.get_storage_mut::<T>()?.remove(id))
    }

    pub fn with_components<'b, C: ComponentSet<'b>>(
        &'b self,
    ) -> Box<Iterator<Item = C::MutRefs> + 'b> {
        C::iter_mut_refs(&self.components)
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

    pub fn insert<C: 'static>(&mut self, c: C) -> Result<&mut Self, Error> {
        self.world.insert_component(self.id, c)?;
        Ok(self)
    }

    pub fn remove<C: 'static>(&mut self) -> Result<&mut Self, Error> {
        self.world.remove_component::<C>(self.id)?;
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
            .id();
        let e2 = world
            .add_entity()
            .insert(Position(0, 0))
            .unwrap()
            .insert(Velocity(3, 4))
            .unwrap()
            .id();
        let e3 = world.add_entity().insert(Position(0, 0)).unwrap().id();

        for (mut pos,) in world.with_components::<(Position,)>() {
            pos.0 = 10;
        }

        for (mut pos, vel) in world.with_components::<(Position, Velocity)>() {
            pos.0 += vel.0;
            pos.1 += vel.1;
        }

        assert_eq!(world.get_component::<Position>(e1).unwrap().0, 15);
        assert_eq!(world.get_component::<Position>(e2).unwrap().0, 13);
        assert_eq!(world.get_component::<Position>(e3).unwrap().0, 10);

        world.remove_component::<Position>(e1).unwrap();

        for (mut pos,) in world.with_components::<(Position,)>() {
            pos.0 = 5;
        }

        assert_eq!(world.get_component::<Position>(e2).unwrap().0, 5);
        assert_eq!(world.get_component::<Position>(e3).unwrap().0, 5);

        world.remove_entity(e3);

        for (mut pos, vel) in world.with_components::<(Position, Velocity)>() {
            pos.0 += vel.0;
            pos.1 += vel.1;
        }

        assert_eq!(world.get_component::<Position>(e2).unwrap().0, 8);
    }
}
