# extend

[API documentation](https://docs.rs/extend/0.2.1/extend/)

Create extensions for types you don't own with [extension traits] but without the boilerplate.

Example:

```rust
use extend::ext;

#[ext]
impl<T: Ord> Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

fn main() {
    assert_eq!(
        vec![1, 2, 3],
        vec![2, 3, 1].sorted(),
    );
}
```

[extension traits]: https://dev.to/matsimitsu/extending-existing-functionality-in-rust-with-traits-in-rust-3622
