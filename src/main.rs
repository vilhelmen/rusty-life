use std::io::Write;

mod life {

	extern crate std;
	extern crate rand;
	use self::rand::Rng; // ???
	use std::io::{Write,Read};
	use std::error::Error;

	pub struct Board {
		// row-major, idx = y_i * COL + x_i
		data: Vec<bool>,
		x: usize,
		y: usize,
	}

	impl Board {
		pub fn generate(x: usize, y: usize) -> Board {
			if x == 0 || y == 0 {
				panic!("Generator dim is zero!");
			}
			// That sure is a command
			// Also I can't figure out how to get rid of the rand:Rng use statement
			// I'd really rather learn full names.
			Board{x: x, y: y, data:
				rand::thread_rng().gen_iter::<bool>().take(x * y).collect::<Vec<bool>>()
			}
		}

		// This should really return a result thing. But then again, so should load
		pub fn save(&self, fname: &std::path::Path) {
			let mut file = match std::fs::File::create(&fname) {
				Ok(file) => file,
				Err(why) => panic!("Couldn't create file \"{}\" because {}", fname.display(), why.description()),
			};

			// fmt::Display != .display it seems
			// but this works, I guess.
			match file.write_all(format!("{}",self).as_bytes()) {
				Err(why) => panic!("Writing to \"{}\" ate it because {}", fname.display(), why.description()),
				Ok(_) => println!("Saved to \"{}\"", fname.display()),
			};
		}

		// THIS IS A HOUSE OF CARDS DOOMED TO FAIL DO NOT LOOK DIRECTLY AT IT
		pub fn load(fname: &std::path::Path) -> Board {
			let mut data = String::new();

			{
				let mut file = std::fs::File::open(fname)
					.expect("FILE OPEN FAILED >:C");

				file.read_to_string(&mut data)
					.expect("FILE LOAD DIED >:C");
			}

			let mut data_lines = data.split('\n');

			// it's gross, but it works to get the x and y values.
			// It'll explode in new and terrifying ways if it doesn't parse right.

			let header_values = data_lines.next()
										.unwrap()
										.split(',')
										.map(|val_str| val_str.parse::<usize>().unwrap())
										.collect::<Vec<usize>>();

			let tot = header_values[0] * header_values[1];

			if tot == 0 {
				panic!("Bad dimensions!");
			}

			let mut file_data: Vec<bool> = Vec::with_capacity(tot);

			for line in data_lines {
				file_data.extend(line.chars().map(|ch| match ch {
					'#' => true,
					'-' => false,
					_ => panic!("Unrecognized symbol \"{}\" in file!", ch),
				}));
			}

			if file_data.len() != tot {
				panic!("Expected {} but got {} from file", tot, file_data.len());
			}

			Board{x: header_values[0], y: header_values[1], data:file_data}
		}

		fn coord_map(&self, x_i: usize, y_i: usize) -> Result<usize,&'static str> {
			// we never rollover/under by more than one
			// expect weirdness if the x or y dim is usize's max
			// Also that means you're using like all you memory and then some
			// So this is your fault, really, not mine.
			if x_i < self.x && y_i < self.y {
				Ok((y_i * self.y) + x_i)
			} else {
				Err("No")
			}
		}

		pub fn iterate(&self) {
			let window_offsets: Vec<(isize,isize)> = vec!(
				(-1,-1), ( 0,-1), ( 1,-1),
				(-1, 0),          ( 1, 0),
				(-1, 1), ( 0, 1), ( 1, 1));
			let mut next = self.data.clone();

			// WHY IS THIS SO DIFFUCULT.
			// you can't add negative one to an unsigned integer
			// You have to subtract one
			// How the hell do you use that with offsets like this

			for x_i in 0..self.x {
				for y_i in 0..self.y {
					// next[self.coord_map(x_i,y_i)] = 
					println!("{:?}", window_offsets.iter().map(|&(dx, dy)| x_i.wrapping_add(dx)).collect::<Vec<_>>());
					/*
					println!("{:?}",window_offsets.iter().map(|(d_x, d_y)| 
							match self.coord_map(x_i.wrapping_add(d_x), y_i.wrapping_add(d_y)) {
								Ok(coord) => match self.data[coord] {
									true => 1,
									false => 0,
								},
								Err(_) => 0,
					}));
					*/
				}
			}
			unimplemented!();
		}
	}

	// allows easy printing of struct without just relying on debug stuff
	impl std::fmt::Display for Board {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

			let mut board_view: String = self.data.chunks(self.x)
				.map(|row_data| row_data.iter()
					.map(|&value| match value {
						true => "#",
						false => "-",
					}))
				.fold(String::with_capacity((self.x + 1) * self.y), |mut acc, cur| {
					acc.extend(cur);
					acc.push('\n');
					acc
				});

			// THE SHAME
			board_view.pop();

			// consider removing x,y since it's data duplication
			// But it makes parsling slightly easier
			write!(f, "{},{}\n{}", self.x, self.y, board_view)
		}
	}
}


fn main_menu() {
	'main_menu: loop {
		print!("(G)enerate, (L)oad, (E)xit: ");
		std::io::stdout().flush()
			.expect("FLUSH ATE IT >:C");

		let mut sel = String::new();
		std::io::stdin().read_line(&mut sel)
			.expect("READLINE ATE IT >:C");

		//sel.pop(); // remove newline, etc

		let mut board: life::Board;

		match sel.trim() {
			"g" | "G" => {
				'gen: loop {

					let mut x_str = String::new();
					
					print!("X dim: ");
					std::io::stdout().flush()
						.expect("FLUSH ATE IT >:C");
					std::io::stdin().read_line(&mut x_str)
						.expect("READLINE ATE IT >:C");

					let x_trim = x_str.trim();

					let x_val = match x_trim.parse::<usize>() {
						Ok(x_val) => x_val,
						Err(_) => {
							println!("Could not parse \"{}\"", x_trim);
							continue;
						},
					};


					let mut y_str = String::new();
					
					print!("Y dim: ");
					std::io::stdout().flush()
						.expect("FLUSH ATE IT >:C");
					std::io::stdin().read_line(&mut y_str)
						.expect("READLINE ATE IT >:C");

					let y_trim = y_str.trim();

					let y_val = match y_trim.parse::<usize>() {
						Ok(y_val) => y_val,
						Err(_) => {
							println!("Could not parse \"{}\"", y_trim);
							continue;
						},
					};

					board = life::Board::generate(x_val, y_val);
					break;
				}
			},
			"l" | "L" => {
				let mut fname_str = String::new();
				print!("File: ");
				std::io::stdout().flush()
					.expect("FLUSH ATE IT >:C");
				std::io::stdin().read_line(&mut fname_str)
					.expect("READLINE FUKKIN ATE IT >:C");

				board = life::Board::load(std::path::Path::new(fname_str.trim()));
			},
			"e" | "E" => {
				return;
			},
			val @ _ => {
				println!("\"{}\" unrecognized.", val);
				continue;
			}
		};

		'op_menu: loop {

			println!("{}",board);

			print!("(I)terate, (S)ave, (B)ack: ");
			std::io::stdout().flush()
				.expect("FLUSH ATE IT >:C");

			// UGH SHADOWING I KNOW
			let mut sel = String::new();

			std::io::stdin().read_line(&mut sel)
				.expect("READLINE FUKKIN ATE IT >:C");

			match sel.trim() {
				"i" | "I" => {
					board.iterate();
				},
				"s" | "S" => {
					let mut fname_str = String::new();
					print!("File: ");
					std::io::stdout().flush()
						.expect("FLUSH ATE IT >:C");
					std::io::stdin().read_line(&mut fname_str)
						.expect("READLINE FUKKIN ATE IT >:C");

					board.save(std::path::Path::new(fname_str.trim()));
				},
				"b" | "B" => {
					break;
				},
				val @ _ => {
					println!("\"{}\" unrecognized.", val);
					continue;
				}
			};
		}
		// back to main menu!
	}
}

fn main() {
    main_menu()
}
