pub struct Argument<F: Fn(String)> {
    aliases: Vec<String>,
    callback: Option<F>
}

impl<F: Fn(String)> Argument<F> {
    pub fn new() -> Argument<F> {
        Argument {
            aliases: vec![],
            callback: None
        }
    }
    
    pub fn alias(mut self, alias: &'static str) -> Self {
        self.aliases.push(alias.to_string());
        self
    }
    
    pub fn set_callback(mut self, cb: F) -> Self {
        self.callback = Some(cb);
        self
    }
}
