# aoutils

A tiny utility library I published in order to learn to publish and use myown crates. As I learn more Rust, I may add more functions until this becomes an actual useful utility.

## version

0.1.1

## usage

```rust
use aoutils;

fn main() {
    let result = aoutils::ensure_newline("alpha");
    assert_eq!(result, "alpha\n");

    let result = aoutils::is_alphabetic("alpha");
    assert_eq!(result, true);
}
```

## License
[MIT](https://choosealicense.com/licenses/mit/)