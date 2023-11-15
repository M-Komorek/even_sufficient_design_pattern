use crate::logistics::{Logistics, Transport};

pub struct Ship;

impl Transport for Ship {
    fn deliver(&self) {
        println!("I will deliver the goods by ship!");
    }
}

pub struct ShipLogistics;

impl Logistics for ShipLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship)
    }
}
