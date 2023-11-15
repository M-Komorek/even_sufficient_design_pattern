use crate::gui::elements::{AbstractButton, AbstractCheckbox, AbstractTextField};

pub struct Button;

impl AbstractButton for Button {
    fn press(&self) {
        println!("MacOs Button pressed!");
    }
}

pub struct Checkbox;

impl AbstractCheckbox for Checkbox {
    fn switch(&self) {
        println!("MacOs Checkbox switched!")
    }
}

pub struct TextField;

impl AbstractTextField for TextField {
    fn write(&self) {
        println!("MacOs TextField input saved!")
    }
}
