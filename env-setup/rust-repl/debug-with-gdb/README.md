# Debugging Rust apps with GDB

## `rust-gdb` example

To debug this, we need to build it and then run `rust-gdb` with the binary. Make sure you build this with debug mode and NOT in release mode.

```bash
cd rust-gdb-example
cargo build --example basic

rust-gdb target/debug/examples/basic
```

If you never worked with GDB before, this [GDB cheat sheet](https://darkdust.net/files/GDB%20Cheat%20Sheet.pdf) might be helpful.

Let's set a breakpoint, which we can do by using the `break` command or simply using `b`:

```bash
➜  rust-gdb-example git:(master) ✗ rust-gdb target/debug/examples/basic
GNU gdb (GDB) 16.3
Copyright (C) 2024 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from target/debug/examples/basic...
(gdb) break basic::get_chip
Breakpoint 1 at 0x952e: file examples/basic.rs, line 26.
(gdb) info b
Num     Type           Disp Enb Address            What
1       breakpoint     keep y   0x000000000000952e in basic::get_chip at examples/basic.rs:26
```

We can set breakpoints at lines (e.g., `basic.rs:17`), or by providing a function to break at.

Now, that we set up our breakpoint, we can run the program by executing `run` or simply `r`:

```bash
(gdb) r
Starting program: /home/kvmvkxnt/Work/rust-roadmap/env-setup/rust-repl/debug-with-gdb/rust-gdb-example/target/debug/examples/basic

This GDB supports auto-downloading debuginfo from the following URLs:
  <https://debuginfod.archlinux.org>
Enable debuginfod for this session? (y or [n]) y
Debuginfod has been enabled.
To make this setting permanent, add 'set debuginfod enabled on' to .gdbinit.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Breakpoint 1, basic::get_chip (animals=&[rust_gdb_example::Animal](size=3) = {...}) at examples/basic.rs:26
26          let chip = animals.first();
```

This starts the program. We stop at the defined breakpoint, at the first line of the `get_chip` function. Here, we can look at the arguments to the function and try to print them:

```bash
(gdb) info args
animals = &[rust_gdb_example::Animal](size=3) = {rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Chip", age: 4}, rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Nacho", age: 6}, rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Dog, name: "Taco", age: 2}}
(gdb) p animals
$1 = &[rust_gdb_example::Animal](size=3) = {rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Chip", age: 4}, rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Nacho", age: 6}, rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Dog, name: "Taco", age: 2}}
(gdb) p *animals
Attempt to take contents of a non-pointer value.
```

The `info args` command provides an overview of the incoming arguments.

Okay, where were we? Let's execute `f` or `frame` to see where we're at:

```bash
(gdb) f
#0  basic::get_chip (animals=&[rust_gdb_example::Animal](size=3) = {...}) at examples/basic.rs:26
26          let chip = animals.first();
```

## Layouts and inspecting state

`layouts` in GDB help you see where you are in your Rust source code. Using the `layout src` command opens a command-line interface:

![Execution of `layout` command](media/layout_src.png)

Our command prompt stays right below it. There are other layouts, such as `layout split`, which shows the source and the corresponding assembly:

![Execution of `layout split` command](media/layout_split.png)

Neat. If you want to get rid of the layout, you can use `CTRL+X a`. If the rendering gets messed up, `CTRL + L` will refresh it.

As with other debuggers, we can step through the code using `n` or `next`, or step into functions on the line we're at using `s` or `step`. If you want to repeat this, you can simply press enter and the previous command will be repeated.

Let's step one line further and see what's inside our `chip` variable after calling `.first` on the animals `Vec`:

```bash
(gdb) n
28          println!("chip: {chip:?}");
(gdb) p chip
$2 = core::option::Option<&rust_gdb_example::Animal>::Some(0x5555555ae480)
(gdb) print *(0x5555555ae480 as &rust_gdb_example::Animal)
$3 = rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Chip", age: 4}
```

We execute `n` and we're on the next line (28). Here, we try to print `chip`, and we see it's an `Option` with a reference to an `Animal` inside. Unfortunately, GDB only shows us the address again; we need to cast the address to an `&rust_gdb_example::Animal` to see the actual values of the animal.

We can also print function definitions:

```bash
(gdb) p get_chip
$4 = {fn (&[rust_gdb_example::Animal])} 0x55555555d520 <basic::get_chip>
```

If we want to get to the end of this function and one step upward to the calling site, we can use `finish`. And if we’re done with our current break point, we can use `continue` or `c` to continue execution of the program — which, in this case, will simply run the program to its end:

```bash
(gdb) finish
Run till exit from #0  basic::get_chip (animals=&[rust_gdb_example::Animal](size=3) = {...}) at examples/basic.rs:28
chip: Some(Animal { kind: Cat, name: "Chip", age: 4 })
0x000055555555d4f1 in basic::main () at examples/basic.rs:22
22          get_chip(&animals);
(gdb) c
Continuing.
[Inferior 1 (process 1364722) exited normally]
```

## Manipulating state and watchpoints

First, let's create another example withing the `examples` folder within the [`nested.rs`](rust-gdb-example/examples/nested.rs) file.

Again, we're creating an animal list. But this time, we also create a `Person` and set the animals as their pets. Also, we print the person, set their age to `100` and reverse their name.

Before we can debug this program, we need to build it again and start `rust-gdb` with the binary:

```bash
cargo build --example nested
rust-gdb target/debug/examples/nested
```

Great, let's set breakpoints at line 22 and line 27 and run the program:

```bash
(gdb) b nested.rs:22
Breakpoint 1 at 0x94cd: file examples/nested.rs, line 22.
(gdb) b nested.rs:27
Breakpoint 2 at 0x9529: file examples/nested.rs, line 27.
(gdb) info b
Num     Type           Disp Enb Address            What
1       breakpoint     keep y   0x00000000000094cd in nested::main at examples/nested.rs:22
2       breakpoint     keep y   0x0000000000009529 in nested::main at examples/nested.rs:27
(gdb) r
Starting program: /home/kvmvkxnt/Work/rust-roadmap/env-setup/rust-repl/debug-with-gdb/rust-gdb-example/target/debug/examples/nested

This GDB supports auto-downloading debuginfo from the following URLs:
  <https://debuginfod.archlinux.org>
Enable debuginfod for this session? (y or [n]) y
Debuginfod has been enabled.
To make this setting permanent, add 'set debuginfod enabled on' to .gdbinit.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Breakpoint 1, nested::main () at examples/nested.rs:22
22          let mut some_person = Person {
```

We're at first breakpoint, where the person is created. Let's continue to print statement. Then, we'll set a so called watchpoint on `some_person.age`. This watchpoint will notify us every time `some_person.age` changes:

```bash
(gdb) c
Continuing.

Breakpoint 2, nested::main () at examples/nested.rs:27
27          println!("person: {some_person:?}");
(gdb) watch some_person.age
Hardware watchpoint 3: some_person.age
(gdb) n
person: Person { name: "Some", pets: [Animal { kind: Cat, name: "Chip", age: 4 }, Animal { kind: Cat, name: "Nacho", age: 6 }, Animal { kind: Dog, name: "Taco", age: 2 }], age: 24 }
28          some_person.age = 100;
(gdb)

Hardware watchpoint 3: some_person.age

Old value = 24
New value = 100
0x000055555555d5a5 in nested::main () at examples/nested.rs:28
28          some_person.age = 100;
```

Let's rerun the program by calling `run` again and confirming we want to rerun. This time, when we're at the second breakpoint, let's change the value manually using `set`:

```bash
(gdb) run
The program being debugged has been started already.
Start it from the beginning? (y or n) y
Starting program: /home/kvmvkxnt/Work/rust-roadmap/env-setup/rust-repl/debug-with-gdb/rust-gdb-example/target/debug/examples/nested
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Breakpoint 1, nested::main () at examples/nested.rs:22
22          let mut some_person = Person {
(gdb) c
Continuing.

Breakpoint 2, nested::main () at examples/nested.rs:27
27          println!("person: {some_person:?}");
(gdb) set some_person.age = 22
(gdb) p some_person
$1 = rust_gdb_example::Person {name: "Some", pets: Vec(size=3) = {rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Chip", age: 4},
    rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Nacho", age: 6},
    rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Dog, name: "Taco", age: 2}}, age: 22}
```

As you can see, we can use `set ...args` to manipulate the state of our variables. This works very well with primitives but gets more tricky with complex values, such as Rust standard library, or external crate types.

Another nice feature we can try is to execute functions and see what they return:

```bash
(gdb) p some_func("Hello")
$3 = "olleH"
(gdb) p some_func("Debug")
$4 = "gubeD"
(gdb) p some_func(some_person.name)
$5 = "emoS"
(gdb) set some_person.name = some_func(some_person.name)
(gdb) p some_person
$6 = rust_gdb_example::Person {name: "emoS", pets: Vec(size=3) = {rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Chip", age: 4}, rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Cat, name: "Nacho", age: 6},
    rust_gdb_example::Animal {kind: rust_gdb_example::AnimalType::Dog, name: "Taco", age: 2}}, age: 22}
```

## Debugging an async network application

Let's create [`tokio.rs`](rust-gdb-example/examples/tokio.rs) in the `examples` folder.

Let's compile the example and fire up `rust-gdb` with the resulting binary:

```bash
cargo build --example tokio
rust-gdb target/debug/examples/tokio
```

Let's set a breakpoint at the beginning of the process function at line 19:

```bash
(gdb) b tokio.rs:19
Breakpoint 1 at 0x27511: tokio.rs:19. (2 locations)
(gdb) info b
Num     Type           Disp Enb Address            What
1       breakpoint     keep y   <MULTIPLE>
1.1                         y   0x0000000000027511 in tokio::process at examples/tokio.rs:19
1.2                         y   0x0000000000032669 in tokio::process::{async_fn#0} at examples/tokio.rs:19
```

Interesting, the breakpoint is split up into `1.1` and `1.2`. These are called locations in GDB. This can happen due to optimizations, such as inlining, for example, where GDB will add a breakpoint at every point where the function is inlined or templated.

We can disable either location if we want, but it doesn't matter in this case. Let's run the program:

```bash
(gdb) r
Starting program: /home/kvmvkxnt/Work/rust-roadmap/env-setup/rust-repl/debug-with-gdb/rust-gdb-example/target/debug/examples/tokio

This GDB supports auto-downloading debuginfo from the following URLs:
  <https://debuginfod.archlinux.org>
Enable debuginfod for this session? (y or [n]) y
Debuginfod has been enabled.
To make this setting permanent, add 'set debuginfod enabled on' to .gdbinit.
Downloading 3.07 M separate debug info for /usr/lib/libm.so.6
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".
[New Thread 0x7ffff7c7b6c0 (LWP 1887157)]
[New Thread 0x7fffef9ff6c0 (LWP 1887158)]
[New Thread 0x7ffff7a7a6c0 (LWP 1887159)]
[New Thread 0x7ffff78796c0 (LWP 1887160)]
[New Thread 0x7ffff76786c0 (LWP 1887161)]
[New Thread 0x7ffff74776c0 (LWP 1887162)]
[New Thread 0x7ffff72766c0 (LWP 1887163)]
[New Thread 0x7ffff70756c0 (LWP 1887164)]
[New Thread 0x7ffff6e746c0 (LWP 1887165)]
[New Thread 0x7ffff6c736c0 (LWP 1887166)]
[New Thread 0x7ffff6a726c0 (LWP 1887167)]
[New Thread 0x7ffff68716c0 (LWP 1887168)]
[New Thread 0x7ffff66706c0 (LWP 1887169)]
[New Thread 0x7ffff646f6c0 (LWP 1887170)]
[New Thread 0x7ffff626b6c0 (LWP 1887171)]
[New Thread 0x7ffff60676c0 (LWP 1887172)]
[New Thread 0x7ffff5e636c0 (LWP 1887173)]
[New Thread 0x7ffff5c5f6c0 (LWP 1887174)]
[New Thread 0x7ffff5a5b6c0 (LWP 1887175)]
[New Thread 0x7ffff58576c0 (LWP 1887176)]
[New Thread 0x7ffff56536c0 (LWP 1887177)]
[New Thread 0x7ffff544f6c0 (LWP 1887178)]
[New Thread 0x7ffff52476c0 (LWP 1887179)]
[New Thread 0x7ffff50466c0 (LWP 1887180)]
Accepting TCP on port 8080
```

Our listener is up and running and we can even see the threads the Tokio runtime spawned in the background.

Let's send some data to the endpoint from another terminal using `netcat`:

```bash
nc 127.0.0.1 8080
```

This triggers our breakpoint in `process`:

```bash
[Switching to Thread 0x7ffff50466c0 (LWP 1887180)]

Thread 25 "tokio-runtime-w" hit Breakpoint 1.1, tokio::process (socket=...) at examples/tokio.rs:19
19      async fn process(mut socket: TcpStream) {
(gdb) p socket
$1 = tokio::net::tcp::stream::TcpStream {io: tokio::io::poll_evented::PollEvented<mio::net::tcp::stream::TcpStream> {io: core::option::Option<mio::net::tcp::stream::TcpStream>::Some(mio::
net::tcp::stream::TcpStream {inner: mio::io_source::IoSource<std::net::tcp::TcpStream> {state: mio::sys::unix::selector::stateless_io_source::IoSourceState, inner: std::net::tcp::TcpStrea
m (std::sys::net::connection::socket::TcpStream {inner: std::sys::net::connection::socket::unix::Socket (std::sys::fd::unix::FileDesc (std::os::fd::owned::OwnedFd {fd: core::num::niche_ty
pes::I32NotAllOnes (10)}))}), selector_id: mio::io_source::SelectorId {id: core::sync::atomic::AtomicUsize {v: core::cell::UnsafeCell<usize> {value: 1}}}}}), registration: tokio::runtime:
:io::registration::Registration {handle: tokio::runtime::scheduler::Handle::MultiThread(Arc(strong=126, weak=0) = {
          value = tokio::runtime::scheduler::multi_thread::handle::Handle {shared: tokio::runtime::scheduler::multi_thread::worker::Shared {remotes: tokio::runtime::scheduler::multi_threa
d::worker::Remote {steal: tokio::runtime::scheduler::multi_thread::queue::Steal<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>> (alloc::sy
nc::Arc<tokio::runtime::scheduler::multi_thread::queue::Inner<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>, alloc::alloc::Global>), unp
ark: tokio::runtime::scheduler::multi_thread::park::Unparker {inner: Arc(strong=2, weak=0) = {
                    value = tokio::runtime::scheduler::multi_thread::park::Inner {state: tokio::loom::std::atomic_usize::AtomicUsize {inner: core::cell::UnsafeCell<core::sync::atomic::Ato
micUsize> {value: core::sync::atomic::AtomicUsize {v: core::cell::UnsafeCell<usize> {value: 2}}}}, mutex: tokio::loom::std::parking_lot::Mutex<()> (core::marker::PhantomData<std::sync::po
ison::mutex::Mutex<()>>, lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ()> {raw: parking_lot::raw_mutex::RawMutex {state: core::sync::atomic::AtomicU8 {v: core::cell::UnsafeCel
l<u8> {value: 0}}}, data: core::cell::UnsafeCell<()> {value: ()}}), condvar: tokio::loom::std::parking_lot::Condvar (core::marker::PhantomData<std::sync::poison::condvar::Condvar>, parkin
g_lot::condvar::Condvar {state: core::sync::atomic::AtomicPtr<parking_lot::raw_mutex::RawMutex> {p: core::cell::UnsafeCell<*mut parking_lot::raw_mutex::RawMutex> {value: 0x0}}}), shared:
Arc(strong=24, weak=0) = {
                        value = tokio::runtime::scheduler::multi_thread::park::Shared {driver: tokio::util::try_lock::TryLock<tokio::runtime::driver::Driver> {locked: core::sync::atomic::
AtomicBool {v: core::cell::UnsafeCell<u8> {value: 1}}, data: core::cell::UnsafeCell<tokio::runtime::driver::Driver> {value: tokio::runtime::driver::Driver {inner: tokio::runtime::driver::
TimeDriver::Enabled{driver: tokio::runtime::time::Driver {park: tokio::runtime::driver::IoStack::Enabled(tokio::runtime::process::Driver {park: {...}, signal_handle: {...}})}}}}}},
                        strong = 24, weak = 0}}, strong = 2,
                    weak = 0}}}, inject: tokio::runtime::scheduler::inject::shared::Shared<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle, alloc::alloc::Global>>
 {len: tokio::loom::std::atomic_usize::AtomicUsize {inner: core::cell::UnsafeCell<core::sync::atomic::AtomicUsize> {value: core::sync::atomic::AtomicUsize {v: core::cell::UnsafeCell<usize
> {value: 0}}}}
...
{value: tokio::util::linked_list::PointersInner<tokio::runtime::io::scheduled_io::ScheduledIo> {prev: core::option::Option<core::ptr::non_null::NonNull<tokio::runtime::io::scheduled_io::ScheduledIo>>::None, next: core::option::Option<core::ptr::non_null::NonNull<tokio::runtime::io::scheduled_io::ScheduledIo>>::Some(core::ptr::non_null::NonNull<tokio::runtime::io::scheduled_io::ScheduledIo> {pointer: 0x5555556f5180}), _pin: core::marker::PhantomPinned}}}}, readiness: tokio::loom::std::atomic_usize::AtomicUsize {inner: core::cell::UnsafeCell<core::sync::atomic::AtomicUsize> {value: core::sync::atomic::AtomicUsize {v: core::cell::UnsafeCell<usize> {value: 65538}}}}, waiters: tokio::loom::std::parking_lot::Mutex<tokio::runtime::io::scheduled_io::Waiters> (core::marker::PhantomData<std::sync::poison::mutex::Mutex<tokio::runtime::io::scheduled_io::Waiters>>, lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, tokio::runtime::io::scheduled_io::Waiters> {raw: parking_lot::raw_mutex::RawMutex {state: core::sync::atomic::AtomicU8 {v: core::cell::UnsafeCell<u8> {value: 0}}}, data: core::cell::UnsafeCell<tokio::runtime::io::scheduled_io::Waiters> {value: tokio::runtime::io::scheduled_io::Waiters {list: tokio::util::linked_list::LinkedList<tokio::runtime::io::scheduled_io::Waiter, tokio::runtime::io::scheduled_io::Waiter> {head: core::option::Option<core::ptr::non_null::NonNull<tokio::runtime::io::scheduled_io::Waiter>>::None, tail: core::option::Option<core::ptr::non_null::NonNull<tokio::runtime::io::scheduled_io::Waiter>>::None, _marker: core::marker::PhantomData<*const tokio::runtime::io::scheduled_io::Waiter>}, reader: core::option::Option<core::task::wake::Waker>::None, writer: core::option::Option<core::task::wake::Waker>::None}}})}, strong = 2, weak = 0}}}}
(gdb) c
Continuing.

Thread 25 "tokio-runtime-w" hit Breakpoint 1.2, tokio::process::{async_fn#0} () at examples/tokio.rs:19
19      async fn process(mut socket: TcpStream) {
```

When the breakpoint is triggered, GDB notifies us that this happened in one of the runtime's spawned threads and that we have the `socket` variable, which we can inspect.

The `socket` is a Tokio TCPStream, but we can't really say much just from printing it. There's a file descriptor with the number `11` in there, which is the open network connection, but the rest seems to be Tokio and mio internals.

In any case, it worked - we successfully set a breakpoint in an async handler running in one of many threads. That means the same approach will work just as well if we have, for example, an Actix or warp web server running, setting a breakpoint in one of the handler functions, to inspect the incoming HTTP request data.

Here is the `Hello` response in our second terminal after we use `c` to continue execution:

```bash
nc 127.0.0.1 8080
Hello%
```
