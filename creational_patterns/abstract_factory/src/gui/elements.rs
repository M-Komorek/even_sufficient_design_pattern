pub trait AbstractButton {
    fn press(&self);
}

pub trait AbstractCheckbox {
    fn switch(&self);
}

pub trait AbstractTextField {
    fn write(&self);
}
