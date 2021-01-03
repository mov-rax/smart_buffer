# smart_buffer
A stack/heap buffer with const generics.

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
```
