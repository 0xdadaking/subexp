use std::fmt::Display;

pub trait TrafficLightDuration {
    fn duration(&self) -> u32;
}

pub enum TrafficLight {
    Red(u32),
    Green(u32),
    Yellow(u32),
}

impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red(d) => *d,
            TrafficLight::Green(d) => *d,
            TrafficLight::Yellow(d) => *d,
        }
    }
}

impl Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrafficLight::Red(d) => write!(f, "Red({})", d),
            TrafficLight::Green(d) => write!(f, "Green({})", d),
            TrafficLight::Yellow(d) => write!(f, "Yellow({})", d),
        }
    }
}

fn main() {
    let traffic_lights = vec![
        TrafficLight::Red(60),
        TrafficLight::Green(90),
        TrafficLight::Yellow(3),
    ];
    
    for tl in &traffic_lights {
        println!("traffic light: {}", tl);
    }
}
