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
        .build();
}

fn callback() {
    let arg = Argument::new()
        .alias("test")
        .with_callback(|params| println!("Those are the parameters: {}", params))
        .build();
}