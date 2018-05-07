// An attempt to make a procedural Mythos generator
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rand;

//use rand::Rng;

#[derive(Debug)]
struct Entity {
    name: String,
}

#[derive(Debug)]
enum RelationType {
    Parent,
    Child,
}

#[derive(Debug)]
struct Relation<'a> {
    type_: RelationType,
    source: &'a Entity,
    destiny: &'a Entity,
}

struct Relations<'a>(&'a mut Vec<Relation<'a>>);

impl<'a> Relations<'a> {
    fn add(&mut self, type_: RelationType, source: &'a Entity, destiny: &'a Entity) {
        self.0.push(Relation{ type_, source, destiny });
    }
}

fn set_logger() {
    use simplelog::*;
    
    TermLogger::init(LevelFilter::Info, Config::default()).unwrap();
}

fn main() {
    set_logger();

    info!("Random Mythos engage");

    let mut entites: Vec<Entity> = vec![];
    for i in 0..10 {
        entites.push(Entity{ name: format!("ent{}", i) });
    }
    
    let mut rel: Vec<Relation> = vec![];
    let mut relations = Relations(&mut rel);
    
    relations.add(RelationType::Parent, &entites[0], &entites[1]);
    relations.add(RelationType::Child, &entites[1], &entites[0]);

    //for i in entites.iter() { println!("{:?}", i); }
    //for i in relations.0.iter() { println!("{:?}", i); }
}
