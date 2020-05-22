use extend::ext;

#[ext(pub, name = Persistable)]
impl<T: Default> T {
    fn persist_state(&self) {}
}

fn main() {
    let _: Box<dyn Persistable> = todo!();
}
