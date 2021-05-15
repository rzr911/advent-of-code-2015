pub mod fourth_problem {
    use md5;
    use std::io::{self, prelude::*, BufReader};
    use fancy_regex::Regex;

    use rayon::prelude::*;
    
    pub fn first_part() -> io::Result<()> {
        
        let mut lowest_number: u64 = 1;
        let secret_key = "iwrupvqb";
        let re: Regex = Regex::new(r"^[0]{5}").unwrap();
        
        let found_string_index = (0..1e6 as u32).into_par_iter().find_first(|i| {
            let hash_input = secret_key.to_owned() + &i.to_string();
            let hash_digest = md5::compute(hash_input);
            let hash_string = format!("{:x}", hash_digest);
            return is_hash_valid(hash_string, &re);
        });

        println!("Day 4 Part 1 {:?}", found_string_index);
        
        Ok(())
    }

    pub fn second_part() -> io::Result<()> {
        let mut lowest_number: u64 = 1;
        let secret_key = "iwrupvqb";
        let re: Regex = Regex::new(r"^[0]{6}").unwrap();

        let found_string_index = (0..1e12 as u32).into_par_iter().find_first(|i| {
            let hash_input = secret_key.to_owned() + &i.to_string();
            let hash_digest = md5::compute(hash_input);
            let hash_string = format!("{:x}", hash_digest);
            return is_hash_valid(hash_string, &re);
        });

        println!("Day 4 part 2 {:?}", found_string_index);
        Ok(())
    }

    pub fn is_hash_valid(hash: String, valid_regex: &Regex) -> bool{
        return valid_regex.is_match(&hash).unwrap();
    }
}


