use std::io;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    println!("[PROJECT] Password Generator - This a Basic Rust Project to start from.");

    println!("");
    println!("[INFO] Commands:");
    println!("[INFO] - Type 'Exit' or 'Quit' to terminate program.");

    println!("");
    println!("[INFO] Type multiple key words and press ENTER to generate a password");

    let mut keywords: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();

        if keywords.len() <= 0{
            println!("[INPUT] Please enter your keywords: ");
            println!("");
        }
        else {
            println!("[INFO] Enter a keyword (or press ENTER to finish)");
            println!("");
        }

        io::stdin().read_line(&mut input).expect("[WARN] Failed to read line. Try again!");

        let input = input.trim();
        if input.is_empty() && keywords.len() <= 0 {
            println!("[INFO] No input detected and no keywords saved. Terminating process...");
            break;
        } 

        if input.to_lowercase().eq("exit") || input.to_lowercase().eq("quit") {
            println!("[INFO] Exiting Password Generator. Goodbye!");
            break;
        }

        if input.is_empty() {
            println!("[INFO] Finished receiving keywords.");
            break;
        }

        keywords.push(input.to_string());
    }

    if keywords.len () > 0 {
        println!("[INFO] Keywords received: {}", keywords.join(", "));
        println!("");

        let mut hashed_keywords: Vec<String> = Vec::new();
        let mut hasher = DefaultHasher::new();

        for keyword in &keywords {
            keyword.hash(&mut hasher);
            let hash_value = hasher.finish();

            hashed_keywords.push(format!("{:x}", hash_value));
        }

        let mut rng = thread_rng();
        hashed_keywords.shuffle(&mut rng);

        let mut password = String::new();

        for hashed_word in &hashed_keywords{
            password.push_str(&hashed_word[..4]);
        }

        println!("[INFO] Generating password...");
        println!("");

        if password.len() > 0 {
            println!("[INFO] Password generated Successfully!");
            println!("[RESULT] Your generated password is: {}", password);
        } else {
            println!("[WARN] Password Generation failed!");
        }
        
    } else {
        println!("[WARN] No keywords were provided. Password generation failed!");
    }
}
