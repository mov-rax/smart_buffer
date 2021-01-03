# smart_buffer
A stack/heap buffer with const generics. No std needed.

## THIS CRATE REQUIRES USING NIGHTLY RUST.

## What is this?

smart_buffer provides a datatype that allows the creation of a memory structure that is split between the stack and the heap.

The size of the SmartBuffer's memory in the stack is defined at compile time with the usage of const generics. The total allowed size of the entire SmartBuffer can be determined
at runtime, where any additional required on runtime will be allocated in the heap.

An example of such is shown below:

```rust
let mut buf = SmartBuffer::<u8, 3>::new(5); // 3 elements on the stack, 2 on the heap
buf.push(3); // stack
buf.push(21); // stack
buf.push(100); // stack
buf.push(65); // heap
buf.push(21); // heap
buf.push(0); // not pushed, not enough space

buf[0] = 128; // modified on the stack
buf[4] = 40; // modified on the heap
```

To offer flexibility while using this crate, it is also possible to iterate through all values as if it was contiguous data structure.

```rust
let mut buf = SmartBuffer::<f64,128>::new(256); 
// code goes here
for elem in &buf{
  println!("Whoa: {}", elem);
}
```

However, using the `new()` function only supports types with traits `Copy` and `Clone`, which limits the types that can
be used.

Luckily, there is an included macro named `buf!` which can simply the creation of any SmartBuffer with types that
include the `Clone` trait!

An example of using the macro is shown below

```rust
#[macro_use]
fn some_function(){
    let mut buffer = buf!(String::new(), 2, 10); // Creates a SmartBuffer
    buffer.push(String::from("Wow, look at this")); // stack
    buffer.push(String::from("This is pretty nice, huh?")); // stack
    buffer.push(String::from("This is one nice heap!")); // heap
    buffer[1] = String::from("Yes it is!"); // heap
}
```

In the example above, the macro REQUIRES that the length of the stack (2 in the example) is known on compile time. The total
length of the SmartBuffer can be known at runtime!

