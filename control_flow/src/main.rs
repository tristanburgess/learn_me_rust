fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }

    // NOTE(tristan): number could be (for example) divisible by
    // 4, 3, and 2, but will only print the line for divisible by 4
    // due to that if branch matching first.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 11 };
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("indexed while loop: The value is: {}", a[index]);
        index += 1;
    }
    for index in 0..5 {
        println!("indexed for loop: The value is: {}", a[index]);
    }
    for elem in a.iter() {
        println!("for iter: The value is: {}", elem);
    }
}
