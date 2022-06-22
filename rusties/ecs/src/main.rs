mod bit_field;
mod entity_storage;
use bevy_ecs::prelude::*;
#[derive(Component)]
struct Position(f32, f32);
#[derive(Component)]
struct Velocity(f32, f32);
#[derive(Component)]
struct Acceleration(f32, f32);

fn main() {
    let mut entity_storage = entity_storage::EntityStorage::new(422_500, 10000);

    entity_storage.register_component::<Position>();
    entity_storage.register_component::<Velocity>();
    entity_storage.register_component::<Acceleration>();
    
    let id = entity_storage.create_entity(None).unwrap();
    //let l = std::time::Instant::now();
    entity_storage.add_component_to(id, Position(0.0, 0.0)).unwrap();
    entity_storage.add_component_to(id, Velocity(0.0, 0.0)).unwrap();
    entity_storage.add_component_to(id, Acceleration(0.0, 0.0)).unwrap();

    /* for _ in 0..50000 {
        entity_storage.add_component_to(id, Position(0.0, 0.0)).unwrap();
    } */
    
    //println!("{}", entity_storage.entities.read().len());
    //println!("{} {:?}", l.elapsed().as_nanos(), entity_storage.entities.read().len());







    
    /* let mut world = World::new();

    let mut l = std::time::Instant::now();
    let entity1 = world.spawn()
    .insert(Position(0.0, 0.0))
    .insert(Velocity(0.0, 0.0))
    .insert(Acceleration(0.0, 0.0))
    .id();
    println!("{}", l.elapsed().as_nanos());

    l = std::time::Instant::now();
    let entity2 = world.spawn()
    .insert(Position(0.0, 0.0))
    .insert(Velocity(0.0, 0.0))
    .insert(Acceleration(0.0, 0.0))
    .id();
    println!("{}", l.elapsed().as_nanos()); */
}
