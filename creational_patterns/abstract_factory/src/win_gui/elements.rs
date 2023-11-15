use crate::gui::elements::{AbstractButton, AbstractCheckbox, AbstractTextField};

pub struct Button;

impl AbstractButton for Button {
    fn press(&self) {
        println!("Windows Button pressed!");
    }
}

pub struct Checkbox;

impl AbstractCheckbox for Checkbox {
    fn switch(&self) {
        println!("Windows Checkbox switched!")
    }
}

pub struct TextField;

impl AbstractTextField for TextField {
    fn write(&self) {
        println!("Windows TextField input saved!")
    }
}
