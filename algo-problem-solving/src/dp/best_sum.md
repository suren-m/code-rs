# Best Sum

* extension from `can_sum`
* least number of combinations to reach target

```rust
/// No early returns as in can sum since we need to check all the combinations
pub fn best_sum(nums: &Vec<isize>, target: isize) -> Option<Vec<isize>> {
    match target {
        0 => Some(Vec::new()),
        x if x < 0 => None,
        _ => {
            let mut shortest_combo: Option<Vec<isize>> = None;
            for num in nums {
                let diff = target - num; // new target
                if let Some(mut combo) = best_sum(nums, diff) {
                    combo.push(*num);
                    match &shortest_combo {
                        Some(shortest) => {
                            if combo.len() < shortest.len() {
                                shortest_combo = Some(combo);
                            }
                        },
                        None => {
                            shortest_combo = Some(combo)
                        } 
                    } 
                }
            }
            shortest_combo
        }
    }
}

// [3, 4] or [4, 3] or [7]
// expected is 7 because it's the smallest
assert_eq!(best_sum(&vec![5, 3, 4, 7], 7).unwrap(), vec![7]);
// no combinations possible. is_none is true
assert_eq!(best_sum(&vec![2, 4], 7).is_none(), true);
// [2, 2, 2, 2] or [2, 3, 3] or [3, 5]
assert_eq!(best_sum(&vec![2, 3, 5], 8).unwrap(), vec![5, 3]);
assert_eq!(best_sum(&vec![1, 4, 5], 8).unwrap(), vec![4, 4]);
println!("done");
```

```rust
use std::collections::HashMap;
pub fn best_sum(
    nums: &Vec<isize>, 
    target: isize, 
    memo: &mut HashMap<isize, Option<Vec<isize>>>) 
-> Option<Vec<isize>> {
    match target {
        0 => Some(Vec::new()),
        x if x < 0 => None,
        _ => {            
            if let Some(val) = memo.get(&target) {
                return val.clone();
            }
            let mut best_res: Option<Vec<isize>> = None;
            for num in nums {
                let diff = target - num;
                if let Some(mut combo) = best_sum(nums, diff, memo) {
                    combo.push(*num);
                    match &best_res {
                        Some(shortest) => {
                            if combo.len() < shortest.len() {
                                best_res = Some(combo);
                            }
                        },
                        None => {
                            best_res = Some(combo)
                        }
                    }
                }
            }
            memo.insert(target, best_res.clone());
            best_res
        }
    }
}

// [3, 4] or [4, 3] or [7]
// expected is 7 because it's the smallest
assert_eq!(
    best_sum(&vec![5, 3, 4, 7], 7, &mut HashMap::new()).unwrap(),
    vec![7]
);

// no combinations possible. is_none is true
assert_eq!(
    best_sum(&vec![2, 4], 7, &mut HashMap::new()).is_none(),
    true
);

// [2, 2, 2, 2] or [2, 3, 3] or [3, 5]
assert_eq!(
    best_sum(&vec![2, 3, 5], 8, &mut HashMap::new()).unwrap(),
    vec![5, 3]
);

assert_eq!(
    best_sum(&vec![1, 4, 5], 8, &mut HashMap::new()).unwrap(),
    vec![4, 4]
);

// long runner
assert_eq!(
    best_sum(&vec![1, 2, 5, 25], 100, &mut HashMap::new()).unwrap(),
    vec![25, 25, 25, 25]
);

// long runner
// assert_eq!(how_sum_brute(&vec![7, 14], 300).is_none(), true);}
println!("done");
```