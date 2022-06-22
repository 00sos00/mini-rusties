#![allow(dead_code)]
use crate::bit_field::BitField;
use hashbrown::{HashMap, HashSet};
use parking_lot::RwLock;
use std::any::{Any, TypeId};

type EntityId = usize;
type EntityContainer = RwLock<Entity>;
type ComponentContainer<T> = RwLock<Option<T>>;

#[derive(Debug, Clone)]
pub struct Entity {
    id: EntityId,
    group_id: String,
    components_bitfield: BitField,
}

pub struct EntityStorage {
    max_entities: usize,
    max_registered_components: usize,
    pub entities: RwLock<Vec<EntityContainer>>,
    groups: RwLock<HashMap<String, HashSet<EntityId>>>,
    archetypes: RwLock<HashMap<BitField, HashSet<EntityId>>>,
    components: RwLock<HashMap<TypeId, Box<dyn Any>>>,
    component_bitfields: HashMap<TypeId, BitField>,
    free_entity_ids: RwLock<Vec<EntityId>>,
}

impl EntityStorage {
    pub fn new(max_entities: usize, max_registered_components: usize) -> Self {
        let mut entities = vec![];

        (0..max_entities).for_each(|id| {
            entities.push(RwLock::new(Entity {
                id,
                group_id: String::new(),
                components_bitfield: BitField::with_bits(max_registered_components),
            }))
        });

        Self {
            max_entities,
            max_registered_components,
            entities: RwLock::new(entities),
            groups: RwLock::new(HashMap::from([(String::new(), HashSet::new())])),
            archetypes: RwLock::new(HashMap::new()),
            components: RwLock::new(HashMap::with_capacity(max_registered_components)),
            component_bitfields: HashMap::with_capacity(max_registered_components),
            free_entity_ids: RwLock::new((0..max_entities).rev().collect::<Vec<usize>>()),
        }
    }

    pub fn register_group(&self, group_id: &str) {
        if let Some(mut groups) = self.groups.try_write() {
            groups.insert(group_id.to_string(), HashSet::new());
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        if let Some(mut components) = self.components.try_write() {
            let mut bitfield = BitField::with_bits(self.max_registered_components);
            bitfield.set_nth_bit(components.len());

            let mut component_vec: Vec<ComponentContainer<T>> =
                Vec::with_capacity(self.max_entities);
            ((0..self.max_entities).for_each(|_| component_vec.push(RwLock::new(None))));

            components.insert(TypeId::of::<T>(), Box::new(component_vec));
            self.component_bitfields.insert(TypeId::of::<T>(), bitfield);
        }
    }

    pub fn create_entity(&self, group_id: Option<&str>) -> Option<EntityId> {
        let entity_id = self.free_entity_ids.try_write()?.pop()?;

        let entities = self.entities.try_read()?;
        let mut entity = entities.get(entity_id).unwrap().try_write()?;

        let group_id = group_id.unwrap_or("").to_string();
        entity.group_id = group_id.clone();

        self.groups
            .try_write()?
            .get_mut(&group_id)?
            .insert(entity_id);

        Some(entity_id)
    }

    pub fn add_component_to<T: 'static>(&self, entity_id: EntityId, component: T) -> Option<()> {
        let l = std::time::Instant::now();
        let component_bitfield = self.component_bitfields.get(&TypeId::of::<T>())?;

        
        let entities = self.entities.try_read()?;
        let mut entity = entities.get(entity_id).unwrap().try_write()?;
        
        let mut archetypes = self.archetypes.try_write()?;
        
        if let Some(old_archetype) = archetypes.get_mut(&entity.components_bitfield) {
            old_archetype.remove(&entity_id);
        }
        
        entity.components_bitfield = entity.components_bitfield.or(component_bitfield);
        
        let new_archetype = archetypes
        .entry(entity.components_bitfield.clone())
        .or_insert(HashSet::new());
        
        new_archetype.insert(entity_id);
        
        let components = self.components.try_read()?;
        let component_vec = components
        .get(&TypeId::of::<T>())?
        .downcast_ref::<Vec<ComponentContainer<T>>>()?;
        
        let mut old_component = component_vec[entity_id].try_write()?;
        
        old_component.replace(component);
        println!("{}", l.elapsed().as_nanos());

        Some(())
    }
}
