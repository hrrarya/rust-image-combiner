use std::env::args;
fn get_nth( n: usize ) -> String {
    args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            image_1: get_nth(1),
            image_2: get_nth(2),
            output: get_nth(3),
        }
    }
}
