fn demo() {
    let x = String::from("hello");
    let y = x; // ownership moved to y
               // println!("{}", x); // compile time error

    let s1 = String::from("hello");

    let (s1, len) = get_len(s1);
    fn get_len(s: String) -> (String, usize) {
        let len = s.len();
        (s, len)
    }

    let len = get_len_v2(&s1);
    dbg!(len);
    fn get_len_v2(s: &String) -> usize {
        s.len()
    }

    let mut s = String::from("hello");
    change(&mut s);
    change_again(&mut s);
    fn change(s1: &mut String) {
        s1.push_str(", world");
    }
    fn change_again(s1: &mut String) {
        s1.push_str("...!");
    }

    // let ref1 = &mut s;
    // let ref2 = &mut s;
    // let x = ref1;
    // dbg!(s);

    {
        let ref1 = &mut s;
        // do some work with the data
    } // ref1 goes out of scope
    let ref2 = &mut s;

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // No more Dangling pointers.
    // This won't even compile
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }

    let s: String = String::from("hello world");

    let hello: &str = &s[0..5]; //hello
    let world = &s[6..]; //world
    let all = &s[..]; //hello world

    // Takes a slice and returns a slice
    // works with both String and slice inputs
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let some_literal = "Hello World";
    let res = first_word(some_literal);

    let mut some_str = String::from("hello again");
    let res2 = first_word(&some_str[..]);

    // Clear not allowed because immutable borrow still being used.
    // some_str.clear();
    println!("{}", res2);

    print!("..done..");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_test() {
        demo();
    }
}
