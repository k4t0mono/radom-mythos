// An attempt to make a procedural Mythos generator
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rand;

use std::fs::OpenOptions;
use std::io::prelude::*;
use rand::Rng;
use std::fmt;


fn write_file<'a>(name: &'a str, path: &'a str) {
    let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path);

    let mut file = match f {
        Err(e) => {
            error!("Something is terrible wrong happend while oppening the file");
            error!("{}", e);

            panic!(e)
        },

        Ok(fl) => fl,
    };
    
    match file.write_all(name.as_bytes()) {
        Err(e) => {
            error!("Something is terrible wrong happend while writing the file");
            error!("{}", e);

            panic!(e)
        },

        Ok(_) => info!("File writed sucessfully"),
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Entity {
    name: String,
}


#[derive(Debug, PartialEq, Eq)]
enum RelationType {
    Parent,
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

    fn generte_dot(&mut self) {
        let mut s: String = "digraph G {\n".to_string();

        for e in self.entites.iter() {
            s += &format!("\t{}\n", e.name);
        }
        
        s += "\n";
        for &Relation{ ref type_, ref source, ref destiny } in self.relations.iter() {
            match type_ {
                &RelationType::Parent => s += &format!("\t{} -> {}\n", source.name, destiny.name),
            }
        }

        s += "}";
        let s_slice: &str = &*s;
        write_file(&s_slice, "relations.dot");
    }

    fn exists(&mut self, i_src: usize, i_dest: usize, rt: RelationType) -> bool {
        let src = &self.entites[i_src];
        let dest = &self.entites[i_dest];

        for &Relation{ ref type_, ref source, ref destiny } in self.relations.iter() {
            if *type_ == rt && *source == src && *destiny == dest {
                return true   
            }
        }

        false
    }

    fn generate_child_parent(&mut self) {
        let s = self.entites.len();
        // let n = rand::thread_rng().gen_range(3, s/2+3);
        let n = 4*s/4 as usize;
        info!("{} relations child parent", n);
        
        for _i in 0..n {
            let mut i_parent = rand::thread_rng().gen_range(0, s);
            let mut i_child = rand::thread_rng().gen_range(0, s);
            trace!("parent: {:?}, child: {:?}", i_parent, i_child);
            
            while i_parent == i_child  {
                trace!("I can't be my own parent");
                i_child = rand::thread_rng().gen_range(0, s);
                trace!("New child: {}", i_child);
            }

            let parent = &self.entites[i_parent];
            let child = &self.entites[i_child];
            
            while self.exists(i_parent, i_child, RelationType::Parent) {
                trace!("You already are the parent");
                i_parent = rand::thread_rng().gen_range(0, s);
                trace!("New parent: {}", i_parent);
            }

            info!("parent: {:?}, child: {:?}", parent.name, child.name);
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
    for i in 0..10 {
        entites.push(Entity{ name: format!("ent{:02}", i) });
    }
    
    let mut rel: Vec<Relation> = vec![];
    let mut relations = Relations{ relations: &mut rel, entites: &entites };
    
    relations.generate_child_parent();

    relations.generte_dot();
}
