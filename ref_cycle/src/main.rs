use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List<T: Debug> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T: Debug> List<T> {
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node<T: Debug> {
    val: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b created = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next time = {:?}", b.tail());

    // CRITICAL(tristan): causes a reference cycle. b already linked to a, and this makes a link to b.
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after linking a to b = {}", Rc::strong_count(&b));
    println!("a rc count after linking a to b = {}", Rc::strong_count(&a));

    // CRITICAL(tristan): uncommenting causes stack overflow since a and b have a reference cycle!
    // println!("a next item = {:?}", a.tail());
    let leaf = Rc::new(Node {
        val: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong count = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            val: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "branch strong count = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong count = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    let parent = leaf.parent.borrow().upgrade();
    if let Some(parent) = parent {
        println!("leaf parent still exists! = {:?}", parent);
    } else {
        println!("leaf parent was dropped!");
    }

    println!(
        "leaf strong count = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
