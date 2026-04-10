mod models;   //  defined the module: models
mod commands; // defined the module: Commands

use std::io::{self, Write};

fn main() {
    let assistant = models::Clip::new();
    let mut running = true;

    println!("Welcome to {} v{}", assistant.name, assistant.version);

    while running {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }

        // Llamamos al manejador de comandos del otro archivo
        running = commands::handle(&assistant, &input);
    }
}