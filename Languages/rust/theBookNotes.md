# 3 - Common Programming Concepts

## Variables and Mutability

Rust requires the `mut` keyword if the variable is meant to be mutated

__Constants__ are declared with `const` instead of `let`, and must include the type annotation. They should be written as all uppercase with underscores.

__Shadowing__ is when we use the same variable name that was previously declared to denote a new variable value, ie 
```rust
let x = 5;
let x = "five";
```

### Data Types

We have two main types of primitive data, scalar and compound.

__Scalar__ types represent a single value, and can be integers, floating-point nums, booleans, and characters.

__Compound__ types represent a group of multiple values. These can be either tuples or arrays.

__Tuples__ group together a number of values into one compound type. They have a fixed length.
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
// or!
let x = tup;
let five_hundred = x.0;
```
Another thing of note, is that a tuple w/o values is considred a 'unit', written `()`. It is used to represent an empty value, and are the return value of a function or expression if they don't have one defined.

__Arrays__ must all have the same type for their elements, and also have a fixed length.