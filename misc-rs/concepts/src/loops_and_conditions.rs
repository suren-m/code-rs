fn demo() {
    let x: i32 = if 1 > 0 { 1 } else { 0 };
}

fn fib(n: u32) -> u32 {
    let (mut a, mut b) = (1, 1);
    let result = loop {
        if b > n {
            break b;
        }
        let c = a + b;
        a = b;
        b = c;
    };
    result
}

fn for_demo() {
    for i in (1..11).rev() {
        println!("{}", i);
    }
    println!("..Done..");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_test() {
        demo();
    }

    #[test]
    fn fib_test() {
        assert_eq!(fib(10), 13);
        for_demo();
    }
}
