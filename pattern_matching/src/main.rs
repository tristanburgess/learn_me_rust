
struct Point {
    x: i32,
    y: i32,
}

fn print_coords(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "27".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 25 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using black as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (idx, val) in v.iter().enumerate() {
        println!("val {} is at idx {}", val, idx);
    }

    let (x, y, z) = (1, 2, 3);
    let (_, y, ..) = (1, 2, 3);
    let point = (3, 5);
    print_coords(&point);

    let x = 'c';
    match x {
        'a'..='j' => println!("early ascii lowercase letter"),
        'k'..='z' => println!("late ascii lowercase letter"),
        _ => println!("something else..."),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }
}
