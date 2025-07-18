# Debugging Rust with rust-lldb

This is a quick and dirty primer to debugging rust applications with `rust-lldb`.

## Start the Debugger

Assuming you are using `cargo` to build your executable, the first problem you'll
run into is that you cannot debug `target/debug/BIN`; instead, you need to debug
`target/debug/deps/BIN-HASH` where `BIN` is the name of your program, and `HASH`
is a random hash... you'll probably have a ton of builds in the `deps` directory;
just use the latest (something like `ls -lt target/debug/deps/BIN-*` should help
you find the latest).

If you're trying to debug an example program, they'll live under
`target/debug/examples/BIN` - no hash nonsense.

Ok, now you can start the debugger:

```bash
rust-lldb target/debug/deps/BIN-HASH -- arg1 arg2 ...
```

Where `arg1 arg2 ...` are command line arguments to your application. If your
application needs no arguments you can just leave them off.

## Breakpoints

You are now in the lldb debugger. You'll probably want to set some breakpoints
before your program starts: you do that with the `breakpoint set` command (`b`).
There are lots of ways to set your breakpoint (specific file and line number, function
name, etc). Type `help breakpoint set` to see documentation. The shorthand, `b`,
actually accepts a simpler syntax from which it will guess the right thing to do.
For example:

```bash
breakpoint set --name func
b func
```

These two commands will do the same thing. So will:

```bash
breakpoint set -f src/path/to/file.rs -l linenr
b src/path/to/file.rs:linenr
```

Run `help b` to see the simplified syntax.

You can see a list of your breakpoints with `breakpoint list`. There are additional
things you can do with breakpoints: see `help breakpoint`.

## Running

Once you've set your breakpoints, you can start the program with `run` (`r`).
Execution will continue until a breakpoint is hit or the program ends (intentionally
or not).

## Examining Program State

Ok, so your program si running and it hits a breakpoint. Now what? You'll probably
want to examine the state of your program to get an idea of what's going on. Here
are some commands to get you started:

- `frame variable` will show you all the variables in the current "frame"; i.e.,
  variables in scope.
- `expression <expr>` (shortcut `expr` or just `p`) will execute `<expr>` and display
  the result. This can be used to examine the value of a variable or execute some
  code to see what the result is.

You may have noticed when execution first paused that a listing of your code is shown,
pointing to the current line that execution paused on. You can get that listing back
with `list` (`l`).

- `n` will "step over" the current line, i.e., the current line will execute and
  then the program will pause again.
- `s` will "step into" the current line, i.e., the innermost function will be called
  and execution will pause at the start of that function.
- `c` will continue execution until another breakpoint is reached or the program
  exits.

Finally, if you're debugging a program that interops with another language (ffi),
it might be useful to examine specific memory addresses. That can be done with
`x <addr>`. This will show a hex-dump of memory at that addresses.

## Some Other Helpful Things

If you would normally invoke your program by piping something into stdin (i.e.,
`cat file.txt | my-program`), you won't be able to do that when running your program
under rust-lldb. Instead, run rust-lldb with your program (i.e., `rust-lldb my-pogram`)
and then run:

```bash
settings set target.input-path <path-to-file>
```
