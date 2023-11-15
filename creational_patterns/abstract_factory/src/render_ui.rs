use crate::gui::{
    elements::{AbstractButton, AbstractCheckbox, AbstractTextField},
    factory::{AbstractFactory, AbstractFactoryDynamic},
};

pub fn render_static_dispatch(factory: impl AbstractFactory) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();
    let textfield = factory.create_textfield();

    button.press();
    checkbox.switch();
    textfield.write();
}

pub fn render_dynamic_disaptch(factory: &dyn AbstractFactoryDynamic) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();
    let textfield = factory.create_textfield();

    button.press();
    checkbox.switch();
    textfield.write();
}
