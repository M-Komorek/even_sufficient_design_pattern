use crate::gui::elements::{AbstractButton, AbstractCheckbox, AbstractTextField};
use crate::gui::factory::{AbstractFactory, AbstractFactoryDynamic};
use crate::macos_gui::elements::{Button, Checkbox, TextField};

pub struct Factory;

impl AbstractFactory for Factory {
    type B = Button;
    type C = Checkbox;
    type T = TextField;

    fn create_button(&self) -> Self::B {
        Button {}
    }

    fn create_checkbox(&self) -> Self::C {
        Checkbox {}
    }

    fn create_textfield(&self) -> Self::T {
        TextField {}
    }
}

impl AbstractFactoryDynamic for Factory {
    fn create_button(&self) -> Box<dyn AbstractButton> {
        Box::new(Button)
    }

    fn create_checkbox(&self) -> Box<dyn AbstractCheckbox> {
        Box::new(Checkbox)
    }

    fn create_textfield(&self) -> Box<dyn AbstractTextField> {
        Box::new(TextField)
    }
}
