rustc hello_world.rs

formatta codice:

> rustfmt hello_world.rs

 create project with cargo

> cargo new hello_cargo

ftom inside directory 

> cargo build

> ./target/debug/hello_cargo 

> cargo run

> cargo check

When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug.

> cargo build --release

> ./target/release/hello_cargo


documentation?
> cargo doc --open

### good rules
- 4 spazi, non tab
- constant vanno scritte in maiuscolo, con underscore tra le due parole nel caso.(const MAX_POINTS: u32 = 100_000;)


### appunti
println (without the !). We’ll discuss Rust macros in more detail in Chapter 19. For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function.

con ! chiama una macro, senza una funzione

Just compiling with rustc is fine for simple programs, but as your project grows, you’ll want to manage all the options and make it easy to share your code. Next, we’ll introduce you to the Cargo tool, which will help you write real-world Rust programs.

programmi semplici compili con rustc, per più complessi con cargo

We can build a project using cargo build or cargo check.
We can build and run a project in one step using cargo run.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

std::io
- std = standard library
- io = input output

In Rust, variables are immutable by default.
mut before the variable name to make a variable mutable:

```
let foo = 5; // immutable
let mut bar = 5; // mutable
```

Rust’s naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability
