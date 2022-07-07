mod coerce;
mod holder;
mod borrow_state;

use holder::Holder;
use paste::paste;

struct EntityStorage;

macro_rules! query_from {
    ($entity_storage:expr, $($component_type:ty,)+) => {
        let entity_storage: EntityStorage = $entity_storage;
        let mut query_bitfield = BitField::new();

        $(
            let bitfield = entity_storage.bitfields.get(&TypeId::of::<<$component_type>::Unborrowed>());
            query_bitfield = query_bitfield.or(bitfield);
        )+

        let archetype_entities = entity_storage.archetypes.get(query_bitfield).unwrap();
        let mut query = vec![];

        for entity in archetype_entities {
            query.push(
                (entity, ($(
                    let type_id = TypeId::of::<<$component_type>::Unborrowed>();
                    let component_vec = entity_storage.components.get(&type_id).unwrap();

                    component_vec[entity].get::<$component_type>().unwrap()
                ),+))
            )
        }

        query
    };
}

macro_rules! test {
    ($t:ty) => {
        paste! { [<_ $t>] }
    };
}

fn main() {
    test!(u8);

    let holder = Holder::new(0u8);

    let holder_val1 = holder.get::<&u8>().unwrap();
    let holder_val2 = holder.get::<&mut u8>().unwrap();

    println!("{holder_val1} {holder_val2}");

    *holder_val2 += 1;

    println!("{}", holder.get::<&u8>().unwrap());
}
