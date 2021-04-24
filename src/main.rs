mod day1;
mod day2;
mod day3;

pub use crate::day1::first_problem;
pub use crate::day2::second_problem;
pub use crate::day3::third_problem;

fn main() {
    first_problem::first_part();
    first_problem::second_part();
    second_problem::first_part();
    second_problem::second_part();
    third_problem::first_part();
    third_problem::second_part();
}
