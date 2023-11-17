use std::{error::Error, env::args, fs::read_to_string};

/// 
/// Simple Grep tool
/// 
/// # Purpose
/// Search for terms inside of given text file.
/// 
fn main() -> Result<(), Box<dyn Error>> {

    // Get command line arguments from the environment
    let args: Vec<String> = args() // Get pointer to command line arguments
        .skip(1) // Skip path
        .collect(); // Collect to args vector

    // If 2 arguments were not provided via command line...
    if args.len() != 2 {
        return Err("Usage: minigrep-rs [search term] [file]".into()); // Display usage message
    } else {
        let search_term = args[0].as_str(); // Convert first argument to string slice
        let file = args[1].as_str(); // Convert second argument to string slice.

        // If file is not a text file.
        if !file.ends_with(".txt") {
            return Err("File must be a .txt file".into());
        } else {
            grep(search_term, file); // Search for search term in file.
        }

        Ok(())
    }
}

///
/// grep
/// 
/// # Purpose
/// 
///  Look for term inside a file
/// 
/// # Parameters
/// 
/// `search_term`: 
/// term being grepped for in `file`.
/// 
/// `file`:
/// file being grepped.
/// 
/// # Returns
/// 
/// Nothing
/// 
/// # Example
/// ```
/// grep("noodle", spaghetti_recipe.txt);
/// ```
/// 
fn grep(search_term: &str, file: &str) -> Result<(), std::io::Error> {
    // Read file contents into string.
    let contents = read_to_string(file)?;
    let mut found: bool = false;

    // Iterate over each line in the file, keep track of what line we're on.
    for (i, line) in contents.lines().enumerate() {
        // If line contains search term...
        if line.contains(search_term) {
            // Print line number and line.
            println!("{}: {}", i, line);
            found = true;
        }
    }

    // If search term was not found in file...
    if !found {
        println!("No results found for \"{}\"", search_term);
    }

    Ok(())
}