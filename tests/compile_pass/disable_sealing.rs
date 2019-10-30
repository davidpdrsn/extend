use ext::ext;

#[ext(name = Foo, sealed = false)]
impl i32 {
    fn foo() {}
}

impl Foo for i64 {
    fn foo() {}
}

fn main() {}
