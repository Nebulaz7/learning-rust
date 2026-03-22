fn main() {
   // my first rust code
    println!("main\nmy name is joshua");
    let name = "my ex";
    println!("I am in love with {}", name);

    let mut x = 5;
    
    println!("x = {}", x);
    x = 10; 

    let y = x + 5;

    const MY_CONSTANT: i32 = 87365;

    println!("My constant value is {}", MY_CONSTANT);

    println!("x = {}", y);

    let is_rust_good = true;

    println!("Is Rust good? {}", is_rust_good);
} 