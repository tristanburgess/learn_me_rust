fn main() {
    another_function(5, 6, (5,6));

    let y = {
        let x = 3;
        // expression
        x + 1
        // statement (does not return value)
        // x + 1;
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(five());
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32, z: (i32, i32)) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: ({}, {})", z.0, z.1);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
