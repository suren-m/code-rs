fn demo() {
    let check = true;

    const MAX_HEALTH: u8 = 100;

    let x: u8 = 250;
    let x: i32 = 25_000; // shadow

    let mut pi: f64 = 3.14;
    pi = 3.14159;

    let letter: char = 'a';

    let items = [1, 2, 3, 4, 5];
    let sliced = [items[2]..items[4]];
    let zeroes: [i32; 4] = [0; 4]; // init with 0s

    let plots: [(i32, i32); 2] = [(2, 3), (1, 2)];
    for (x, y) in plots.iter() {
        dbg!("x:{} y:{}", x, y);
    }

    'outer: for i in (1..=5).rev() {
        for j in ('a'..'z') {
            if i == 4 {
                break 'outer;
            }
            println!("{}{}", j, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::demo;
    #[test]
    // cargo test -- --nocapture
    fn demo_test() {
        demo();
    }
}
