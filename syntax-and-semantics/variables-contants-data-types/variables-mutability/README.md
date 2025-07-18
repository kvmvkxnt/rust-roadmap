# Variables and Mutability

By default, variables are immutable. This is one of many nudges Rust gives you to
write your code in a way that takes advantage of the safety and easy concurrency
that Rust offers. However, you still have the option to make your variables mutable.
Let's explore how and why Rust encourages you to favor immutability and why sometimes
you might want to opt out.

When a variable is immutable, once a value is bound to a name, you can't change
that value. To illustrate this, generate a new project called _variables-mutability_
by using `cargo new variables-mutability`.

Then, in your _src/main.rs_, replace its code with the following code, which won't
compile just yet:

Filename: src/main.rs

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

Save and run the program using `cargo run`. You should receive an error message
regarding an immutability error, as shown in this output:

```bash
➜  variables-mutability git:(main) ✗ cargo run
   Compiling variables-mutability v0.1.0 (/Users/kvmvkxnt/hobby/rust-roadmap/syntax-and-semantics/variables-contants-data-types/variables-mutability)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables-mutability` (bin "variables-mutability") due
to 1 previous error
```

You received the error message `cannot assign twice to immutable variable 'x'`
because you tried to assign a second value to the immutable `x` variable.

It's important that we get compile-time errors when we attempt to change a value
that's designated as immutable because this very situation can lead to bugs. If one
part of your code operates on the assumption that a value will never change and
another part of our code changes that value, it's possible that the first part of
the code won't do what it was designed to do. The cause of this kind of bug can
be difficult to track down after the fact, especially when the second piece of code
changes the value only _sometimes_. The Rust compiler guarantees that when you state
that a value won't change, it really won't change, so you don't have to keep track
of it yourself. Your code is thus easier to reason through.

But mutability can be very useful, and can make code more convenient to write.
Although variables are immutable by default, you can make them mutable by adding
`mut` in front of the variable name. Adding `mut` also conveys intent to future
readers of the code by indicating that other parts of the code will be changing
this variable's value.

For example, let's change _src/main.rs_ to the following:

Filename: src/main.rs

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

When we run the program now, we get this:

```bash
➜  variables-mutability git:(main) ✗ cargo run
   Compiling variables-mutability v0.1.0 (/Users/kvmvkxnt/hobby/rust-roadmap/syntax-and-semantics/variables-contants-data-types/variables-mutability)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/variables-mutability`
The value of x is: 5
The value of x is: 6
```

We're allowed to change the value bound to `x` from `5` to `6` when `mut` is used.
Ultimately, deciding whether to use mutability or not is up to you and depends on
what you think is clearest in that particular situation.

## Constants

Like immutable variables, _constants_ are values that are bound to a name and are
not allowed to change, but there are a few differences between constants and
variables.

First, you aren't allowed to use `mut` with constants. Constants aren't just immutable
by default - they're always immutable. You declare constants using the `const`
keyword instead of the `let` keyword, and the type of the value _must_ be annotated.

## **Constants**

Like immutable variables, _constants_ are values that are bound to a name and are
not allowed to change, but there are a few differences between constants and variables.

First, you aren't allowed to use `mut` with constants. Constants aren't just immutable
by default - they're always immutable. You declare constants using the `const`
keyword instead of the `let` keyword, and the type of the value _must_ be annotated.

Constants can be declared in any scope, including the global scope, which makes
them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression,
not the result of a value that could only be computed at runtime.

Here's an example of a constant declaration:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Constants are valid for the entire time a program runs, within the scope in which
they were declared. This property makes constants useful for values in your application
domain that multiple parts of the program might need to know about, such as the
maximum number of points any player of a game is allowed to earn, or the speed of
light.

## Shadowing

You can declare a new variable with the same name as a previous variable. Rustaceans
say that the first variable is _shadowed_ by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In effect,
the second variable overshadows the first, taking any uses of the variable name
to itself until either it itself is shadowed or the scope ends. We can shadow a
variable by using the same variable's name and repeating the use of the `let`
keyword as follows:

Filename: src/main.rs

```rust
fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

This program first binds `x` to a value of `5`. Then it creates a new variable `x`
by repeating `let x =`, taking the original value and adding `1` so the value of
`x` is then `6`. Then, within an inner scope created with the curly brackets, the
third `let` statement also shadows `x` and creates a new variable, multiplying the
previous value by `2` to give `x` a value of `12`. When that scope is over, the
inner shadowing ends and `x` returns to being `6`. When we run this program, it
will output the following:

```bash
➜  variables-mutability git:(main) ✗ cargo run
   Compiling variables-mutability v0.1.0 (/Users/kvmvkxnt/hobby/rust-roadmap/syntax-and-semantics/variables-contants-data-types/variables-mutability)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/variables-mutability`
The value of x in the inner scope is: 12
The value of x is: 6
```

Shadowing is different from marking a variable as `mut` because we'll get a
compile-time error if we accidentally try to reassign to this variable without
using the `let` keyword. By using `let`, we can perform a few transformations on
a value but have the variable be immutable after those transformations have been
completed.

The other difference between `mut` and shadowing is that because we're effectively
creating a new variable when we use the `let` keyword again, we can change the
type of the value but reuse the same name. For example, say our program asks a user
to show how many spaces they want between some text by inputting space characters,
and then we want to store that input as a number:

```rust
  let spaces = "    ";
  let spaces = spaces.len();
```

The first `spaces` variable is a string type and the second `spaces` variable is
a number type. Shadowing thus spares us from having to come up with different names,
such as `spaces_str` and `spaces_num`; instead, we can reuse the simpler `spaces`
name. However, if we try to use `mut` for this, as shown here, we'll get a compile-time
error:

```rust
  let mut spaces = "    ";
  spaces = spaces.len();
```

The error says we're not allowed to mutate a variable's type:

```bash
➜  variables-mutability git:(main) ✗ cargo run
   Compiling variables-mutability v0.1.0 (/Users/kvmvkxnt/hobby/rust-roadmap/syntax-and-semantics/variables-contants-data-types/variables-mutability)
error[E0308]: mismatched types
  --> src/main.rs:29:14
   |
28 |     let mut spaces = "    ";
   |                      ------ expected due to this value
29 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables-mutability` (bin "variables-mutability") due
to 1 previous error
```
