pub mod day1;
pub mod day2;
mod day3;
pub mod read_file;


fn day0p1_dummy(){

}
fn day0p2_dummy(){

}

fn main() {
    let problems: Vec<(fn(), fn())> = vec![
        (day0p1_dummy, day0p2_dummy),
        (day1::p1, day1::p2),
        (day2::p1, day2::p2),
        (day3::p1, day3::p2),
    ];
    let day = 3;
    let problem = 1;

    match problem{
        0=> problems[day].0(),
        _ => problems[day].1(),
    }

}
