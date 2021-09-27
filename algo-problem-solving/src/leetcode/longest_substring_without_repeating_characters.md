# Longest Substring
[https://leetcode.com/problems/longest-substring-without-repeating-characters/](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

```rust
//! 
use std::cmp;
use std::collections::HashMap;

/// two pointer and sliding window approach - single pass
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut dict: HashMap<char, usize> = HashMap::with_capacity(s.len());
    let mut max = 0;
    let mut start = 0;
    let mut end = 0;

    for (i, ch) in s.chars().enumerate() {
        if let Some(cached_index) = dict.get(&ch) {
            if *cached_index >= start {
                // only if is in current window
                let curr = end - start; // calc length of current window
                max = cmp::max(curr, max);
                // move left pointer - set start to just after of current duplicate index
                start = *cached_index + 1;
            }
            // skip if it's a new sequence
        }
        end += 1; // move right pointer - keep counting
        dict.insert(ch, i); //insert or update ch and its index
    }
    let curr = end - start;
    max = cmp::max(curr, max);
    max as i32
}

assert_eq!(3, length_of_longest_substring("abaaacabcbb".to_string())); // abc
assert_eq!(3, length_of_longest_substring("abcabcbb".to_string())); // abc
assert_eq!(4, length_of_longest_substring("abcadabb".to_string())); // bcad
assert_eq!(3, length_of_longest_substring("pwwkew".to_string())); // wke
```