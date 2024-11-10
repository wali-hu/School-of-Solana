use std::io;

const FOO: u32 = 10;

fn main() {
    println!("The value of constant variable FOO is: {}", FOO);

    //mutable variable
    let mut x = 5;
    println!("x is a mutable variable: {}", x);
    x = 6;
    println!("that is why new value of x: {}", x);

    //immutable variable
    let y = 4;
    println!("y is a immutable variable: {}", y);

    //tupple
    let z: (u32, u32, u32) = (1, 2, 3);
    println!("The value of tupple z is: {:?}", z);

    //array
    let arr: [u32; 3] = [1, 2, 3];
    println!("The value of arr is: {:?}", arr);

    //string-slice
    let s1: &str = "salaam there";
    println!("The value of string s1 is: {}", s1);

    //string
    let s2 = String::from("wasalaam there");
    println!("The value of string s2 is: {}", s2);

    let s3 = ("wasalaam here").to_string();
    println!("The value of string s3 is: {}", s3);

    // accessing the value of string from &str
    let _s4 = String::from("jado dekhaoun ?");
    let s5: &str = &_s4;
    println!("The value of string s5 is: {}", s5);

    // accessing the value of &str from String (to_string)
    let s6: &str = "han dekhao";
    let s7: String = s6.to_string();
    println!("The value of string s7 is: {}", s7);

    // accessing the value of &str from String (String::from)
    let s8: &str = "kesa lga jado?";
    let s9: String = String::from(s8);
    println!("The value of string s9 is: {}", s9);

    // concatination of a string and &str
    let mut s10 = String::from("hello");
    s10.push_str(", world");
    println!("The value of string s10 is: {}", s10);
    //println!();

    // borrowing a part of string by using &str[start..end]
    let s11: &str = &s10[0..5];
    println!("The value of string s11 is: {}", s11);

    // Control flow: if-else
    let x = 10;
    if x > 87 {
        println!("{} is greater than 5", x);
    } else {
        println!("{} is less than 5", x);
    }

    let y = if x > 5 { 10 } else { 0 };
    println!("The value of y is: {}", y);

    // Control flow: loop
    loop {
        println!("This is a loop");
        break;
    }

    // Control flow: while
    let mut i = 5;
    while i > 0 {
        println!("The value of i is: {}", i);
        i -= 1;
    }

    // Control flow: for
    for n in 1..5 {
        println!("The value of n is : {}", n);
    }

    // Control flow: match
    let color = "green";
    match color {
        "red" => println!("The color is red"),
        "blue" => println!("The color is blue"),
        _ => println!("The color is neither red nor blue"),
    }

    println!("Please enter a number: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    match num {
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        _ => println!("The number is not 1, 2, or 3"),
    }
}
