// Generate description

use RelationType;
use Relations;

fn entity_description(relations: &Relations, e: usize) -> String {
    let get_names = |v: Vec<usize>| -> String {
        let n = v.len();
        let mut s: String = String::new();

        s += &relations.entites[v[0]].name;

        if n == 2 {
            s += &format!(" and {}", &relations.entites[v[1]].name);

        } else if n > 2 {
            for i in 1..n-1 {
                s += &format!(", {}", &relations.entites[v[i]].name);
            }

            s += &format!(" and {}", &relations.entites[v[n-1]].name);
        }

        s
    };


    let mut s: String = format!("{}", relations.entites[e].name);

    let adj_in = relations.adjacent_in(e);
    if adj_in.len() == 0 {
        s += " children of the Void";

    } else {
        let rt = relations.relations[adj_in[0]][e].unwrap();

        s += match rt {
            RelationType::Invoker => " invoked by",
            RelationType::Creator => " created by",
            RelationType::Parent => " children of",
            _ => "",
        };

        s += &format!(" {}", get_names(adj_in));
    }

    s += ".";
    s
}

pub fn get_descriptions(relations: &Relations) -> String {
    let mut s: String = String::new();
    let n = relations.entites.len();

    for i in 0..n-1 {
        s += &format!("{}\n", entity_description(relations, i));
    }
    s += &entity_description(relations, n-1);

    s
}

