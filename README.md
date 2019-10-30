# extend

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

## How does it work?

Under the hood it generates a trait with methods in your `impl` and implements those for the
type you specify. The code shown above expands roughly to:

```rust
trait VecExt<T: Ord> {
    fn sorted(mut self) -> Self;
}

impl<T: Ord> VecExt<T> for Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
```

## Configuration

You can configure:

- The visibility of the trait. The default visibility is private. Example: `#[ext(pub)]`. This
must be the first argument to the attribute
- The name of the generated extension trait. Example: `#[ext(name = MyExt)]`.
- Whether or not the generated trait should be [sealed]. Example: `#[ext(sealed = false)]`. The
default is `true`.

[sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed

More examples:

```rust
use extend::ext;

#[ext(name = SortedVecExt)]
impl<T: Ord> Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

#[ext(pub(crate))]
impl i32 {
    fn double(self) -> i32 {
        self * 2
    }
}

#[ext(pub, name = ResultSafeUnwrapExt)]
impl<T> Result<T, std::convert::Infallible> {
    fn safe_unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(_) => unreachable!(),
        }
    }
}
```

[extension traits]: https://dev.to/matsimitsu/extending-existing-functionality-in-rust-with-traits-in-rust-3622

License: MIT
