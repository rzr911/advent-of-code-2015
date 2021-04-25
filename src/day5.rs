pub mod fifth_problem {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use crate::day4::fourth_problem;

    pub fn first_part() -> io::Result<()> {
        let mut file = File::open("./data/5.txt")?;
        let reader = BufReader::new(file);
        let re: Regex = Regex::new(r"(?=.*[aeiou].*[aeiou].*[aeiou])").unwrap();
        
        for line in reader.lines() {
            let i = line?;
            println!("{}", i);
            if ()
        }
        // println!("Day 3 part 1 {}", presents_distributed);
        Ok(())
    }

    pub fn second_part() -> io::Result<()> {
       
        Ok(())
    }

}