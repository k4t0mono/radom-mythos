// An attempt to make a procedural Mythos generator

extern crate rand;

use rand::Rng;

#[derive(Debug)]
struct Entity {
    name: String,
    parent0: i16,
}

fn main() {
    println!("Random Mythos engage");

    let mut entites: Vec<Entity> = vec![];

    for i in 0..10 {
        entites.push(Entity{ name: format!("ent{}", i), parent0: -1 });
    }

    let n_parent_child = rand::thread_rng().gen_range(2, 7);
    println!("n_parent_child: {}", n_parent_child);
    
    for _i in 0..n_parent_child {
        let mut parent: i16 = rand::thread_rng().gen_range(0, 10);
        let mut child: i16 = rand::thread_rng().gen_range(0, 10);
        
        println!("tentando: {} , {}", parent, child);

        while parent == child {
            println!("\t{} não pode ser pai de si msm", parent);
            parent = rand::thread_rng().gen_range(0, 10);
        }

        while entites[child as usize].parent0 > -1 {
            println!("\t{} já tem pai", child);
            child = rand::thread_rng().gen_range(0, 10);
        }

        println!("parent: {}, child: {}", parent, child);

        entites[child as usize].parent0 = parent;
    }

    println!();
    for e in entites.iter() {
        let parent = e.parent0 as usize;
        if e.parent0 > -1  {
            println!("{} is child of {}", e.name, entites[parent].name);

        } else {
            println!("{} is child of nobody", e.name);
        }
    }
}
