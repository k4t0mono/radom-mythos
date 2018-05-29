// Dot related functions

use domains::*;
use relations::RelationType;
use Entity;
use Mythos;

const NODE_COLORS: &'static str = "color=\"#e9e9f4\" fontcolor=\"#e9e9f4\""; 


fn domain_to_dot(domain: &Domain) -> String {
	format!(
		"{{ 🜄{:02x} 🜃{:02x} | 🜂{:02x} 🜁{:02x} }}",
		domain.get(DomainType::Water),
		domain.get(DomainType::Earth),
		domain.get(DomainType::Fire),
		domain.get(DomainType::Air),
	)
}


fn entity_to_dot(entity: &Entity) -> String {
	format!(
		"{0} [label=\"{{ {0} | {{ {1} | {2} }} }}\" {3}]",
		entity.name,
		entity.level,
		domain_to_dot(&entity.domain),
		NODE_COLORS,
	)
}


pub fn relations_to_dot(mythos: &Mythos) -> String {
	let get_color = |rt: &RelationType| -> &str {
		match rt {
			&RelationType::Base => "#909090",
			&RelationType::Invoker => "#ea51b2",
			&RelationType::Creator => "#00f769",
			&RelationType::Parent => "#62d6e8",
		}
	};

	let relation_to_dot = |i: usize, j: usize, rt: &RelationType| -> String {
		format!(
			"{} -> {} [color=\"{}\"]",
			mythos.entites[i].name,
			mythos.entites[j].name,
			get_color(rt),
		)
	};

	let mut s: String = "digraph G {\n".to_string();
	s += "\tgraph [bgcolor=\"#282936\"]\n";
	s += "\tnode [shape=record style=rounded]\n\n";

	for e in mythos.entites.iter() {
		s += &format!("\t{}\n", entity_to_dot(e));
	}

	s += "\n";

	let n = mythos.entites.len();
	for i in 0..n {
		for j in 0..n {
			match &mythos.relations.get(i, j) {
				&Some(ref rt) => s += &format!("\t{}\n", relation_to_dot(i, j, rt)),
				&None => (),
			}
		}
	}

	s += "}";

	s
}
