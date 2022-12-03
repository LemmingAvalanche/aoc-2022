See: https://adventofcode.com/2022

# Resources

- https://docs.rs/inpt/latest/inpt/

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
