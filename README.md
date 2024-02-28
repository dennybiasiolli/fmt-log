# fmt_log

This module contains macros for logging to the console
using the `std::println!` or `std::eprintln!` macros, but also returns the
formatted string.

Disclaimer: this started as a fun experiment on how to create a custom macro like the `std::println!` one, and now I'm using it to print logs and get the formatted output at the same time.


## `fmt_printf` example

```rust
use fmt_log::fmt_printf;

let s1 = "Hello";
let s2 = String::from("world!");
let n1 = 123;

let output = fmt_printf!("{}, {} {}", s1, s2, n1);

assert_eq!(output, format!("{}, {} {}", s1, s2, n1));
```

This will log `"Hello, world! 123"` to the console.


## `fmt_errorf` example

```rust
use fmt_log::fmt_errorf;

let s1 = "Hello";
let s2 = String::from("world!");
let n1 = 123;

let output = fmt_errorf!("{}, {} {}", s1, s2, n1);

assert_eq!(output, format!("{}, {} {}", s1, s2, n1));
```

This will log `"Hello, world! 123"` to the stderr.
