See: https://adventofcode.com/2022

# Resources

- https://docs.rs/inpt/latest/inpt/
- Bit operators:
  - https://stackoverflow.com/questions/51571066/what-are-the-exact-semantics-of-rusts-shift-operators
  - https://en.wikipedia.org/wiki/Logical_shift
  - https://en.wikipedia.org/wiki/Arithmetic_shift

# Snippets and common problems

## `lines()`

I often have problems with this iterator.

Collecting into a `Vec` can help. Then you can use slice iterators,
which tend to be more forgiving.

``` rust

        .lines()
        [what is inbetween—probably one or more `map(…)`]
        .collect::<Vec<u32>>();
```

### `lines()` into numbers

``` rust
[…]
        .lines()
        .map(|s| s.parse::<u32>().expect("not an integer"));
```
