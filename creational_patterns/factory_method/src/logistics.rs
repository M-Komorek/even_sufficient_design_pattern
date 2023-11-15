pub trait Transport {
    fn deliver(&self);
}

pub trait Logistics {
    fn create_transport(&self) -> Box<dyn Transport>;

    fn plan_delivery(&self) {
        let transport = self.create_transport();
        transport.deliver();
    }
}
