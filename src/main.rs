use std::io;

use crate::{
    input_handler::get_user_input,
    saber_color::{SaberColor, Trait},
};

mod input_handler;
mod saber_color;

fn main() -> Result<(), io::Error> {
    println!("Which trait do you most identify with? (power, peace, wisdom, defense)");

    let input = get_user_input()?;

    let trait_ = Trait::from_character(&input);

    let color = SaberColor::from_trait(&trait_);

    println!("Your recommended lightsaber color is: {}", color);

    Ok(())
}
