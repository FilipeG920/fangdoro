use fangdoro::domain::{PomodoroPlan, user_input_parse};

fn main() {
    println!("Hello, world!");
    println!("Creating a pomodoro plan!");

    let test_plan = PomodoroPlan::new(user_input_parse("3:25"), user_input_parse("1:37"), 3);
    // let plan1: PomodoroPlan = test_plan.unwrap();
    // if let Ok(okplan) = test_plan {
    //     okplan.plan();
    // }
    // plan1.plan();
    // user_input_parse("2:30");
    match test_plan {
        Ok(okplan) => {
            okplan.plan();
        }
        Err(e) => println!("{}", e),
    }
    println!("The program doesn't stops after this error!");
}
