use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T: Debug>(T);

impl<T> MyBox<T>
where
    T: Debug,
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> 
where
    T: Debug, {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> 
where
    T: Debug, {
    fn drop(&mut self) {
        println!("Dropping MyBox wahoo!: {:?}", self);
    }
}

use crate::List::{Cons, Nil};

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("b = {:?}", list);

    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let m = MyBox::new(String::from("test"));
    hello(&m);

    let t = MyBox::new(String::from("early drop test"));
    hello(&t);
    std::mem::drop(t);
    println!("early drop test dropped before end of main.");
}
