fn main() {
    // my first rust code
    println!("main\nmy name is ");
    let mut x = 5;

    println!("x = {}", x);
    x = 10;
    let mut ectasy = "stuff";

    let y = x + 5;

    const MY_CONSTANT: i32 = 87365;

    println!("My constant value is {}", MY_CONSTANT);

    println!("x = {}", y);

    let is_rust_good = true;

    println!("Is Rust good? {}", is_rust_good);

    let age = 30;
    let can_vote = age >= 18;

    println!("Can I vote? {}", can_vote);

    let is_logged_in = false;

    if is_logged_in {
        println!("Welcome back!")
    } else {
        println!("Please Log in..............")
    }

    let month = 3;
    //   let more_months = 12; syff

    match month {
        1 => println!("january"),
        2 => println!("febuary"),
        3 => println!("march"),
        4 => println!("april"),
        5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => println!("Months coming soon!!"),
        _ => println!("Invalid date"),
    };
}
