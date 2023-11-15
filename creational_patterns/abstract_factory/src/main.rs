mod gui;
mod macos_gui;
mod render_ui;
mod win_gui;

use std::io;

use crate::render_ui::{render_dynamic_disaptch, render_static_dispatch};

fn main() {
    println!("Are you using windows?");
    println!("y / n");

    let mut user_answer = String::new();
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    let user_answer = user_answer.trim();

    println!("\nStatic dispatch:");
    match user_answer {
        "y" | "Y" => render_static_dispatch(win_gui::factory::Factory),
        "n" | "N" => render_static_dispatch(macos_gui::factory::Factory),
        _ => println!("Invalid input. Please enter 'y' for yes or 'n' for no."),
    }

    println!("\nDynamic dispatch:");
    let factory: &dyn gui::factory::AbstractFactoryDynamic = if user_answer == "y" {
        &win_gui::factory::Factory
    } else {
        &macos_gui::factory::Factory
    };

    render_dynamic_disaptch(factory)
}
