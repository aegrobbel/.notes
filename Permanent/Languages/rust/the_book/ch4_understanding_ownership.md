# 4 - Understanding Ownership

## Basics

Ownership is Rust's way of managing memory (on the stack and heap). These rules are enforced by the compiler.

Rules:
- Each value in Rust has an __owner__
- There can only be one owner at a time
- When the owner goes out of scope, the value is dropped (memory is deallocated)

## Using Strings as an Example of Heap Allocation

Because Strings are not primitives and therefore their size cannot be known at compile time, they won't be stored on the stack. (So a String, rather than a string literal, String vs str.)

It can be noted, that string literals ARE faster and more efficient, so should be prioritized when possible.

Rust handles memory by calling `drop` on a variable when the variable itself is out of scope, and never used again. This means we don't need a garbage collector, nor any explicit calls from the programmer to allocate or free memory. This is actually a concept used in C++ called _Resource Acquisition Is Initialization (RAII)_

We need to remember that there is a big difference between:

```rust
let x = 5;
let y = x;
```

and
 
```rust
let s1 = String::from("hello");
let s2 = s1;
```

The difference here, is that in example 1, because both integers are on the stack and a known size at compile time, we literallly copy over the value from x into y, so both variables are valid and equal to 5. However, the second example involves setting the ptr location in s2 to the location that s1 is pointing at. We are not copying over any data, rather, we are changing ownership of this String variable.

Also, the genius of allowing only 1 owner at any time for dynamic memory, is that we don't have to worry about deallocating the same memory twice (which results in an error in languages that don't protect against this) We also can be sure that any _automatic_ copying is inexpensive in terms of runtime performance.

Instead of this being a "shallow copy", which it really is similar to, we are actually performing what Rust calls a "move".

### Cloning Data

If we do want to perform a deep copy, we can use a method called `clone`. The usage is like:

```rust
let s1 = String::from("hi");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);
```

### Ownership and Functions

Passing variables to functions is very similar to the way we asign values to variables. It will either move or copy, depending on the type of variable and the annotation used.

Therefore, if we call a function and pass a variable into it that is not a copy, that variable is moved, and the caller function no longer has to call drop on it.

### Return values and scope

Returning values can also transfer ownership. So if we pass in a variable to a function that takes ownership, but then that function returns the same variable, we are effectively "givin it back" to the original function.

### References and Borrowing



















