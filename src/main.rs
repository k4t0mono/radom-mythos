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

struct Relations<'a>{
    relations: &'a mut Vec<Relation<'a>>,
    entites: &'a Vec<Entity>,
}

impl<'a> Relations<'a> {
    fn add(&mut self, type_: RelationType, source: &'a Entity, destiny: &'a Entity) {
        self.relations.push(Relation{ type_, source, destiny });
    }

    fn generte_dot(&mut self) -> String {
        let mut s: String = "digraph G {\n".to_string();

        for e in self.entites.iter() {
            s += &format!("\t{}\n", e.name);
        }
        
        s += "\n";
        for &Relation{ ref type_, ref source, ref destiny } in self.relations.iter() {
            match type_ {
                &RelationType::Parent => s += &format!("\t{} -> {}\n", source.name, destiny.name),
                _ => trace!("Noting for you uwu"),
            }
        }

        s += "}";
        s
    }

    fn generate_child_parent(&mut self) {
        let s = self.entites.len();
        trace!("s: {}", s);
        // let n = rand::thread_rng().gen_range(3, s/2+3);
        let n = 4*s/4 as usize;
        info!("{} relations child parent", n);
        
        for _i in 0..n {
            let parent = &self.entites[rand::thread_rng().gen_range(0, s)];
            let child = &self.entites[rand::thread_rng().gen_range(0, s)];

            info!("parent: {:?}, child: {:?}", parent.name, child.name);
            self.add(RelationType::Child, child, parent);
            self.add(RelationType::Parent, parent, child);
        }
    }
}

fn set_logger() {
    use simplelog::*;
    
    TermLogger::init(LevelFilter::Trace, Config::default()).unwrap();
}

fn main() {
    set_logger();

    info!("Random Mythos engage");

    let mut entites: Vec<Entity> = vec![];
    for i in 0..42 {
        entites.push(Entity{ name: format!("ent{}", i) });
    }
    
    let mut rel: Vec<Relation> = vec![];
    let mut relations = Relations{ relations: &mut rel, entites: &entites };
    
    relations.generate_child_parent();

    //relations.add(RelationType::Parent, &entites[0], &entites[1]);
    //relations.add(RelationType::Child, &entites[1], &entites[0]);

    //for i in entites.iter() { println!("{:?}", i); }
    for i in relations.entites.iter() { println!("{:?}", i); }
    println!("{}", relations.generte_dot());
}
