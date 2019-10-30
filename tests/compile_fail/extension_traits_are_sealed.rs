use extend::ext;

#[ext(name = Foo)]
impl i32 {
    fn foo() {}
}

impl Foo for i64 {
    fn foo() {}
}

fn main() {}
