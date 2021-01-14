#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let val = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));

        *val.borrow_mut() += 10;
        println!("a after mut = {:?}", a);
        println!("b after mut = {:?}", b);
        println!("c after mut = {:?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
