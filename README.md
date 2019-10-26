# brainfrsck
A [brainfrick](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in safe rust


## Usage
The main entry point is the `eval_string` function

### Example
```rust
use brainfrsck::prelude::eval_string;

let hello_world = "++++++++[>++++[>++>+++>++
+>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++
.>>.<-.<.+++.------.--------.>>+.>++.";

assert_eq!(
    eval_string(hello_world, None)?.to_string(),
    "Hello World!\n".to_owned(),
);

```

## Notes
- `eval_string` returns an `InterpreterOutput` which is essentially a wrapper for a `Vec<u8>`, it has methods to convert to a `String` (`to_string`) and to get the internal `Vec` (`to_vec`), as well as `Debug` writing the `Vec` and `Display` writing the `String`
