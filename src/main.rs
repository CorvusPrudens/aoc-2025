use std::path::PathBuf;

/// Solutions for Advent of Code 2025!
#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[cfg_attr(feature = "cli", command(version, about))]
struct Args {
    /// The advent of code day.
    day: usize,
    /// The path to the advent of code input data, if any.
    input: Option<PathBuf>,
    /// Place the output into the clipboard.
    #[cfg_attr(feature = "cli", arg(long, short))]
    clipboard: bool,
}

impl Args {
    /// Get the input, assuming it's present.
    ///
    /// # Panics
    ///
    /// Panics if no file is provided.
    fn get_input(&self) -> String {
        let path = self.input.as_ref().expect("expected path to input file");
        std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("Failed to read file `{path:?}`: {e}");
        })
    }
}

fn main() {
    #[cfg(feature = "cli")]
    {
        use clap::Parser;
        let args = Args::parse();

        let result = match args.day {
            0 => panic!("`0` isn't a valid day, silly!"),
            1 => {
                let input = args.get_input();
                aoc_2025::days::one::part_two(&input).to_string()
                // aoc_2025::days::one::part_one(&input).to_string()
            }
            2 => {
                let input = args.get_input();
                // aoc_2025::days::two::part_one(&input).to_string()
                aoc_2025::days::two::part_two(&input).to_string()
            }
            3 => {
                let input = args.get_input();
                // aoc_2025::days::three::part_one(&input).to_string()
                aoc_2025::days::three::part_two(&input).to_string()
            }
            4 => {
                let input = args.get_input();
                // aoc_2025::days::four::part_one(&input).to_string()
                aoc_2025::days::four::part_two(&input).to_string()
            }
            5 => {
                let input = args.get_input();
                // aoc_2025::days::five::part_one(&input).to_string()
                aoc_2025::days::five::part_two(&input).to_string()
            }
            n if n < 12 => panic!("`{n}` does not yet have a solution :/"),
            n => panic!("There aren't even `{n}` days this year, silly!"),
        };

        if args.clipboard {
            let mut clipboard = arboard::Clipboard::new().unwrap();
            clipboard.set_text(&result).unwrap();
        } else {
            println!("{result}");
        }
    }
}
