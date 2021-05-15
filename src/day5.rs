pub mod fifth_problem {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use crate::day4::fourth_problem;
    use fancy_regex::Regex;


    pub fn first_part() -> io::Result<()> {
        let mut file = File::open("./data/test.txt")?;
        let reader = BufReader::new(file);
        let re: Regex = Regex::new(r"(?=.*[aeiou].*[aeiou].*[aeiou])(?=.*([a-z])\1{1,})(?!.*(xy|pq|ab|cd).*)").unwrap();
        let mut count_of_nice_strings = 0;
        for line in reader.lines() {
            let i = line?;
            let mut is_valid = fourth_problem::is_hash_valid(i.to_string(), &re);
            if (is_valid) {
                println!("{}", i);
                count_of_nice_strings +=1;
            }
        }
        println!("Day 5 part 1 {}", count_of_nice_strings);
        Ok(())
    }

    pub fn second_part() -> io::Result<()> {
       
        Ok(())
    }

}