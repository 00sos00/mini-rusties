#![allow(dead_code)]
use crate::bit_field::BitField;
use anymap::AnyMap;
use hashbrown::{HashMap, HashSet};
use std::{any::TypeId, borrow::BorrowMut, sync::Mutex};

type EntityId = usize;
type EntityContainer = Mutex<Entity>;
type ComponentContainer<T> = Mutex<Option<T>>;

#[derive(Debug)]
pub struct Entity {
    id: EntityId,
    group_id: String,
    components_bitfield: BitField,
}

pub struct EntityStorage {
    max_entities: usize,
    max_registered_components: usize,
    entities: Vec<EntityContainer>,
    groups: HashMap<String, HashSet<EntityId>>,
    archetypes: HashMap<BitField, HashSet<EntityId>>,
    components: AnyMap,
    component_bitfields: HashMap<TypeId, BitField>,
    free_entity_ids: Vec<EntityId>,
}

impl EntityStorage {
    pub fn new(max_entities: usize, max_registered_components: usize) -> Self {
        Self {
            max_entities,
            max_registered_components,
            entities: Vec::with_capacity(max_entities),
            groups: HashMap::new(),
            archetypes: HashMap::new(),
            components: AnyMap::with_capacity(max_registered_components),
            component_bitfields: HashMap::with_capacity(max_registered_components),
            free_entity_ids: vec![],
        }
    }

    pub fn register_group(&mut self, group_id: &str) {
        self.groups.insert(group_id.to_string(), HashSet::new());
    }

    pub fn register_component<T: 'static>(&mut self) {
        let mut bitfield = BitField::with_bits(self.max_registered_components);
        bitfield.set_nth_bit(self.components.len());

        let mut component_vec: Vec<ComponentContainer<T>> = Vec::with_capacity(self.max_entities);
        ((0..self.max_entities).for_each(|_| component_vec.push(Mutex::new(None))));

        self.components.insert(component_vec);
        self.component_bitfields.insert(TypeId::of::<T>(), bitfield);
    }

    pub fn create_entity(&mut self, group_id: Option<&str>) -> EntityId {
        assert!(
            self.entities.len() <= self.max_entities,
            "Max entities reached"
        );

        let group_id = group_id.unwrap_or("").to_string();
        let components_bitfield = BitField::with_bits(self.max_registered_components);

        if let Some(entity_id) = self.free_entity_ids.pop() {
            *self.entities.get_mut(entity_id).unwrap() = Mutex::new(Entity {
                id: entity_id,
                group_id: group_id.clone(),
                components_bitfield,
            });

            if let Some(group) = self.groups.get_mut(&group_id) {
                group.insert(entity_id);
            }

            return self.entities.get(entity_id).unwrap().lock().unwrap().id;
        } else {
            let entity_id = self.entities.len();

            self.entities.push(Mutex::new(Entity {
                id: entity_id,
                group_id: group_id.clone(),
                components_bitfield,
            }));

            if let Some(group) = self.groups.get_mut(&group_id) {
                group.insert(entity_id);
            }

            return self.entities.last().unwrap().lock().unwrap().id;
        };
    }

    pub fn add_component_to<T: 'static>(&mut self, entity_id: EntityId, component: T) {
        let component_bitfield =
            if let Some(component_bitfield) = self.component_bitfields.get(&TypeId::of::<T>()) {
                component_bitfield
            } else {
                return;
            };

        let entity_container = if let Some(entity_container) = self.entities.get(entity_id) {
            entity_container
        } else {
            return;
        };

        let mut entity = if let Ok(entity) = entity_container.try_lock() {
            entity
        } else {
            return;
        };

        if let Some(old_archetype) = self.archetypes.get_mut(&entity.components_bitfield) {
            old_archetype.remove(&entity_id);
        }

        entity.components_bitfield = entity.components_bitfield.or(component_bitfield);

        let new_archetype = self
            .archetypes
            .entry(entity.components_bitfield.clone())
            .or_insert(HashSet::new());

        new_archetype.insert(entity_id);

        let component_vec =
            if let Some(component_vec) = self.components.get::<Vec<ComponentContainer<T>>>() {
                component_vec
            } else {
                return;
            };

        if let Ok(mut component_option) = component_vec[entity_id].try_lock() {
            component_option.borrow_mut().replace(component);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Position(f32, f32);
    struct Velocity(f32, f32);

    #[test]
    fn create_entity() {
        let mut entity_storage = EntityStorage::new(1024, 16);

        let l = std::time::Instant::now();
        for _ in 0..100 {
            entity_storage.create_entity(None);
        }

        println!("{}", l.elapsed().as_nanos());
    }
}
