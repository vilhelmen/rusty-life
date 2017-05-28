extern crate rand;
use rand::Rng;
use std::io::{Write,Read};
// Need to figure out the correct scope whatever to get of these

struct Board {
	// row-major, idx = y_i * COL + x_i
	data: Vec<bool>,
	x: usize,
	y: usize,
}

impl Board {
	fn generate(x: usize, y: usize) -> Board {
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

	fn load(fname: &std::path::Path) -> Board {
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

		let mut file_data: Vec<bool> = Vec::with_capacity(tot);

		for line in data_lines {
			file_data.extend(line.chars().map(|ch| match ch {
				'#' => true,
				'-' => false,
				_ => panic!("Unrecognized symbol in file!"), // how to insert it into string?
			}));
		}

		Board{x: header_values[0], y: header_values[1], data:file_data}
	}
}

// allows easy printing of struct without just relying on debug stuff
impl std::fmt::Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		//let mut row_vec = Vec::<String>::new();
		
		// Ok. Slice through rows
		// Apply some sort of lambda to match true/false to characters
		// then join with space and pushback to row_vec

		// Rust currently doesn't have a (stable) range step operation.
		// Gross.
		// BUT THERE ARE CHUNKS WHOOOOOO

		// Probably not optimal, but god damn it took hours to get this working
		// I know there's got to be a monster function to do it in one step
		/*
		for row in self.data.chunks(self.x) {
			row_vec.push(
				row.iter()
				.map(
					|&v| match v {
						true => "#",
						false => "-",
					})
				.collect::<Vec<&str>>().join(""));
			// Vec<_> to make the compiler figure out the type
		}
		*/

		/*
		// I HAVE AN IDEA
		let i_am_the_greetest: String = self.data.chunks(self.x)
			.map(|obj| obj.iter()
				.map(|&v| match v {
					true => "#",
					false => "-",
				})
				.collect::<Vec<_>>().join(""))
			.collect::<Vec<_>>().join("\n");
			*/

		// surely I don't need collect.join and can use extend
		let i_am_the_greetest: String = self.data.chunks(self.x)
			.map(|row_data| row_data.iter()
				.map(|&value| match value {
					true => "#",
					false => "-",
				}))
			.fold(String::with_capacity((self.x + 1) * self.y), |mut acc, cur| {
				acc.push('\n');
				acc.extend(cur);
				acc
			});
		// HOLY SHIT THAT IS SO CLOSE BUT THERE'S AN EXTRA NEWLINE AT THE START
		// AND I CAN'T PRETEND THAT WAS ON PURPOSE
		// If it goes after extend then there will be a trailing newline.
		// HMMMMMMMM

		write!(f, "{},{}\n{}", self.x, self.y, i_am_the_greetest)

		// consider removing x,y since it's data duplication
		//write!(f, "{},{}\n{}", self.x, self.y, row_vec.join("\n"))
		//write!(f, "{}", row_vec.join("\n"))
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

		let board: Board;

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

					board = Board::generate(x_val, y_val);
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

				board = Board::load(std::path::Path::new((fname_str.trim())));
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
					unimplemented!();
				},
				"s" | "S" => {
					unimplemented!();
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
