// testing

#[cfg(test)]
mod tests {
	use relations::RelationType;
	use Mythos;

	#[test]
	#[should_panic]
	fn mythos_init_invalid_size_should_panic() {
		let _r = Mythos::init(0);
	}

	#[test]
	fn mythos_init_valid_size_should_create() {
		let r = Mythos::init(10);

		assert_eq!(r.entites.len(), 10);
	}

	#[test]
	fn mythos_should_not_have_base() {
		let mut rs = Mythos::init(10);
		rs.generate();

		let mut has_base = false;
		for i in 0..10 {
			for j in 0..10 {
				let r = rs.relations.get_relation(i, j);

				if r.is_some() && r.unwrap() == RelationType::Base {
					has_base = true;
				}
			}
		}

		assert_eq!(has_base, false);
	}

	#[test]
	fn mythos_should_be_same_type() {
		let mut rs = Mythos::init(10);
		rs.generate();

		let mut same_type = true;
		for i in 0..10 {
			let adj_in = rs.relations.get_adj_in(i);
			if adj_in.len() == 0 { continue; }

			let t = rs.relations.get_relation(adj_in[0], i).unwrap();
			for j in adj_in.iter() {
				if rs.relations.get_relation(*j, i).unwrap() != t {
					same_type = false;
				}
			}
		}

		assert_eq!(same_type, true);
	}
	
	#[test]
	fn should_have_correct_levels() {
		let mut rs = Mythos::init(42);
		rs.generate();

		let mut same = true;
		for e in 0..42 {
			let adj_in = rs.relations.get_adj_in(e);
			if adj_in.is_empty() { continue; }

			let rt = rs.relations.get_relation(adj_in[0], e).unwrap();
			let mut min = rs.entites[adj_in[0]].level;
			for a in adj_in.iter() {
				let l = rs.entites[*a].level;
				if l < min { min = l; }
			}

			let inc = match rt {
				RelationType::Creator => 1,
				RelationType::Invoker => -1,
				_ => 0,
			};
			
			if rs.entites[e].level != min+inc { same = false; }
		}

		assert_eq!(same, true);
	}

}
