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

        Ok(_) => info!("File {} writed sucessfully", path),
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Entity {
    name: String,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
enum RelationType {
    Base,
    Parent,
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

    fn adjacent_out(&mut self, vertex: usize) -> Vec<usize> {
        let mut v: Vec<usize> = vec![];
        
        for j in 0..self.relations[vertex].len() {
            match &self.relations[vertex][j] {
                &Some(_) => v.push(j),
                &None => (),
            }
        }

        v
    }

    fn generate_base_relation(&mut self) {
        let s = self.entites.len();
        trace!("self.entites: {}", s);

        let n = s as usize;
        trace!("num relations: {}", n);

        for _i in 0..n {
            let mut src = rand::thread_rng().gen_range(0, s);
            let mut dest = rand::thread_rng().gen_range(0, s);
            trace!("src: {:?}, dest: {:?}", src, dest);
            
            while src == dest  {
                trace!("I can't be my own src");
                dest = rand::thread_rng().gen_range(0, s);
                trace!("New dest: {}", dest);
            }
            
            while let &Some(ref _rt) = &self.relations[src][dest] {
                trace!("You already are the src");
                src = rand::thread_rng().gen_range(0, s);
                trace!("New src: {}", src);
            }

            let mut stack: Vec<usize> = vec![];
            let mut verif = vec![false; n];
            stack.push(dest);
            trace!("Verificando ciclos");
            while let Some(top) = stack.pop() {
                trace!("v: {}", top);

                if verif[top] || top == src {
                    trace!("A cicle identifyed");

                    stack.clear();
                    verif = vec![false; n];
                    dest = rand::thread_rng().gen_range(0, s);
                    stack.push(dest);

                    trace!("New dest: {}",dest);

                } else {
                    verif[top] = true;
                    let adj = self.adjacent_out(top);
                    for i in adj.iter() { stack.push(*i); }
                }
            }

            info!("src: {:?}, dest: {:?}", src, dest);
            self.add(src, dest, RelationType::Base);
        }
    }

    fn generate_dot(&mut self, name: Option<String>) {
        let mut s: String = "digraph G {\n".to_string();

        for e in self.entites.iter() {
            s += &format!("\t{}\n", e.name);
        }
        
        s += "\n";
        let n = self.entites.len();
        for i in 0..n {
            for j in 0..n {
                match &self.relations[i][j] {
                    &Some(RelationType::Parent) => {
                        s += &format!(
                            "\t {} -> {}\n",
                            self.entites[i].name,
                            self.entites[j].name
                        )
                    },

                    &Some(RelationType::Base) => {
                        s += &format!(
                            "\t {} -> {} [color=\"grey41\"]\n",
                            self.entites[i].name,
                            self.entites[j].name
                        )
                    },
                    _ => s += "",
                };
            }
        }

        s += "}";
        let s_slice: &str = &*s;
        
        let fl: String;
        match name {
            None => fl = "relations.dot".to_string(),
            Some(n) => fl = n + ".dot",
        }
        let fl_slice: &str = &*fl;
        
        write_file(&s_slice, &fl_slice);
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
    
    relations.generate_base_relation();
//    println!("{:?}", relations);
    relations.generate_dot(None);
}
