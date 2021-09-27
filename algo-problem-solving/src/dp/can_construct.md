# Can Construct

* Similar to can sum but with strings
* diff here would be remaining substring but don't take strings in middle
* only take `starts_with` / `prefix`

```rust
pub fn can_construct(target: &str, words: &Vec<String>) -> bool {
    if target == "" {
        return true;
    }

    for word in words {
        if let Some(res) = target.strip_prefix(word) {
            if can_construct(res, words) == true {
                return true;
            }
        }
    }
    false
}

assert_eq!(can_construct(
    "abcdef",
    &vec![
        "ab".to_string(),
        "abc".to_string(),
        "cd".to_string(),
        "def".to_string(),
        "abcd".to_string()
    ]),
    true
);

assert_eq!(can_construct(
    "skateboard",
    &vec![
        "bo".to_string(),
        "rd".to_string(),
        "ate".to_string(),
        "t".to_string(),
        "ska".to_string(),
        "sk".to_string(),
        "boar".to_string()
    ]),
    false
);       
println!("done");

```

### With Memoization
```rust
use std::collections::HashMap;
pub fn can_construct(
    target: &str,
    words: &Vec<String>,
    memo: &mut HashMap<String, bool>,
) -> bool {
    if target == "" {
        return true;
    }

    if let Some(val) = memo.get(target) {
        return *val;
    }

    for word in words {
        if let Some(suffix) = target.strip_prefix(word) {
            if can_construct(suffix, words, memo) {
                memo.insert(suffix.to_owned(), true);
                return true;
            }
        }
    }

    memo.insert(target.to_owned(), false);
    false
}

assert_eq!(
    can_construct(
        "abcdef",
        &vec![
            "ab".to_string(),
            "abc".to_string(),
            "cd".to_string(),
            "def".to_string(),
            "abcd".to_string()
        ],
        &mut HashMap::new()
    ),
    true
);

assert_eq!(
    can_construct(
        "skateboard",
        &vec![
            "bo".to_string(),
            "rd".to_string(),
            "ate".to_string(),
            "t".to_string(),
            "ska".to_string(),
            "sk".to_string(),
            "boar".to_string()
        ],
        &mut HashMap::new()
    ),
    false
);

assert_eq!(
    can_construct(
        "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
        &vec![
            "e".to_string(),
            "ee".to_string(),
            "eee".to_string(),
            "eeee".to_string(),
            "eeeee".to_string(),
            "eeeeee".to_string()
        ],
        &mut HashMap::new()
    ),
    false
);

println!("done");
```