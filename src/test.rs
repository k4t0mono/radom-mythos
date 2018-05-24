// testing

#[cfg(test)]
mod tests {
	use RelationType;
	use Relations;

	#[test]
	#[should_panic]
	fn relations_init_invalid_size_should_panic() {
		let _r = Relations::init(0);
	}

	#[test]
	fn relations_init_valid_size_should_create() {
		let r = Relations::init(10);

		assert_eq!(r.entites.len(), 10);
	}

	#[test]
	fn relations_should_not_have_base() {
		let rs = Relations::init(10);

		let mut has_base = false;
		for i in 0..10 {
			for j in 0..10 {
				let r = rs.relations[i][j];

				if r.is_some() && r.unwrap() == RelationType::Base {
					has_base = true;
				}
			}
		}

		assert_eq!(has_base, false);
	}

	#[test]
	fn relations_should_be_same_type() {
		let rs = Relations::init(10);

		let mut same_type = true;
		for i in 0..10 {
			let adj_in = rs.adjacent_in(i);
			if adj_in.len() == 0 { continue; }

			let t = rs.relations[adj_in[0]][i].unwrap();
			for j in 1..adj_in.len() {
				if rs.relations[j][i].unwrap() != t {
					same_type = false;
				}
			}
		}

		assert_eq!(same_type, true);
	}


}
