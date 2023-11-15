mod logistics;
mod road_logistics;
mod sea_logistics;

use crate::logistics::Logistics;
use std::io;

fn main() {
    println!("Is the shipment intercontinental? (Way through the sea/ocean)");
    println!("y / n");

    let mut user_answer = String::new();
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    let answer = user_answer.trim();

    match answer {
        "y" | "Y" => {
            let logistics = sea_logistics::ShipLogistics;
            logistics.plan_delivery();
        }
        "n" | "N" => {
            let logistics = road_logistics::RoadLogistics;
            logistics.plan_delivery();
        }
        _ => {
            println!("Invalid input. Please enter 'y' for yes or 'n' for no.");
        }
    }
}
