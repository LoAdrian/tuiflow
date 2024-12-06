#[derive(Clone)]

#[derive(PartialEq, Debug)]
pub struct Control {
    name: String,
    key: String,
}

impl<'a> Control {
    pub fn new(name: &str, key: &str) -> Self {
        Self {
            name: String::from(name),
            key: String::from(key),
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}