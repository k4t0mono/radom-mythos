// generate relations

extern crate rand;

use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum RelationType {
	Base,
	Parent,
	Invoker,
	Creator,
}

#[derive(Deserialize, Serialize)]
pub struct Relations {
	size: usize,

	/* The relation between entities is modeled as a
	 * directed acyclic graph (dag), and stored in an
	 * adjacency matrix for simplicit.
	 */
	data: Vec<Vec<Option<RelationType>>>,
}

impl Relations {
	pub fn new(size: usize) -> Relations {
		let data = vec![vec![None; size]; size];

		Relations{ size, data }
	}

	pub fn generate(&mut self) {
		self.generate_base_relation();
		self.generate_relations();
	}

	pub fn get(&self, i: usize, j: usize) -> Option<RelationType> {
		self.data[i][j]
	}

	pub fn get_adj_in(&self, vertex: usize) -> Vec<usize> {
		self.adjacent_in(vertex)
	}

	pub fn get_topological_sort(&self) -> Vec<usize> {
		self.topological_sort()
	}

	pub fn get_roots(&self) -> Vec<usize> {
		let mut roots: Vec<usize> = vec![];

		for i in 0..self.size {
			if self.adjacent_in(i).is_empty() { roots.push(i); }
		}

		roots
	}

	// =================================================================

	fn adjacent_out(&self, vertex: usize) -> Vec<usize> {
		let mut v: Vec<usize> = vec![];

		for j in 0..self.size {
			match &self.data[vertex][j] {
				&Some(_) => v.push(j),
				&None => (),
			}
		}

		v
	}

	fn adjacent_in(&self, vertex: usize) -> Vec<usize> {
		let mut v: Vec<usize> = vec![];

		for i in 0..self.size {
			match &self.data[i][vertex] {
				&Some(_) => v.push(i),
				&None => (),
			}
		}

		v
	}

	fn topological_sort(&self) -> Vec<usize> {
		let mut list: Vec<usize> = vec![];

		let n = self.size;
		let mut visited: Vec<bool> = vec![false; n];
		for i in 0..n {
			if visited[i] { continue; }
			self.topological_sort_visit(i, &mut visited, &mut list);
		}

		let mut sorted: Vec<usize> = vec![];
		while let Some(top) = list.pop() { sorted.push(top); }

		sorted
	}

	fn topological_sort_visit(&self, n: usize, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
		if visited[n] { return; }

		for i in self.adjacent_out(n).iter() {
			self.topological_sort_visit(*i, visited, stack);
		}

		visited[n] = true;
		stack.push(n);
	}

	// ========================================================================

	fn generate_base_relation(&mut self) {
		let s = self.size;
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

			while let &Some(ref _rt) = &self.data[src][dest] {
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
			self.data[src][dest] = Some(RelationType::Base);
		}
	}

	fn generate_relations(&mut self) {
		let n = self.size;
		info!("n relations: {}", n);

		for e in 0..n {
			info!("ent: {}", e);
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
				self.data[*src][e] = Some(rt);
			}
		}
	}

}
