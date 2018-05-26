// An attempt to make a procedural Mythos generator
#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate docopt;

use rand::Rng;
use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;
use docopt::Docopt;

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
	level: i32,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
enum RelationType {
	Base,
	Parent,
	Invoker,
	Creator,
}



#[derive(Deserialize, Serialize)]
pub struct Mythos {
	entites: Vec<Entity>,

	/* The relation between entities is modeled as a
	 * directed acyclic graph (dag), and stored in an
	 * adjacency matrix for simplicit.
	 */
	relations: Vec<Vec<Option<RelationType>>>,
}

impl Mythos {
	pub fn init(size: usize) -> Mythos {
		if size == 0 { panic!("Size must be grater then 0"); }

		let mut entites: Vec<Entity> = vec![];
		for i in 0..size {
			entites.push(Entity{
				name: format!("ent{:02}", i),
				level: 0,
			});
		}

		let relations: Vec<Vec<Option<RelationType>>> = vec![vec![None; size]; size];

		Mythos{ entites, relations }
	}

	pub fn from_json(json: String) -> Mythos {
		let r: Mythos = serde_json::from_str(&json).unwrap();

		r
	}

	fn add(&mut self, source: usize, destiny: usize, rt : RelationType) {
		self.relations[source][destiny] = Some(rt);
	}

	fn adjacent_out(&self, vertex: usize) -> Vec<usize> {
		let mut v: Vec<usize> = vec![];
		
		for j in 0..self.entites.len() {
			match &self.relations[vertex][j] {
				&Some(_) => v.push(j),
				&None => (),
			}
		}

		v
	}

	fn adjacent_in(&self, vertex: usize) -> Vec<usize> {
		let mut v: Vec<usize> = vec![];
		
		for i in 0..self.entites.len() {
			match &self.relations[i][vertex] {
				&Some(_) => v.push(i),
				&None => (),
			}
		}

		v
	}

	fn generate_base_relation(&mut self) {
		let s = self.entites.len();
		trace!("self.entites: {}", s);

		let n = s as usize;
		let mut n_gem = n;
		if n < 5 { n_gem -= 1; }
		trace!("num relations: {}", n);

		for _i in 0..n_gem {
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

			// It's not allowed to be a Philip J. Fry
			debug!("Verifying cycles");
			let mut stack: Vec<usize> = vec![];
			let mut verif = vec![false; n];
			stack.push(dest);
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

	fn generate_relations(&mut self) {
		let n = self.entites.len();

		info!("n relations: {}", n);

		for e in 0..n {
			info!("ent: {}", self.entites[e].name);
			let adj_in = self.adjacent_in(e);
			trace!("adj_in: {:?}", adj_in);

			let rt_n = rand::thread_rng().gen_range(0, 3);

			let rt: RelationType = match rt_n {
				0 => RelationType::Parent,
				1 => RelationType::Invoker,
				2 => RelationType::Creator,
				
				_ => panic!("Help"),
			};

			info!("rt: {:?}", rt);
			for src in adj_in.iter() {
				trace!("src {}", src);
				self.relations[*src][e] = Some(rt);
			}

			if !adj_in.is_empty() { self.fix_levels(e, rt); }
		}
	}

	fn fix_levels(&mut self, i: usize, rt: RelationType) {
		/* TODO
		 * - Melhorar menssagens de debug 
		 * - Verificar Creator
		 * - Verificar quando abs(delta) > 1
		 */

		info!("Fixing levels");

		trace!("Fixing level of {:?}", self.entites[i].name);
		let adj_in = self.adjacent_in(i);
		trace!("adj_in: {:?}", adj_in);
			
		let mut min: i32 = self.entites[adj_in[0]].level;
		for j in 1..adj_in.len() {
			let l = self.entites[j].level;
			if l < min { min = l; }
		}

		let inc = match rt {
			RelationType::Invoker => -1,
			RelationType::Creator => 1,
			_ => 0,
		};

		let new_level = min + inc;
		let delta = new_level - self.entites[i].level;

		trace!("rt: {:?}", rt);
		trace!("min: {}, inc: {}, l': {}, delta: {}", min, inc, new_level, delta);
		if delta == 0 { return; }

		self.entites[i].level = new_level;

		trace!("Propagate new value");
		let mut queue: VecDeque<usize> = VecDeque::new();
		for j in self.adjacent_out(i).iter() { queue.push_back(*j); }
		while !queue.is_empty() {
			let first = queue.pop_front().unwrap();
			trace!("first: {:?}", first);

			let nl = self.entites[first].level + inc;
			if nl == self.entites[first].level {
				trace!("Same level, clear queue");
				queue.clear();
			}

			trace!("Set new level to {}", nl);
			self.entites[first].level = nl;
			for out in self.adjacent_out(first).iter() {
				queue.push_back(*out);
			}
		}
	}

	// ========================================================================================

	fn to_json(&self) -> String {
		let j = serde_json::to_string_pretty(self).unwrap();

		j
	}

	fn generate(&mut self) {
		self.generate_base_relation();
		self.generate_relations();
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
