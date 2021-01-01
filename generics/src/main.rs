use generics:: {
    Summary,
    Test,
    Tweet,
};

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &elem in list {
        if elem > largest {
            largest = elem;
        }
    }
    largest
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    println!("Largest number is: {}", largest(&nums));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char is: {}", largest(&chars));

    let pi = Point { x: 2, y: 4 };
    let pf = Point { x: 2.5, y: 4.1 };
    let pif = Point { x: 2, y: 4.1 };
    let pfi = Point { x: 2.5, y: 4 };
    println!("Test - pi: {:#?}, pf: {:#?}, pif: {:#?}, pfi: {:#?}", pi, pf, pif, pfi);
    println!("pf.x = {}", pf.x());
    println!("pi mixup pf == pif: {:#?}", pi.mixup(pf));

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let test = Test{};
    println!("new test avialable: {}", test.summarize());
}
