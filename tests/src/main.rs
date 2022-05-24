use rusty_args::Argument;

fn main() {
    version();
    callback();
}

fn version() {
    let arg = Argument::new()
        .alias("-V")
        .alias("-v")
        .alias("--version")
        .set_callback(|params| println!("{}", params));
}

fn callback() {
    let arg = Argument::new()
        .alias("test")
        .set_callback(|params| println!("Those are the parameters: {}", params));
}