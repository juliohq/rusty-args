pub struct ArgumentBuilder {
    aliases: Vec<String>,
}

pub struct Argument {
    aliases: Vec<String>,
}

impl ArgumentBuilder {
    pub fn build(&self) -> Argument {
        Argument {
            aliases: self.aliases.clone(),
        }
    }

    pub fn alias(mut self, alias: &str) -> ArgumentBuilder {
        self.aliases.push(alias.to_string());
        self
    }
}

impl Argument {
    pub fn new() -> ArgumentBuilder {
        ArgumentBuilder { aliases: vec![] }
    }
}
