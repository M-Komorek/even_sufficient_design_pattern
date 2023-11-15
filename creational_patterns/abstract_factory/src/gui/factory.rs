use crate::gui::elements::{AbstractButton, AbstractCheckbox, AbstractTextField};

pub trait AbstractFactory {
    type B: AbstractButton;
    type C: AbstractCheckbox;
    type T: AbstractTextField;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
    fn create_textfield(&self) -> Self::T;
}

pub trait AbstractFactoryDynamic {
    fn create_button(&self) -> Box<dyn AbstractButton>;
    fn create_checkbox(&self) -> Box<dyn AbstractCheckbox>;
    fn create_textfield(&self) -> Box<dyn AbstractTextField>;
}
