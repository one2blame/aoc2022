mod one;

#[cfg(test)]
mod tests {
    use crate::one::solve::{first_puzzle, second_puzzle};
    use std::env;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    const CARGO_MANIFEST_DIR_ENV_VAR: &str = "CARGO_MANIFEST_DIR";
    const PUZZLES_DIR: &str = "puzzles";

    /// Returns the contents of a puzzle file
    ///
    /// # Parameters
    ///
    /// - `puzzle_filename`: A string slice that contains the name of the puzzle file
    ///
    /// # Returns
    ///
    /// A `String` of the contents in the puzzle file specified by "puzzle_filename"
    fn get_puzzle_contents(puzzle_filename: &str) -> String {
        // Get directory of the library crate
        let current_directory = env::var(CARGO_MANIFEST_DIR_ENV_VAR).unwrap_or_else(|error| {
            panic!(
                "Error: {}, Are you sure you're running 'cargo test'?",
                error
            )
        });

        // Create absolute file path for the puzzle file
        let mut puzzle_filepath = PathBuf::from(current_directory);
        puzzle_filepath.push(PUZZLES_DIR);
        puzzle_filepath.push(puzzle_filename);

        // Read all contents of the puzzle file
        let puzzle_contents = match read_to_string(puzzle_filepath) {
            Ok(puzzle_contents) => puzzle_contents,
            Err(e) => {
                panic!("Error reading {}: {}", puzzle_filename, e);
            }
        };

        return puzzle_contents;
    }

    /// Test for Day 1 of AoC 2022
    #[test]
    fn one() {
        let puzzle_contents = get_puzzle_contents("one_puzzle.txt");
        assert_eq!(first_puzzle(puzzle_contents), 75622);

        let puzzle_contents = get_puzzle_contents("one_puzzle.txt");
        assert_eq!(second_puzzle(puzzle_contents), 213159);
    }
}
