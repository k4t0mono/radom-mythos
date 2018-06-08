// Plains of existence

use domains::*;

#[derive(Debug)]
pub struct Plain {
	name: String,
	domain: Domain,
	created: bool,
}

impl Plain {
	pub fn new(name: String, created: bool) -> Plain {
		Plain {
			name,
			created,
			domain: Domain::gen_domain(),
		}
	}
}
