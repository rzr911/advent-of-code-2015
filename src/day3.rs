pub mod third_problem {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::collections::HashSet;

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct Coordinates{
        x: i32,
        y: i32
    }

    pub fn first_part() -> io::Result<()> {
        let mut file = File::open("./data/3.txt")?;
        let reader = BufReader::new(file);
        let mut coordinates_visited : HashSet<Coordinates> = HashSet::new();
        let mut current_coordinate = Coordinates{x: 0, y: 0};
        let mut presents_distributed: u32 = 1; // starts with 1 considering the starting location
        
        coordinates_visited.insert(current_coordinate.clone());
        
        for line in reader.lines() {
            let i = line?;
            let mut moves : Vec<char> = i.chars().collect();

            for m in moves {
                if m == '>' {
                    current_coordinate.x +=1;
                } else if m == '<' {
                    current_coordinate.x -=1;
                } else if m == '^' {
                    current_coordinate.y +=1;
                } else if m == 'v' {
                    current_coordinate.y -=1;
                }

                if !coordinates_visited.contains(&current_coordinate.clone()) {
                    presents_distributed += 1;
                    coordinates_visited.insert(current_coordinate.clone());
                }
            }
        }

        println!("Day 3 part 1 {}", presents_distributed);
        Ok(())
    }

    pub fn second_part() -> io::Result<()> {
        let mut file = File::open("./data/3.txt")?;
        let reader = BufReader::new(file);
        let mut coordinates_visited : HashSet<Coordinates> = HashSet::new();
        let mut coordinates_visited_by_robo_santa : HashSet<Coordinates> = HashSet::new();
        let mut current_coordinate_of_santa = Coordinates{x: 0, y: 0};
        let mut current_coordinate_of_robo_santa = Coordinates{x: 0, y: 0};

        let mut presents_distributed: u32 = 1; // starts with 1 considering the starting location of santa and robo santa
        
        coordinates_visited.insert(current_coordinate_of_santa.clone());
        
        let mut turn : u32 = 0;

        for line in reader.lines() {
            let i = line?;
            let mut moves : Vec<char> = i.chars().collect();

            for m in moves {
                if turn % 2 == 0 {
                    current_coordinate_of_santa = move_coordinate(current_coordinate_of_santa, m);
                    
                    if !coordinates_visited.contains(&current_coordinate_of_santa.clone()) {
                        presents_distributed += 1;
                        coordinates_visited.insert(current_coordinate_of_santa.clone());
                    }
                } else {
                    current_coordinate_of_robo_santa = move_coordinate(current_coordinate_of_robo_santa, m);
                    
                    if !coordinates_visited.contains(&current_coordinate_of_robo_santa.clone()) {
                        presents_distributed += 1;
                        coordinates_visited.insert(current_coordinate_of_robo_santa.clone());
                    }
                }

                turn +=1;
                
            }
        }

        println!("Day 3 part 3 {}", coordinates_visited.len());
        Ok(())
    }

    fn move_coordinate(mut current_coordinate: Coordinates, move_value: char) -> Coordinates {
        if move_value == '>' {
            current_coordinate.x +=1;
        } else if move_value == '<' {
            current_coordinate.x -=1;
        } else if move_value == '^' {
            current_coordinate.y +=1;
        } else if move_value == 'v' {
            current_coordinate.y -=1;
        }

        return current_coordinate;
    }
}