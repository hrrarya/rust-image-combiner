#[derive(Debug)]
pub struct Human {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
}

impl Human {
    pub fn new() ->Self {
        Human {
            first_name: String::new(),
            last_name: String::new(),
            address: String::new(),
        }
    }
}