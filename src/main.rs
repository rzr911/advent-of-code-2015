mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub use crate::day1::first_problem;
pub use crate::day2::second_problem;
pub use crate::day3::third_problem;
pub use crate::day4::fourth_problem;
pub use crate::day5::fifth_problem;


fn main() {
    // use std::time::Instant;
    // let now = Instant::now();
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:?}", elapsed);

    // first_problem::first_part();
    // first_problem::second_part();
    // second_problem::first_part();
    // second_problem::second_part();
    // third_problem::first_part();
    // third_problem::second_part();
    // fourth_problem::first_part();
    // fourth_problem::second_part();
    fifth_problem::first_part();
    fifth_problem::second_part();
}
