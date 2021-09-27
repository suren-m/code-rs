# How Sum

* Extending from `Can Sum`
* Find how to reach the 

### Recursion
```rust
pub fn how_sum(nums: &Vec<isize>, target: isize) -> Option<Vec<isize>> {
    match target {
        0 => Some(Vec::new()),
        x if x < 0 => None,
        _ => {
            for num in nums {
                let diff = target - num;
                if let Some(mut val) = how_sum(nums, diff) {
                    val.push(*num);
                    return Some(val);
                }
            }
            None
        }
    }
}

assert_eq!(how_sum(&vec![2,3], 7).unwrap(), vec![3, 2, 2]);
assert_eq!(how_sum(&vec![5,3,4,7], 7).unwrap(), vec![4, 3]);
assert_eq!(how_sum(&vec![5,3,7,4], 7).unwrap(), vec![4,3]);
println!("done");
```

### With Memoization

```rust
use std::collections::HashMap;

pub fn how_sum(
    nums: &Vec<isize>, 
    target: isize, 
    memo: &mut HashMap<isize, Option<Vec<isize>>> ) 
    -> Option<Vec<isize>> {
    match target {
        0 => Some(Vec::new()),
        x if x < 0 => None,
        _ => {
            if let Some(val) = memo.get(&target) {
                return val.clone();    
            }
            for num in nums {
                let diff = target - num;
                if let Some(mut val) = how_sum(nums, diff, memo) {
                    val.push(*num);
                    memo.insert(target, Some(val.clone()));
                    return Some(val);
                }
            }
            memo.insert(target, None);
            None
        }    
    }
}

assert_eq!(
    how_sum(&vec![2, 3], 7, &mut HashMap::new()).unwrap(),
    vec![3, 2, 2]
);
assert_eq!(
    how_sum(&vec![5, 3, 4, 7], 7, &mut HashMap::new()).unwrap(),
    vec![4, 3]
);

// no combinations possible. is_none is true
assert_eq!(
    how_sum(&vec![2, 4], 7, &mut HashMap::new()).is_none(),
    true
);

// same number reuse
assert_eq!(
    how_sum(&vec![2, 3, 5], 8, &mut HashMap::new()).unwrap(),
    vec![2, 2, 2, 2]
);

// long runner
assert_eq!(
    how_sum(&vec![7, 14], 300, &mut HashMap::new()).is_none(),
    true
);
println!("done");

```