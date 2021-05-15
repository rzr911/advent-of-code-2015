pub mod sixth_problem {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use fancy_regex::Regex;

    pub fn first_part() -> io::Result<()> {
        let mut file = File::open("./data/6.txt")?;
        let reader = BufReader::new(file);
        let mut grid = vec![vec![0 as i32; 1000]; 1000];
        let re: Regex = Regex::new(r"^(.*?)[ ](\d+),(\d+) through (\d+),(\d+)").unwrap();
        
        for line in reader.lines() {
            let i = line?;
            let captures = re.captures(&i);

        let wrapped_captures = &captures.unwrap();
            let action = wrapped_captures.as_ref().unwrap().get(1).unwrap().as_str();
            let start_x_coordinate = wrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().parse::<i32>().unwrap();
            let start_y_coordinate = wrapped_captures.as_ref().unwrap().get(3).unwrap().as_str().parse::<i32>().unwrap();
            let end_x_coordinate = wrapped_captures.as_ref().unwrap().get(4).unwrap().as_str().parse::<i32>().unwrap();
            let end_y_coordinate = wrapped_captures.as_ref().unwrap().get(5).unwrap().as_str().parse::<i32>().unwrap();

            light_up_part1(&mut grid, action, start_x_coordinate, start_y_coordinate, end_x_coordinate + 1, end_y_coordinate + 1);
        }
        
        let mut no_of_lights = 0;

        for i in 0..1000 {
            for j in 0..1000 {
                if grid[i][j] == 1 {
                    no_of_lights += 1;
                }
            }
        }
        
        println!("No of lights {}", no_of_lights);
        Ok(())
    }

    pub fn light_up_part1(grid: &mut Vec<Vec<i32>>, action: &str, start_x:i32, start_y:i32, end_x:i32, end_y:i32) {
        for i in start_x as usize..end_x as usize {
            for j in start_y as usize..end_y as usize {
                if action == "turn on" {
                    grid[i][j] = 1;
                } else if action == "turn off" {
                    grid[i][j] = 0;
                } else if action == "toggle" {
                    if grid[i][j] == 1 {
                        grid[i][j] = 0;
                    } else {
                        grid[i][j] = 1;
                    }
                }
            }
        }
    }

    pub fn second_part() -> io::Result<()> {
        let mut file = File::open("./data/6.txt")?;
        let reader = BufReader::new(file);
        let mut grid = vec![vec![0 as i32; 1000]; 1000];
        let re: Regex = Regex::new(r"^(.*?)[ ](\d+),(\d+) through (\d+),(\d+)").unwrap();
        
        for line in reader.lines() {
            let i = line?;
            let captures = re.captures(&i);

        let wrapped_captures = &captures.unwrap();
            let action = wrapped_captures.as_ref().unwrap().get(1).unwrap().as_str();
            let start_x_coordinate = wrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().parse::<i32>().unwrap();
            let start_y_coordinate = wrapped_captures.as_ref().unwrap().get(3).unwrap().as_str().parse::<i32>().unwrap();
            let end_x_coordinate = wrapped_captures.as_ref().unwrap().get(4).unwrap().as_str().parse::<i32>().unwrap();
            let end_y_coordinate = wrapped_captures.as_ref().unwrap().get(5).unwrap().as_str().parse::<i32>().unwrap();

            light_up_part2(&mut grid, action, start_x_coordinate, start_y_coordinate, end_x_coordinate + 1, end_y_coordinate + 1);
        }
        
        let mut no_of_lights = 0;

        for i in 0..1000 {
            for j in 0..1000 {
                no_of_lights += grid[i][j];
            }
        }
        
        println!("No of lights {}", no_of_lights);
        Ok(())
    }

    pub fn light_up_part2(grid: &mut Vec<Vec<i32>>, action: &str, start_x:i32, start_y:i32, end_x:i32, end_y:i32) {
        for i in start_x as usize..end_x as usize {
            for j in start_y as usize..end_y as usize {
                if action == "turn on" {
                    grid[i][j] += 1;
                } else if action == "turn off" {
                    if grid[i][j] != 0 {
                        grid[i][j] -= 1;
                    }
                } else if action == "toggle" {
                    grid[i][j] += 2;
                }
            }
        }
    }

}