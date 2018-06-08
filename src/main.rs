// An attempt to make a procedural Mythos generator
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate docopt;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;
use docopt::Docopt;

mod relations;
mod domains;
mod plains;

mod dot;
mod description;
mod test;

pub fn write_file<'a>(data: &'a str, path: &'a str) {
	let mut f = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(path)
			.unwrap();

	f.write_all(data.as_bytes()).unwrap();
}

fn read_file<'a>(path: &'a str) -> String {
	let mut f = OpenOptions::new()
			.read(true)
			.open(path)
			.unwrap();

	let mut data = String::new();
	f.read_to_string(&mut data).unwrap();

	data
}


#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Entity {
	name: String,

	/* The base level is 0. The lower the level,
	 * more powerful the entity is.
	 */
	level: i8,
	domain: domains::Domain,
}


#[derive(Deserialize, Serialize)]
pub struct Mythos {
	entites: Vec<Entity>,
	relations: relations::Relations,
}

impl Mythos {
	pub fn init(size: usize) -> Mythos {
		if size == 0 { panic!("Size must be grater then 0"); }

		let mut entites: Vec<Entity> = vec![];
		for i in 0..size {
			entites.push(Entity{
				name: format!("ent{:02}", i),
				level: 0,
				domain: domains::Domain::new(),
			});
		}

		let relations = relations::Relations::new(size);

		Mythos{ entites, relations }
	}

	pub fn from_json(json: String) -> Mythos {
		let r: Mythos = serde_json::from_str(&json).unwrap();

		r
	}

	fn to_json(&self) -> String {
		let j = serde_json::to_string_pretty(self).unwrap();

		j
	}

	fn generate(&mut self) {
		self.relations.generate();
		self.fix_levels();
		self.generate_domains();
	}

	/* The levels are fixed following the topological order to avoid propagation
	 * of a wrong level, and to not access all children nodes every time a
	 * parent node is changed.
	 *
	 * Ajust the entity level according the following rules:
	 * - A invoked entity must be one level below the stronger invoker
	 * - A children entity must be the same level as the strogest parent
	 * - A created entity must be one level above the stronger creator
	 */
	fn fix_levels(&mut self) {
		use relations::RelationType;
		info!("fix_levels_toplogical");

		let entites = self.relations.get_topological_sort();
		debug!("topological_sort: {:?}", entites);
		for entity in entites.iter() {
			trace!("fixing {}", *entity);
			let adj_in = self.relations.get_adj_in(*entity);
			trace!("adj_in: {:?}", adj_in);
			if adj_in.is_empty() { continue; }

			let rt = self.relations.get(adj_in[0], *entity).unwrap();
			let mut min = self.entites[adj_in[0]].level;
			for j in adj_in.iter() {
				let l = self.entites[*j].level;
				if l < min { min = l; }
			}

			let inc = match rt {
				RelationType::Invoker(_) => -1,
				RelationType::Creator => 1,
				_ => 0,
			};

			let new_level = min + inc;
			if new_level - self.entites[*entity].level == 0 { continue; }

			self.entites[*entity].level = new_level;
		}
	}

	/* Generate the entity's domains following this rules:
	 * - A created or successfully invoked entity must have the average of the
	 *   creators domains
	 * - A children entity must be the cross_over of the parents
	 * - A failed invoked entity must have generated domain
	 * - All domains must be mutated
	 *
	 */
	fn generate_domains(&mut self) {
		use rand::Rng;
		use relations::RelationType;
		use domains::Domain;

		info!("Generate domains");

		for i in self.relations.get_roots().iter() {
			self.entites[*i].domain = Domain::gen_domain();
		}

		for &i in self.relations.get_topological_sort().iter() {
			let adj_in = self.relations.get_adj_in(i);
			if adj_in.is_empty() { continue; }
			debug!("Trying {}", i);

			let rt = self.relations.get(adj_in[0], i).unwrap();

			if rt == RelationType::Parent {
				trace!("I found a Parent type");

				let mut domains: Vec<Domain> = vec![];
				for &j in adj_in.iter() { domains.push(self.entites[j].domain.clone()); }

				self.entites[i].domain = Domain::cross_over_many(&domains);
				self.entites[i].domain.mutate();

			} else if rt == RelationType::Creator {
				debug!("I found a Creator type");

				let mut ds: Vec<Domain> = vec![];
				for k in adj_in.iter() { ds.push(self.entites[*k].domain.clone()); }

				self.entites[i].domain = Domain::gen_from_average(ds).mutate();

			} else if rt == RelationType::Invoker(true) {
				debug!("I found a Invoker type");

				if rand::thread_rng().gen_bool(0.7) {
					debug!("The invokation worked");

					let mut ds: Vec<Domain> = vec![];
					for k in adj_in.iter() { ds.push(self.entites[*k].domain.clone()); }
					self.entites[i].domain = Domain::gen_from_average(ds).mutate();

				} else {
					debug!("The invokation failed");

					for j in adj_in.iter() {
						self.relations.add(*j, i, RelationType::Invoker(false));
					}
					self.entites[i].domain = Domain::gen_domain().mutate();
				}
			}
		}
	}
}


const USAGE: &'static str = "
random-mythos

Usage:
    random-mythos [options] <file>
    random-mythos (-h | --help)
    random-mythos (-v | --version)

Options:
    -h --help           Show this screen
    -v --version        Show version
    -d --gen-dot        Generate relations' graph dot file
    --verbose=<n>       Set log level
    --export=<json>     Export relations to JSON file
    --import=<json>     Import relations from JSON file
";

#[derive(Debug, Deserialize)]
struct Args {
	flag_verbose: usize,
	flag_version: bool,
	flag_gen_dot: bool,
	flag_export: Option<String>,
	flag_import: Option<String>,
	arg_file: String,
}


fn set_logger(level: usize) {
	use simplelog::*;

	let log_level: LevelFilter = match level {
		0 => LevelFilter::Off,
		1 => LevelFilter::Error,
		2 => LevelFilter::Warn,
		3 => LevelFilter::Info,
		4 => LevelFilter::Debug,
		_ => LevelFilter::Trace,
	};

	TermLogger::init(log_level, Config::default()).unwrap();
}

fn init_mythos() -> Mythos {
	print!("Number of entites to generate: ");
	io::stdout().flush().unwrap();

	let mut num = String::new();
	io::stdin().read_line(&mut num)
		.expect("Failed to read line");

	let num: usize = match num.trim().parse() {
		Ok(num) => num,
		Err(_) => panic!("Enter a number"),
	};

	let mut mythos = Mythos::init(num);
	mythos.generate();

	mythos
}

fn main() {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.deserialize())
		.unwrap_or_else(|e| e.exit());

	if args.flag_version {
		println!("random-mythos-v{}", env!("CARGO_PKG_VERSION"));
		return;
	}

	set_logger(args.flag_verbose);

	info!("Random Mythos engage");
	io::stdout().flush().unwrap();

	let mythos: Mythos;
	if args.flag_import.is_some() {
		mythos = Mythos::from_json(read_file(&args.flag_import.unwrap()));

	} else {
		mythos = init_mythos();
	}

	if args.flag_export.is_some() {
		write_file(&mythos.to_json(), &args.flag_export.unwrap());
	}

	if args.flag_gen_dot {
		write_file(&dot::relations_to_dot(&mythos), "relations.dot");
	}

	let desc = description::get_descriptions(&mythos);
	write_file(&desc, &args.arg_file);

	println!("{}", desc);
}
