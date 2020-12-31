#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // For the most part, lifetimes are inferred
    // but when there's ambiguity, we have to explicitly resolve it.

    let r;

    {
        let x = 5;
        r = &x;
    //} ERROR: scope ending here means that println! refers to a borrow that's gone out of scope
    // The subject of a reference must live at least as long as the reference.

        println!("r: {}", r);
    }

    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(&s1, s2);
    println!("{}", result);

    let s3 = String::from("long string is long");
    // s4 has a smaller lifetime than s3, but this is still
    // accepted by the boorow checker because the return reference is still valid
    // and only used within this smaller lifetime.
    {
        let s4 = String::from("xyz");
        let result = longest(&s3, &s4);
        println!("The longest string is {}", result);
    }

    /*
    // ERROR: &s6 does not live long enough to satisfy that
    // the println wants to use the result outside the smaller scope
    let s5 = String::from("long string is long");
    let result;
    {
        let s6 = String::from("xyz");
        result = longest(&s5, &s6);
    }
    println!("The longest string is {}", result);
    */

    let novel = String::from("testing 1 2 3 asdf loren ipsum blah. Here's some more words.");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:#?}", i);
}

// Need explicit lifetime params, otherwise because
// compiler doesn't know if return ref is x or y,
// it can't automatically infer lifetimes.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }

    y
}
