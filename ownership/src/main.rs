fn main() {
    // s is not yet valid, hasn't been declared
    
    // hardcoded into final executable
    let s = "hello"; // s is valid from this point forward, comes into scope

    // do stuff with s

    // allocated on the heap (at runtime)
    let mut t = String::from("hello");
    // allocate more heap memory to hold the new total String contents
    t.push_str(", world!");
    println!("{}", t);

    let x = 5;
    // copies the value of x, since int has fixed size, values pushed on stack
    let y = x;

    // s1 is a fixed sized struct containing
    // ptr to heap memory containing string value
    // len in bytes
    // capacity in bytes
    let s1 = String::from("hello");
    // s2 makes a new copy of the struct, not the heap allocated string value
    // ownership is moved to s2 variable to prevent double free
    // s1 is no longer a valid variable
    // thus Rust does a shallow copy + move, never automatically performs deep copy
    let mut s2 = s1;
    // ERROR can't borrow moved value
    // let s3 = s1.clone()
    // intentionally deep copy heap contents pointed to by
    // s2 for pointing by s3
    // explicit clone() only required for heap data, stack copies are fixed size and cheap
    // specifically, explicit clone() only required for types that do not
    // implement the Copy trait. Implementing the Copy trait requires that the type
    // is only composed of scalar values which do not implement the Drop trait.
    // e.g. (i32, i32) is Copy, but (i32, String) is not.
    let mut s3 = s2.clone();

    // passing variables to a function works similarly as assignment in terms of move or copy.
    // function return values can be moved into a variable in the parent scope.

    // We can use references to avoid having to pass variables back from a function (borrows).
    // calc... thus borrows (owns a new read only pointer) to s2.
    let len = calculate_length(&s2);
    println!("The length of '{}' is {}.", s2, len);

    // Changing a referenced value requires to take a mutable reference/borrow
    change(&mut s3);
    println!("s3 is: '{}'", s3);

    // This works because the (very) last time the immutable references are used
    // occurs before the mutable reference is declared.
    let r1 = &s2;
    let r2 = &s2;
    println!("{} and {}", r1, r2);
    let r3 = &mut s2;
    println!("{}", r3);
    // would result in a compile time error stating that r3 is not allowed
    // since we're using immutable references after it would be declared.
    // println!("{} and {}", r1, r2);

    let mut s4 = String::from("hello world");
    let word = first_word_first_pass(&s4);
    s4.clear();
    // word still points to index 5 marking the end of the first word that
    // was in s4, but s4 is now empty, word has to be updated again, and is stale
    // data until then.

    // solution: string slices!
    // references to a portion of a String.
    // range indices must occur at valid UTF-8 character boundaries
    // can't slice into the middle of a multibyte character.
    let mut s4 = String::from("hello world");
    let hello = &s4[..5];
    let world = &s4[6..];

    let word = first_word(&s4);
    // ERROR: the line below borrows s4, thus it can't also be borrowed
    // mutable at this point to clear it
    // s4.clear();
    println!("the first word is: {}", word);
    // but as before, this is ok because the compiler can verify this clear() happens after the
    // last time s4 is used.
    s4.clear();
} // s, t, no longer valid, goes out of scope
// s is popped off the stack
// t is deallocated automatically from the heap using drop() (RAII)

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!")
}

// This mostly works, but the problem
// is that this returns a usize value which must be stored separately from
// and thus maintained alongside the String the usize value is tracking.
fn first_word_first_pass(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// The string slice is a ref back to s, thus
// the slice state is maintained alongside s automagically
// accepts &str as a param because that works with both &String and &str values.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
