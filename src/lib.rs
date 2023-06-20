// Issues with this code:
//  (1) not sure if we want to use the index of the vec or a hashmap with keys.
//      Which is more efficient?
//  (2) Component: 'static necessary? Because T (or C) of impl Component might
//      not live long enough to store in vec. Cloneable? Or is this right?
//  (3) Currently each component is stored in a Vec<Components of the same ID>.
//      Should we take the vec out and just allow one componet with said ID?

use core::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Component: 'static {}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct ComponentId(usize); // An ID for a given component.

pub struct World {
    component_map: Vec<TypeId>,
    table: Table,
}

impl World {
    pub fn new() -> Self {
        Self { component_map: Vec::new(), table: Table::new() }
    }

    pub fn init_component_id<C: Component>(&mut self) -> ComponentId {
        let component_type_id = TypeId::of::<C>();
        // First check if the component already exists in this world.
        if let Some(c) = self.component_map.iter().position(|&t| t == component_type_id) {
            return ComponentId(c);
        };

        self.component_map.push(component_type_id.clone());
        ComponentId(self.component_map.len() - 1)
    }

    pub fn get_component_id<C: Component>(&self) -> Option<ComponentId> {
        let component_type_id = TypeId::of::<C>();
        self.component_map.iter().position(|&t| t == component_type_id).map(|id| ComponentId(id))
    }
    
    // TODO: remove component_id
    
    pub fn insert_entity<E: Component>(&mut self, e: E) {
        let component_id = self.init_component_id::<E>();
        self.table.insert_entity(component_id, e);
    }
    
    /// Get a list of entities stored in this world with the type C: Component.
    pub fn get_entities<E: Component>(&self) -> Vec<&E> {
        let component_id = self.get_component_id::<E>().unwrap(); // TODO
        let column = self.table.get_column(component_id).unwrap(); // TODO
        let entities = column.get().iter().map(|e_erased| e_erased.downcast_ref::<E>().unwrap()).collect::<Vec<&E>>();
        entities
    }
}

/// Contains all entities of components.
pub struct Table {
    map: HashMap<ComponentId, Column>
}

impl Table {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
    
    pub fn insert_entity<E: Component>(&mut self, id: ComponentId, e: E) {
        if let Some(column) = self.map.get_mut(&id) {
            // We have a column already, insert it into the column.
            column.push(Box::new(e));
        } else {
            // We need to create a column for this ID first.
            let mut column = Column::new();
            
            // Now push this specific entity.
            column.push(Box::new(e));
            
            // Now insert it into the map.
            self.map.insert(id, column);
        }
    }
    
    // TODO: remove entity
    
    pub fn get_column(&self, id: ComponentId) -> Option<&Column> {
        self.map.get(&id)
    }
}

/// Contains all entities (e's) of a specific component, but is type-erased.
pub struct Column {
    column: Vec<Box<dyn Any>>,
}

impl Column {
    pub fn new() -> Self {
        Self { column: Vec::new() }
    }
    
    pub fn push(&mut self, e: Box<dyn Any>) {
        self.column.push(e);
    }
    
    // TODO: remove row
    
    pub fn get_row(&self, row: usize) -> Option<&Box<dyn Any>> {
        self.column.get(row)
    }
    
    pub fn get(&self) -> &Vec<Box<dyn Any>> {
        &self.column
    }
}

mod test {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct MyComponent(usize);
    impl Component for MyComponent {}

    #[test]
    fn big_world() {
        let my_entity_a = MyComponent(1);
        let my_entity_b = MyComponent(10);
        let my_entity_c = MyComponent(100);
    
        let mut world = World::new();
        
        world.insert_entity(my_entity_a);
        world.insert_entity(my_entity_b);
        world.insert_entity(my_entity_c);
        
        let entities = world.get_entities::<MyComponent>();
        println!("{entities:?}");

        assert_eq!(entities, vec![&MyComponent(1), &MyComponent(10), &MyComponent(100)]);
    }
}

