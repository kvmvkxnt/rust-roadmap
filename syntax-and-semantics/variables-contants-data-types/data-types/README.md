# Data Types

Every value in Rust is of a certain _data type_, which tells Rust what kind of data
is being specified so it knows how to work with that data. We'll look at two data
type subsets: scalar and compound.

Keep in mind that Rust is a _statically typed_ language, which means that it must
know the types of all variables at compile time. The compiler can usually infer
what type we want to use based on the value and how we use it. In cases when many
types are possible, such as when we converted a `String` to a numeric type using
`parse`, we must add a type annotation, like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don't add the `: u32` type annotation shown in the preceding code, Rust will
display the following error, which means the compilers needs more information from
us to know which type we want to use:

```bash
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `guess` an explicit type
  |
2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
  |              ++++++++++++

For more information about this error, try `rustc --explain E0284`.
error: could not compile `no_type_annotations` (bin "no_type_annotations") due
to 1 previous error
```

You'll see different type annotations for other data types.

## Scalar Types

A _scalar_ type represents a single value. Rust has four primary scalar types:
integers, floating-point numbers, Booleans, and characters. You may recognize these
from other programming languages. Let's jump into how they work in Rust.

### Integer Types

An _integer_ is a number without a fractional component. This type declaration
indicates that the value it's associated with should be an unsigned integer
(signed integer types start with `i` instead of `u`) that takes up 32 bits of space.

Integer types in Rust

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Each variant can be either signed or unsigned and has an explicit size. _Signed_
and _unsigned_ refer to whether it's possible for the number to negative - in other
words, whether the number needs to have a sign with it (signed) or whether it will
only ever be possible and can therefore be represented without a sign (unsigned).
It's like writing numbers on paper: when the sign matters, a number is shown with
a plus sign or a minus sign; however, when it's safe to assume the number is positive,
it's shown with no sign or a minus sign. Signed numbers are stored using [two's
complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation.

Each signed variant can store numbers from $−(2^{n-1})$ to $2^{n−1}−1$ inclusive,
where _n_ is the number of bits that variant uses. So an `i8` can store numbers from
$-(2^7)$ to $2^7-1$, which equals $-128$ to $127$. Unsigned variants can store numbers
from $0$ to $2^n-1$, so a `u8` can store numbers from $0$ to $2^8-1$, which
equals $0$ to $255$.

Additionally, the `isize` and `usize` types depend on the architecture of the computer
your program is running on, which is denoted in the table as "arch": 64 bits if you're
on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.

You can write integer literals in any of the forms shown in table below. Note that
number literals that can be multiple numeric types allow a type suffix, such as
`57u8`, to designate the type. Number literals can also use `_` as a visual separator
to make the number easier to read, such as `1_000`, which will have the same value
as if you had specified `1000`.

Integer literals in Rust

| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

So how do you know which type of integer to use? If you're unsure, Rust's defaults
are generally good places to start: integer types default to `i32`. The primary
situation in which you'd use `isize` or `usize` is when indexing some sort of collection.

> **Integer Overflow**
>
> Let's say you have a variable of type `u8` that can hold values from 0 and 255.
> If you try to change the variable to a value outside that range, such as 256,
> _integer overflow_ will occur, which can result in one of two behaviors. When you're
> compiling in debug mode, Rust includes checks for integer overflow that cause your
> program to _panic_ at runtime if the behavior occurs. Rust uses the term _panicking_
> when a program exists with an error.
>
> When you're compiling in release mode with the `--release` flag, Rust does _not_
> include checks for integer overflow that cause panics. Instead, if overflow occurs,
> Rust performs _two's complement wrapping_. In short, values greater than the maximum
> value the type can hold "wrap around" to the minimum of the values the type can
> hold. In the case of a `u8`, the value 256 becomes 0, the value 257 becomes 1,
> and so on. The program won't panic, but the variable will have a value that probably
> isn't what you were expecting it to have. Relying on integer overflow's wrapping
> behavior is considered an error.
>
> To explicitly handle the possibility of overflow, you can use these families of
> methods provided by the standard library for primitive numeric types:
>
> - Wrap in all models with the `wrapping_*` methods, such as `wrapping_add`.
> - Return the `None` value if there is overflow with the `checked_*` methods.
> - Return the value and a Boolean indicating whether there was overflow with the
>   `overflowing_*` methods.
> - Saturate at the value's minimum or maximum values with the `saturating_*` methods.

### Floating-Point Types

Rust also has two primitive types for _floating-point_ numbers, which are numbers
with decimal points. Rust's floating-point types are `f32` and `f64`, which are
32 bits and 64 bits in size, respectively. The default type is `f64` because on
modern CPUs, it's roughly the same speed as `f32` but is capable of more precision.
All floating-point types are signed.

Here's an example that shows floating-point numbers in action:

Filename: src/main.rs

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard.

### Numeric operations

Rust supports the basic mathematical operations you'd expect for all the number
types: addition, subtraction, division, and remainder. Integer division truncates
toward zero to the nearest integer. The following code shows how you'd use each numeric
operation in a `let` statement:

Filename: src/main.rs

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates to
a single value, which is then bound to a variable.

### The Boolean Type

As in most other programming languages, a Boolean type in Rust has two possible values:
`true` and `false`. Booleans are one byte in size. The Boolean type in Rust is specified
using `bool`. For example:

Filename: src/main.rs

```rust
fn main() {
  let t = true;
  let f: bool = false; // with explicit type annotation
}
```

### The Character Type

Rust's `char` type is the language's most primitive alphabetic type. Here are some
examples of declaring values:

Filename: src/main.rs

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

Note that we specify `char` literals with single quotes, as opposed to string literals,
which use double quotes. Rust's `char` type is four bytes in size and represents
a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width
spaces are all valid `char` values in Rust. Unicode Scalar Values range from `U+0000`
to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. However, a "character" isn't
really a concept in Unicode, so your human intuition for what a "character" is may
not match up with what a `char` is in Rust.

## Compound Types

_Compound types_ can group multiple values into one type. Rust has two primitive
compound types: tuples and arrays.

### The Tuple Type

A _tuple_ is a general way of grouping together a number of values with a variety
of types into one compound type. Tuples have a fixed length: once declared, they
cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses.
Each position in the tuple has a type, and the types of the different values in the
tuple don't have to be the same. We've added optional type annotations in this example:

Filename: src/main.rs

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple because a tuple is considered a single
compound element. To get the individual values out of a tuple, we can use pattern
matching to destructure a tuple value, like this:

Filename: src/main.rs

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
```

This program first creates a tuple and blinds it to the variable `tup`. It then
uses a pattern with `let` to take `tup` and turn into three separate variables,
`x`, `y`, and `z`. This is called _destructuring_ because it breaks the single tuple
into three parts. Finally, the program prints the value of `y`, which is `6.4`.

We can also access a tuple element directly by using a period (`.`) followed by
the index of the value we want to access. For example:

Filename: src/main.rs

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

This program creates the tuple `x` and the accesses each element of the tuple using
their respective indices. As with most programming languages, the first index in
a tuple is 0.

The tuple without any values has a special name, _unit_. This values and its corresponding
type are both written `()` and represent an empty value or an empty return type.
Expressions implicitly return the unit value if they don't return any other value.

### The Array Type

Another way to have a collection of multiple values is with an _array_. Unlike
a tuple, every element of an array must have the same type. Unlike arrays in some
other languages, arrays in Rust have a fixed length.

We write the values in an array as a comma-separated list inside square brackets:

Filename: src/main.rs

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack, the same as the
other types we have seen so far, rather than the heap or when you want to ensure
you always have a fixed number of elements. An array isn't as flexible as vector
type, though. A _vector_ is a similar collection type provided by the standard
library that _is_ allowed to grow or shrink in size. If you're unsure whether to
use an array or a vector, chances are you should use a vector.

However, arrays are more useful when you know the number of elements will not need
to change. For example, if you were using the names of the month in a program, you
would probably use an array rather than a vector because you know it will always
contain 12 elements:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July", "August",
"September", "October", "November", "December"];
```

You write an array's type using square brackets with the type of each element, a
semicolon, and then then the number of elements in the array, like so:

```rust
let a: [i32, 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element. After the semicolon, the number `5` indicates
the array contains five elements.

You can also initialize an array to contain the same value for each element by specifying
the initial value, followed by a semicolon, and then the length of the array in square
brackets, as shown here:

```rust
let a = [3; 5];
```

The array named `a` will contain `5` elements that will all be set to the value
`3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a more
concise way.

#### Accessing Array Elements

An array is a single chunk of memory of a known, fixed size that can be allocated
on the stack. You can access elements of an array using indexing, like this:

Filename: src/main.rs

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named `first` will get the value `1` because that
is the value at index `[0]` in the array. The variable named `second` will get the
value `2` from index `[1]` in the array.

#### Invalid Array Element Access

Let's see what happens if you try to access an element of an array that is past
the end of the array. Say you run this code, similar to the guessing game, to get
an array index from the user:

Filename: src/main.rs

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

This code compiles successfully. If you run this code using `cargo run` and enter
`0`, `1`, `2`, `3`, or `4`, the program will print out the corresponding value at
that index in the array. If you instead enter a number past the end of the array,
such as `10`, you'll see output like this:

```bash
➜  data-types git:(main) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/data-types`
Please enter an array index.
10

thread 'main' panicked at src/main.rs:81:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The program resulted in a _runtime error_ at the point of using an invalid value
in the indexing operation. The program exited with an error message and didn't
execute the final `println!` statement. When you attempt to access an element using
indexing, Rust will check that the index you've specified is less that the array
length. If the index is greater than or equal to the length, Rust will panic. This
check hash to happen at runtime, especially in this case, because the compiler can't
possibly know what value a user will enter when they run the code later.
