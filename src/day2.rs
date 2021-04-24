pub mod second_problem {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};

    pub fn first_part() -> io::Result<()> {
        let mut file = File::open("./data/2.txt")?;
        let reader = BufReader::new(file);
    
        let mut total_square_feet_of_wrapping_paper: u32 = 0;
        
        for line in reader.lines() {
            let i = line?;
            let dimensions_in_string = i.split("x").collect::<Vec<&str>>();
            let length = dimensions_in_string[0].parse::<u32>().unwrap();
            let width = dimensions_in_string[1].parse::<u32>().unwrap();
            let height = dimensions_in_string[2].parse::<u32>().unwrap();
            let area1 =  length * width;
            let area2 = width * height;
            let area3 = height * length;
            let mut slack:u32 = 0;
    
            if area1 <= area2 && area1 <=area3 {
                slack = area1;
            } else if area2 <= area3 {
                slack = area2;
            } else {
                slack = area3;
            }
    
            total_square_feet_of_wrapping_paper += (2 * area1) + (2 * area2) + (2 * area3) + slack;
        }
    
        println!("Day 2 part 1 {}", total_square_feet_of_wrapping_paper);
        Ok(())
    }
    
    
    pub fn second_part() -> io::Result<()> {
        let mut file = File::open("./data/2.txt")?;
        let reader = BufReader::new(file);
    
        let mut total_ribbon_required: u32 = 0;
        
        for line in reader.lines() {
            let i = line?;
            let dimensions_in_string: Vec<u32> = i.split("x").map(|s| s.parse().unwrap()).collect();
            let length = dimensions_in_string[0];
            let width = dimensions_in_string[1];
            let height = dimensions_in_string[2];
    
            let perimeter1 =  2 * length  + 2 * width;
            let perimeter2 = 2 * width + 2 * height;
            let perimeter3 = 2 * height + 2 * length;
            let mut smallest_perimeter: u32 = 0;
            let mut ribbon_length_required:u32 = 0;
    
            if perimeter1 <= perimeter2 && perimeter1 <= perimeter3 {
                smallest_perimeter = perimeter1;
            } else if perimeter2 <= perimeter3 {
                smallest_perimeter = perimeter2;
            } else {
                smallest_perimeter = perimeter3;
            }
    
            total_ribbon_required += smallest_perimeter + length * width * height;
        }
    
        println!("Day 2 part 2 {}", total_ribbon_required);
        Ok(())
    }

}