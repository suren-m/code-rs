# Grid Traveller

* m x n grid 
* Move right or down to reach end of the grid (target)
* calculate max number of ways to reach the target
* spin of from fibonacci using recursion tree


### Basic 

```rust
pub fn travel_grid(m: u8, n: u8) -> u32 {
    match (m, n) {
        (1, 1) => 1, // base case
        (_, 1) => 1, // right only
        (1, _) => 1, // down only
        (_, 0) => 0, // invalid
        (0, _) => 0, // invalid
        (rows, cols) => travel_grid(rows - 1, cols) + travel_grid(rows, cols - 1),
    }
}

assert_eq!(travel_grid(2, 3), 3);
assert_eq!(travel_grid(3, 3), 6);
println!("done");
```

### With Memoization
```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Pos(u8, u8);

pub fn travel_grid(m: u8, n: u8, memo: &mut HashMap<Pos, u32>) -> u32 {
    match (m, n) {
        (1, 1) => 1,
        (_, 1) => 1,
        (1, _) => 1,
        (0, _) => 0,
        (_, 0) => 0,
        (rows, cols) => {
            let pos = Pos(rows, cols);
            if let Some(val) = memo.get(&pos) {
                *val
            } else {
                let res = travel_grid(rows -1, cols, memo) + travel_grid(rows, cols-1, memo);
                memo.insert(pos, res);
                res
            }
        },
    }
}

let mut memo: HashMap<Pos, u32> = HashMap::new();
assert_eq!(travel_grid(2, 3, &mut memo), 3);
assert_eq!(travel_grid(3, 3, &mut memo), 6);
assert_eq!(travel_grid(18, 18, &mut memo), 2_333_606_220);
println!("done");
```