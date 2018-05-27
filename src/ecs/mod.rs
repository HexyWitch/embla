use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{BTreeSet, HashMap};

use failure::Error;

mod component;

use self::component::{ComponentSet, ComponentStorage, GenericComponentStorage};

pub struct World {
    component_index: HashMap<TypeId, BTreeSet<usize>>,
    components: HashMap<TypeId, RefCell<Box<GenericComponentStorage>>>,
    entities: Vec<Option<HashMap<TypeId, usize>>>,
    dead: Vec<usize>,
}

impl World {
    pub fn new() -> World {
        World {
            component_index: HashMap::new(),
            components: HashMap::new(),
            entities: Vec::new(),
            dead: Vec::new(),
        }
    }

    pub fn register_component<C: 'static>(&mut self) {
        self.component_index
            .insert(TypeId::of::<C>(), BTreeSet::new());
        self.components.insert(
            TypeId::of::<C>(),
            RefCell::new(Box::new(ComponentStorage::<C>::new())),
        );
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
            self.entities[id] = Some(HashMap::new());
            id
        } else {
            let id = self.entities.len();
            self.entities.push(Some(HashMap::new()));
            id
        };
        EntityEntry {
            world: self,
            id: id,
        }
    }

    pub fn remove_entity(&mut self, id: usize) {
        if let Some(e) = self.entities.remove(id) {
            for type_id in e.keys() {
                self.components
                    .get_mut(type_id)
                    .unwrap()
                    .borrow_mut()
                    .remove(id)
            }
        }
        if id < self.entities.len() {
            self.entities[id] = None;
        }
        self.dead.push(id);
    }

    pub fn get_component<'b, C: 'static>(&'b self, id: usize) -> Option<Ref<'b, C>> {
        if let Some(k) = self.entities
            .get(id)
            .unwrap()
            .as_ref()
            .unwrap()
            .get(&TypeId::of::<C>())
        {
            Some(Ref::map(
                self.components
                    .get(&TypeId::of::<C>())
                    .expect("component not registered")
                    .borrow(),
                |v| {
                    v.as_any()
                        .downcast_ref::<ComponentStorage<C>>()
                        .unwrap()
                        .get(*k)
                        .unwrap()
                },
            ))
        } else {
            None
        }
    }

    fn get_storage<T: 'static>(&mut self) -> Result<RefMut<ComponentStorage<T>>, Error> {
        Ok(RefMut::map(
            self.components
                .get(&TypeId::of::<T>())
                .ok_or_else(|| format_err!("component not registered"))?
                .borrow_mut(),
            |s| {
                s.as_any_mut()
                    .downcast_mut::<ComponentStorage<T>>()
                    .unwrap()
            },
        ))
    }

    fn entity_mut(&mut self, id: usize) -> Result<&mut HashMap<TypeId, usize>, Error> {
        Ok(self.entities
            .get_mut(id)
            .ok_or_else(|| format_err!("entity does not exist"))?
            .as_mut()
            .ok_or_else(|| format_err!("entity is not alive"))?)
    }

    fn insert_component<T: 'static>(&mut self, id: usize, c: T) -> Result<(), Error> {
        let k = self.get_storage::<T>()?.insert(id, c);
        self.entity_mut(id)?.insert(TypeId::of::<T>(), k);
        Ok(())
    }

    fn remove_component<T: 'static>(&mut self, id: usize) -> Result<Option<T>, Error> {
        if let Some(k) = self.entity_mut(id)?.remove(&TypeId::of::<T>()) {
            Ok(self.get_storage::<T>()?.remove(k))
        } else {
            Ok(None)
        }
    }

    pub fn with_components<'b, C: ComponentSet<'b>>(
        &'b self,
    ) -> Box<Iterator<Item = C::MutRefs> + 'b> {
        C::iter_mut_refs(&self.entities, &self.components)
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
