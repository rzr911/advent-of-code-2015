use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn main() {
    first_problem_first_part();
    first_problem_second_part();
    second_problem_first_part();
    second_problem_second_part();
    third_problem_first_part();
    third_problem_second_part();

}

fn first_problem_first_part() -> io::Result<()> {

    let mut file = File::open("./data/1.txt")?;
    let reader = BufReader::new(file);

    let mut floor: i32 = 0;
    
    for line in reader.lines() {
        let i = line?;
        let mut split : Vec<char> = i.chars().collect();
        
        for s in split {
            if s == '(' {
                floor += 1;
            }
            else if s == ')' {
                floor -= 1;
            }
        }
    }

    println!("Floor value {}", floor);
    Ok(())
}


fn first_problem_second_part() -> io::Result<()> {
    let mut file = File::open("./data/1.txt")?;
    let reader = BufReader::new(file);

    let mut floor: i32 = 0;
    let mut position_where_santa_enters_basement: u32 = 0;
    
    'outer: for line in reader.lines() {
        let i = line?;
        let mut split : Vec<char> = i.chars().collect();
        
        'inner: for s in split {
            if s == '(' {
                floor += 1;
            }
            else if s == ')' {
                floor -= 1;
            }

            position_where_santa_enters_basement += 1;

            if floor < 0 {
                break 'outer;
            }
        }
    }

    println!("Position where santa enters basement {}", position_where_santa_enters_basement);
    Ok(())
}



fn second_problem_first_part() -> io::Result<()> {
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

    println!("Total square feet of wrapping paper {}", total_square_feet_of_wrapping_paper);
    Ok(())
}


fn second_problem_second_part() -> io::Result<()> {
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

    println!("Total square feet of wrapping paper {}", total_ribbon_required);
    Ok(())
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Coordinates{
    x: i32,
    y: i32
}

fn third_problem_first_part() -> io::Result<()> {
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

    println!("Presents distributed {}", presents_distributed);
    Ok(())
}

fn third_problem_second_part() -> io::Result<()> {
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

    println!("Presents distributed {}", coordinates_visited.len());
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