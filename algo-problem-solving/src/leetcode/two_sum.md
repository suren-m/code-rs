# Two Sum
[https://leetcode.com/problems/two-sum/](https://leetcode.com/problems/two-sum/)

```rust
use std::collections::HashMap;
pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut cached: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        let diff = target - v;
        if let Some(cached_index) = cached.get(&diff) {
            return vec![*cached_index, i as i32];
        };
        cached.insert(*v, i as i32);
    }
    Vec::new()
}

let nums: Vec<i32> = vec![2,7,11,15];
let target = 9;
let expected = vec![0,1];
assert_eq!(expected.clone(), two_sum(&nums, target));

```