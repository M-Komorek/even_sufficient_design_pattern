use crate::logistics::{Logistics, Transport};

pub struct Truck;

impl Transport for Truck {
    fn deliver(&self) {
        println!("I will deliver the goods by truck!");
    }
}

pub struct RoadLogistics;

impl Logistics for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Truck)
    }
}
