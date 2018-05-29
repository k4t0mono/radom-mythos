// Generate description

use relations::RelationType;
use domains::*;
use Mythos;

fn get_names(mythos: &Mythos, v: Vec<usize>) -> String {
	let n = v.len();
	let mut s: String = String::new();

	s += &mythos.entites[v[0]].name;

	if n == 2 {
		s += &format!(" and {}", &mythos.entites[v[1]].name);

	} else if n > 2 {
		for i in 1..n-1 {
			s += &format!(", {}", &mythos.entites[v[i]].name);
		}

		s += &format!(" and {}", &mythos.entites[v[n-1]].name);
	}

	s
}

fn get_entity_domain(domain: &Domain) -> String {
	let get_title = |level: u8| -> &'static str {
		if (level > 0) && (level <= 63) { return "novice"; }
		else if (level > 63) && (level <= 127) { return "studant"; }
		else if (level > 127) && (level <= 189) { return "apprentice"; }
		else { return "master"; }
	};

	let pri = domain.get_primary().unwrap();
	let sec = domain.get_secundary().unwrap();
	format!(
		"{} of {} and {} of {}",
		get_title(pri.1),
		pri.0.get_name(),
		get_title(sec.1),
		sec.0.get_name(),
	)
}

// TODO: Passar ref para entidade
fn entity_description(mythos: &Mythos, e: usize) -> String {
	let mut s: String = format!("{}", mythos.entites[e].name);

	if mythos.entites[e].domain.get_primary().is_some() {
		s += &format!(
			", {},",
			get_entity_domain(&mythos.entites[e].domain)
		);
	}

	let adj_in = mythos.relations.get_adj_in(e);
	if adj_in.len() == 0 {
		s += " children of the Void";

	} else {
		let rt = mythos.relations.get(adj_in[0], e).unwrap();

		s += match rt {
			RelationType::Invoker => " invoked by",
			RelationType::Creator => " created by",
			RelationType::Parent => " children of",
			_ => "",
		};

		s += &format!(" {}", get_names(mythos, adj_in));
	}

	s += ".";
	s
}

pub fn get_descriptions(mythos: &Mythos) -> String {
	let mut s: String = String::new();
	let n = mythos.entites.len();

	for i in 0..n-1 {
		s += &format!("{}\n", entity_description(mythos, i));
	}
	s += &entity_description(mythos, n-1);

	s
}

