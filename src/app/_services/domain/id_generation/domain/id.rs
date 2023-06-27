#[derive(Clone)]
pub struct Id {
    value: String,
}

impl Id {
    pub fn new(value: String) -> Id {
        Id {
            value
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
