// An attempt to make a procedural Mythos generator
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rand;

use rand::Rng;
use std::fmt;

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

impl<'a> fmt::Display for Relation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {:?} of {}", self.source.name, self.type_, self.destiny.name)
    }
}

struct Relations<'a>(&'a mut Vec<Relation<'a>>);

impl<'a> Relations<'a> {
    fn add(&mut self, type_: RelationType, source: &'a Entity, destiny: &'a Entity) {
        self.0.push(Relation{ type_, source, destiny });
    }

    fn generate_child_parent(&mut self, entites: &'a mut Vec<Entity>) {
        let s = entites.len();
        let n = rand::thread_rng().gen_range(3, s/2+3);
        info!("{} relations child parent", n);
        
        for _i in 0..n {
            let parent = &entites[rand::thread_rng().gen_range(0, s)];
            let child = &entites[rand::thread_rng().gen_range(0, s)];

            info!("parent: {:?}, child: {:?}", parent.name, child.name);
            self.add(RelationType::Child, child, parent);
            self.add(RelationType::Parent, parent, child);
        }
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
    for i in 0..20 {
        entites.push(Entity{ name: format!("ent{}", i) });
    }
    
    let mut rel: Vec<Relation> = vec![];
    let mut relations = Relations(&mut rel);
    
    relations.generate_child_parent(&mut entites);

    //relations.add(RelationType::Parent, &entites[0], &entites[1]);
    //relations.add(RelationType::Child, &entites[1], &entites[0]);

    //for i in entites.iter() { println!("{:?}", i); }
    for i in relations.0.iter() { println!("{}", i); }
}
