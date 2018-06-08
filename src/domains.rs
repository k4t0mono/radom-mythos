// Domains for the entity

extern crate rand;
use rand::Rng;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum DomainType {
	Water,
	Earth,
	Fire,
	Air,
}

impl DomainType {
	fn next(&self) -> DomainType {
		return match self {
			&DomainType::Water => DomainType::Earth,
			&DomainType::Earth => DomainType::Fire,
			&DomainType::Fire => DomainType::Air,
			&DomainType::Air => DomainType::Water,
		}
	}

	fn prev(&self) -> DomainType {
		return match self {
			&DomainType::Water => DomainType::Air,
			&DomainType::Earth => DomainType::Water,
			&DomainType::Fire => DomainType::Earth,
			&DomainType::Air => DomainType::Fire,
		}
	}

	pub fn get_name(&self) -> String {
		let mut s = String::new();
		s += match self {
			&DomainType::Water => "Water",
			&DomainType::Earth => "Earth",
			&DomainType::Fire => "Fire",
			&DomainType::Air => "Air",
		};

		s
	}
}


#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Domain {
	values: HashMap<DomainType, u8>,
	primary: Option<DomainType>,
	secundary: Option<DomainType>,
}

impl Domain {
	pub fn new() -> Domain {
		let mut d = Domain {
			values: HashMap::new(),
			primary: None,
			secundary: None,
		};

		d.values.insert(DomainType::Water, 0);
		d.values.insert(DomainType::Earth, 0);
		d.values.insert(DomainType::Fire, 0);
		d.values.insert(DomainType::Air, 0);

		d
	}

	pub fn get(&self, dt: DomainType) -> &u8 { self.values.get(&dt).unwrap() }

	pub fn get_values(&self) -> Vec<(DomainType, u8)> {
		let mut list: Vec<(DomainType, u8)> = vec![];

		for (k, v) in &self.values { list.push((*k, *v)); }

		list
	}

	pub fn get_primary(&self) -> Option<(DomainType, u8)> {
		if self.primary.is_none() { return None; }

		let pri = self.primary.unwrap();
		Some(( pri, *self.values.get(&pri).unwrap() ))
	}

	pub fn get_secundary(&self) -> Option<(DomainType, u8)> {
		if self.secundary.is_none() { return None; }

		let pri = self.secundary.unwrap();
		Some(( pri, *self.values.get(&pri).unwrap() ))
	}

	pub fn gen_domain() -> Domain {
		debug!("gen_domain");

		let mut d = Domain::new();

		let pri_d = match rand::thread_rng().gen_range(0, 3) {
			0 => DomainType::Water,
			1 => DomainType::Earth,
			2 => DomainType::Fire,
			3 => DomainType::Air,
			_ => panic!(),
		};

		let pri_l = rand::thread_rng().gen_range(127, 255);
		d.primary = Some(pri_d);
		d.values.insert(pri_d, pri_l);

		let sec_dn = rand::thread_rng().gen_range(0, 1);
		let sec_d: DomainType;
		let ter_d: DomainType;
		if sec_dn == 0 {
			sec_d = pri_d.next();
			ter_d = pri_d.prev();

		} else {
			sec_d = pri_d.prev();
			ter_d = pri_d.next();
		}

		let sec_l = rand::thread_rng().gen_range(63, 127);
		d.secundary = Some(sec_d);
		d.values.insert(sec_d, sec_l);

		let ter_l = rand::thread_rng().gen_range(0, 63);
		d.values.insert(ter_d, ter_l);

		d
	}

	pub fn cross_over(d1: &Domain, d2: &Domain) -> Domain {
		debug!("Cross_over");
		let mut d_new = Domain::new();

		for (k, v) in d1.values.iter() {
			let n = (v & 0xf0) | (d2.values.get(&k).unwrap() & 0x0f);
			trace!("new {:?}: {:?}", k, n);

			d_new.values.insert(*k, n);
		}

		d_new.set_primary_secundary();

		d_new
	}

	pub fn cross_over_many(domains: &Vec<Domain>) -> Domain {
		debug!("Cross-over many");
		let mut new_d = domains[0].clone();

		for i in 1..domains.len() {
			new_d = Domain::cross_over(&new_d, &domains[i]);
		}

		new_d
	}

	fn find_primary_secundary(&self) -> (DomainType, DomainType) {
		let mut l: Vec<(u8, DomainType)> = vec![];

		for (k ,v) in &self.values { l.push((*v, *k)); }
		l.sort();

		(l.pop().unwrap().1, l.pop().unwrap().1)
	}

	fn set_primary_secundary(&mut self) {
		let p = self.find_primary_secundary();
		self.primary = Some(p.0);
		self.secundary = Some(p.1);
	}

	pub fn mutate(&mut self) -> Domain {
		debug!("Mutate");

		let mut new_d = Domain::new();
		for (k, v) in &self.values {
			trace!("mutating {:?}", k);
			let mut new_v = *v;

			let spot0 = rand::thread_rng().gen_range(0, 4);
			let spot1 = rand::thread_rng().gen_range(4, 8);
			trace!("spots: ({}, {})", spot0, spot1);

			if rand::thread_rng().gen_bool(0.2) {
				trace!("mutate spot0");

				let spot0_v: u8 = 2_u8.pow(spot0);
				if (*v & spot0_v) == spot0_v {
					new_v &= 255 - spot0_v;
				} else {
					new_v += spot0_v;
				}
			}

			if rand::thread_rng().gen_bool(0.1) {
				trace!("mutate spot1");

				let spot1_v: u8 = 2_u8.pow(spot1);
				if (*v & spot1_v) == spot1_v {
					new_v &= 255 - spot1_v;
				} else {
					new_v += spot1_v;;
				}
			}

			new_d.values.insert(*k, new_v);
		}

		new_d.set_primary_secundary();

		new_d
	}

	pub fn gen_from_average(list: Vec<Domain>) -> Domain {
		info!("gen_from_average");

		let mut new_d = Domain::new();

		let n = list.len();
		debug!("n: {}", n);

		let mut values: HashMap<DomainType, usize> = HashMap::new();
		for i in 0..n {
			for (k, v) in &list[i].values {
				let v0 = values.entry(*k).or_insert(0);
				*v0 += *v as usize;
			}
		}

		if n == 1 { println!("aaaa: {:?}", list); }
		for (k, v) in &values { 
			new_d.values.insert(*k, (v/n) as u8);
			trace!("new value: ({:?} , {:?})", *k, new_d.values.get(k).unwrap());
		}
		new_d.set_primary_secundary();

		new_d
	}
}
