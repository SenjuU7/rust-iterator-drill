# Rust Iterators Learning Project

This Rust project demonstrates the use of various iterator methods for data manipulation and processing. It's designed as a learning resource for understanding Rust's iterator patterns, including mapping, filtering, folding, and parsing.

## Overview

The project contains several tasks that showcase different iterator operations:

- **Task 1**: Using `map` to transform each element in a collection
- **Task 2**: Using `filter` to select elements based on a condition
- **Task 3**: Combining `filter` and `map` for chained operations
- **Task 4**: Using `fold` for accumulation operations (sum and product)
- **Task 5**: Using `filter_map` for parsing and filtering in one step

## Key Iterator Methods Demonstrated

### `map`
Transforms each element in an iterator using a closure. Returns a new iterator with the transformed values.

```rust
let doubled: Vec<i32> = vec![1, 2, 3].iter().map(|x| x * 2).collect();
// Result: [2, 4, 6]
```

### `filter`
Selects elements that match a predicate. Only elements for which the closure returns `true` are kept.

```rust
let evens: Vec<&i32> = vec![1, 2, 3, 4].iter().filter(|x| *x % 2 == 0).collect();
// Result: [2, 4]
```

### `fold`
Accumulates values from an iterator into a single result using an initial value and a combining function.

```rust
let sum = vec![1, 2, 3, 4].iter().fold(0, |acc, x| acc + x);
// Result: 10
```

### `filter_map`
Combines filtering and mapping in one operation. Useful for operations that might fail (like parsing).

```rust
let numbers: Vec<i32> = vec!["1", "2", "abc", "4"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();
// Result: [1, 2, 4]
```

## Running the Project

1. Ensure you have Rust installed
2. Clone or navigate to the project directory
3. Run `cargo run` to execute all tasks

## Learning Goals

- Understand lazy evaluation in iterators
- Learn chaining iterator methods
- Practice working with closures and borrowing
- Explore error handling with `Result` and `Option` in iterators

Each task includes print statements to show the results of the iterator operations.