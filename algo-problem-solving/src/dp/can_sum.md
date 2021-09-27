# Can Sum

* Return true if target sum can be found from a pair of items in array
* In brute force and iterative approach, same number re-use is not implemented
* Recursion approach addresses the above

### Brute
```rust
fn can_sum(nums: &Vec<usize>, target: usize) -> bool {
    for (i, num) in nums.iter().enumerate() {
        if *num == target { return true; }
        for j in i + 1..nums.len() {
            if nums[j] + *num == target {
               return true;
            }
        }
    }
    false
}

let nums = vec![2, 3, 4, 7];
assert_eq!(true, can_sum(&nums, 7));
assert_eq!(true, can_sum(&nums, 5));
assert_eq!(false, can_sum(&nums, 8));
println!("done");
```

### Iterative - Single Pass
```rust
use std::collections::HashMap;
fn can_sum(nums: &Vec<usize>, target: usize) -> bool {
    let mut memo: Vec<usize> = Vec::new();
    for (i, num) in nums.iter().enumerate() {
        let diff = target - *num;
        if memo.contains(&diff) { // diff is the other val
            return true;
        } 
        memo.push(*num);      
    }
    false
}
let nums = vec![2, 3, 4, 7];
assert_eq!(true, can_sum(&nums, 7));
assert_eq!(true, can_sum(&nums, 5));
assert_eq!(false, can_sum(&nums, 8));
println!("done");
```

### Recursive 
* Create a recursion tree and compute diff against every item in array.
* Remainders with 0 indicate true
* **Here, Solution can use the same number in array multiple times**
* Time: O(n^m), Space: O(m)
* m is target and n is nums.len()

```rust
fn can_sum(nums: &Vec<isize>, target: isize) -> bool { 
    match target {
        0 => true, // base case
        x if x < 0 => false, // invalid case
        _ => {
            for i in nums {
                let diff = target - *i;
                if can_sum(nums, diff) == true {
                    return true;
                }
            }
            false
        }
    }
}

let nums = vec![2, 3, 4, 7];
assert_eq!(true, can_sum(&nums, 7));
assert_eq!(true, can_sum(&nums, 5));
assert_eq!(true, can_sum(&nums, 8));
let nums = vec![2];
assert_eq!(true, can_sum(&nums, 8));
println!("done");
```

### Recursion with Memoization
```rust
use std::collections::HashMap;
fn can_sum(nums: &Vec<isize>, target: isize, memo: &mut HashMap<isize, bool>) -> bool { 
    match target {
        0 => true, // base case
        x if x < 0 => false, // invalid case
        _ => {
            if let Some(val) = memo.get(&target) {
               return *val;
            } 
            for i in nums {
                let diff = target - *i;
                if can_sum(nums, diff, memo) == true {
                    memo.insert(target, true);
                    return true;
                }
            }            
            memo.insert(target, false);
            false
        }
    }
}

let nums = vec![2, 3, 4, 7];
assert_eq!(true, can_sum(&nums, 7, &mut HashMap::new()));
assert_eq!(true, can_sum(&nums, 5, &mut HashMap::new()));
assert_eq!(true, can_sum(&nums, 8, &mut HashMap::new()));
let nums = vec![2];
assert_eq!(true, can_sum(&nums, 8, &mut HashMap::new()));

// long runner
let nums = vec![7, 14];
assert_eq!(false, can_sum(&nums, 300, &mut HashMap::new()));

println!("done");
```