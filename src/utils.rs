// Utilitarian functions

use std::fs::OpenOptions;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn write_file<'a>(data: &'a str, path: &'a str) {
	let f = OpenOptions::new()
			.write(true)
			.create(true)
			.open(path);

	let mut file = match f {
		Err(e) => {
			error!("Something is terrible wrong happend while oppening the file");
			error!("{}", e);

			panic!(e)
		},

		Ok(fl) => fl,
	};
	
	match file.write_all(data.as_bytes()) {
		Err(e) => {
			error!("Something is terrible wrong happend while writing the file");
			error!("{}", e);

			panic!(e)
		},

		Ok(_) => info!("File {} writed sucessfully", path),
	}
}
