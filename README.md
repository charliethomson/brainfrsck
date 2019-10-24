# brainfrsck
A [brainfrick](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in safe rust


## Usage
The main entry point is the `eval_string` function

### Example
```rust
use brainfrsck::prelude::eval_string;

let sum = r#",>,[[-<+>],]<."#;

assert_eq!(
  eval_string(sum, Some(vec![1,2,3,4,5,6]))?.to_vec()[0],
  (0u8..=6).sum::<u8>()
);

```

## Notes
- `eval_string` returns an `InterpreterOutput` which is essentially a wrapper for a `Vec<u8>`, it has methods to convert to a `String` (`to_string`) and to get the internal `Vec` (`to_vec`), as well as `Debug` writing the `Vec` and `Display` writing the `String`
