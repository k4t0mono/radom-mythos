// An attempt to make a procedural Mythos generator
#![allow(dead_code)]

#[derive(Debug)]
enum RelationType {
    Parent,
    Child,
    Sibling
}

#[derive(Debug)]
struct Relation {
    relation_type: RelationType,
    destiny: u8,
}

#[derive(Debug)]
struct Entity {
    name: String,
    relations: Vec<Relation>,
}

fn main() {
    println!("Random Mythos engage");
    
    let mut entities: Vec<Entity> = vec![];
    for n in 0..2 {
        entities.push(Entity{ name: format!("Entity{}", n), relations: vec![] });
    }
    
    
    entities[0].relations.push(Relation {
        relation_type: RelationType::Parent,
        destiny: 1
    });

    entities[1].relations.push(Relation {
        relation_type: RelationType::Child,
        destiny: 0
    });

    println!("{:?}", entities);
}
