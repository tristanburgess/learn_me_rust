fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    // Modifies existing x variable in place
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    println!("The value of y is: {}", y);
    // Creates new (shadowed) y variable
    let y = y + 1;
    println!("The value of y is: {}", y);
    // Creates new (shadowed) y variable
    let y = y * 2;
    println!("The value of y is: {}", y);
    
    // Won't compile, can't shadow mutable spaces var, can only be
    // updated in place with a new value of the same type as declared.
    // let mut spaces = "..."
    let spaces = "...";
    println!("The value of spaces is: {}", spaces);
    // Assign spaces to a new type + value without the need for a new var name
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructure the tuple into 3 (immutable) vars.
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);
    // Alternate tuple member access
    println!("The value of tup is: ({}, {}, {})", tup.0, tup.1, tup.2);
}
