# Fibonacci

```rust
fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 2) + fib(n - 1)
    }
}

assert_eq!(fib(5), 5);
assert_eq!(fib(25), 75025);
println!("done");
```

### Fibonacci with memoization

```rust
use std::collections::HashMap;
fn fib(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            if let Some(val) = memo.get(&n) {
                *val
            } else {
                let res = fib(n - 2, memo) + fib(n -1, memo);
                memo.insert(n, res);
                res
            }
        }
    }
}

let mut memo: HashMap<usize, usize> = HashMap::new();
assert_eq!(fib(5, &mut memo), 5);
assert_eq!(fib(25, &mut memo), 75025);
println!("done");
```

### Fibonacci using iteration

```rust
fn fib(n: usize) -> usize {
    let mut a = 0;
    let mut b = 1;
    let mut res = 0;
    match n {
        0 => 0,
        1 => 1,
        _ => {
            for _ in 2..=n {
                res = b + a;
                a = b;
                b = res;
            }
            res
        }        
    }
}

assert_eq!(fib(5), 5);
assert_eq!(fib(25), 75025);
println!("done");
```

### Fibonacci using Tabulation

```rust
fn fib(n: usize) -> usize {
    let mut table = vec![0; n+1]; // to accomodate fib(0)
    table[1] = 1; // base case
    for i in 0..table.len() -1 {
        table[i+1] += table[i];
        if i < table.len() - 2 {
            table[i+2] += table[i];
        }        
    }  
    return table[n]; 
}
assert_eq!(fib(5), 5);
assert_eq!(fib(25), 75025);
assert_eq!(fib(50), 12586269025);
println!("done");
```