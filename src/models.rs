pub struct Clip {
    pub name: String,
    pub version: String,
}

impl Clip {
    pub fn new() -> Self {
        Self {
            name: String::from("CLIp"),
            version: String::from("0.1.0")
        }
    }
}