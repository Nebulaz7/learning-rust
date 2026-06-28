fn main() {
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

    let mut count = 1;
    loop {
        println!("The value is {}", count);

        if count == 15 {
            println!("The loop is completed at value {}", count);
            break;
        }

        count += 1;
    }
}
