// An attempt to make a procedural Mythos generator
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rand;

use std::fs::OpenOptions;
use std::io::prelude::*;
use rand::Rng;
use std::fmt;

#[allow(dead_code)]
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


#[derive(Debug, PartialEq, Eq, Clone)]
enum RelationType {
    Parent,
    Other,
}


struct Relations<'a> {
    entites: &'a Vec<Entity>,
    relations: Vec<Vec<Option<RelationType>>>,
}

impl<'a> fmt::Debug for Relations<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_string();

        for i in self.relations.iter() {
            for j in i.iter() {
                match j {
                    &None => s += "---",
                    &Some(ref v) => s += &format!("{:?}", v),
                }
                s += "\t";
            }
            s += "\n";
        }

        write!(f, "{}", s)
    }
}

impl<'a> Relations<'a> {
    pub fn init(e: &'a Vec<Entity>) -> Relations<'a> {
        let s = e.len();
        let r: Vec<Vec<Option<RelationType>>> = vec![vec![None; s]; s];

        Relations{ entites: e, relations: r }
    }

    fn add(&mut self, source: usize, destiny: usize, rt : RelationType) {
        self.relations[source][destiny] = Some(rt);
    }


    fn generate_child_parent(&mut self) {
        let s = self.entites.len();
        trace!("self.entites: {}", s);

        let n = s as usize;
        trace!("num relations: {}", n);

        for _i in 0..n {
            let mut i_parent = rand::thread_rng().gen_range(0, s);
            let mut i_child = rand::thread_rng().gen_range(0, s);
            trace!("parent: {:?}, child: {:?}", i_parent, i_child);
            
            while i_parent == i_child  {
                trace!("I can't be my own parent");
                i_child = rand::thread_rng().gen_range(0, s);
                trace!("New child: {}", i_child);
            }
            
            while let &Some(ref _rt) = &self.relations[i_parent][i_child] {
                trace!("You already are the parent");
                i_parent = rand::thread_rng().gen_range(0, s);
                trace!("New parent: {}", i_parent);
            }

            info!("parent: {:?}, child: {:?}", i_parent, i_child);
            self.relations[i_parent][i_child] = Some(RelationType::Parent);
        }
    }

    fn generate_dot(&mut self) {
        let mut s: String = "digraph G {\n".to_string();

        for e in self.entites.iter() {
            s += &format!("\t{}\n", e.name);
        }
        
        s += "\n";
        let n = self.entites.len();
        for i in 0..n {
            for j in 0..n {
                match &self.relations[i][j] {
                    &Some(RelationType::Parent) => s += &format!("\t {} -> {}\n", self.entites[i].name, self.entites[j].name),
                    _ => s += "",
                };
            }
        }

        s += "}";
        let s_slice: &str = &*s;
        write_file(&s_slice, "relations.dot");
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

    let mut relations = Relations::init(&entites);
    relations.add(0, 0, RelationType::Other);
    
    relations.generate_child_parent();
    println!("{:?}", relations);
    relations.generate_dot();
}
