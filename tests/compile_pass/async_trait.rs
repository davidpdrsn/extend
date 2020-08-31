use extend::ext;
use async_trait::async_trait;

#[ext]
#[async_trait]
impl String {
    async fn foo() -> usize {
        1
    }
}

fn main() {}
