use robotcyb_core::modbus::EnergyPolicy;
fn main() {
    println!("{:?}", EnergyPolicy::from_battery_percent(20.0));
}
