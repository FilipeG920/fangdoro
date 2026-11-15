use fangdoro::domain::{user_input_parse, PomodoroPlan};

fn main() {
    println!("Hello, world!");
    println!("Creating a pomodoro plan!");

    let test_plan = PomodoroPlan::new(user_input_parse("2:30"), user_input_parse("0:30"), -(5));
    let x = test_plan.unwrap();
    x.plan();
    user_input_parse("2:30");
}
