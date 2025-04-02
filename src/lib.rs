use std::io;

use starship_battery::Battery;
use starship_battery::Manager;

pub fn battery_info() -> Result<Battery, io::Error> {
    let mng: Manager = Manager::new().map_err(io::Error::other)?;
    let mut ball = mng.batteries().map_err(io::Error::other)?;
    let obat: Option<Result<Battery, _>> = ball.next();
    let rbat: Result<Battery, _> = obat.ok_or_else(|| io::Error::other("no battery found"))?;
    rbat.map_err(io::Error::other)
}

pub fn print_battery(b: &Battery) -> Result<(), io::Error> {
    let sb: SimpleBattery = b.into();
    serde_json::to_writer(io::stdout().lock(), &sb)?;
    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct SimpleBattery {
    pub capacity: f32,
    pub temperature: Option<f32>,
    pub battery_level: f32,
    pub cycle_count: Option<u32>,
}

impl From<&Battery> for SimpleBattery {
    fn from(b: &Battery) -> Self {
        let capacity: f32 = b.state_of_health().into();
        let temperature: Option<f32> = b.temperature().map(|t| t.value);
        let battery_level: f32 = b.state_of_charge().into();
        let cycle_count: Option<u32> = b.cycle_count();
        Self {
            capacity,
            temperature,
            battery_level: battery_level * 100.0,
            cycle_count,
        }
    }
}
