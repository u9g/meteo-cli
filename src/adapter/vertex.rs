#[derive(Debug, Clone)]

pub struct Celsius(pub f32);

#[derive(Debug, Clone)]
pub struct Fahrenheit(pub f32);

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Datapoint(Datapoint),
    Temperature((Celsius, Fahrenheit)),
}

impl Datapoint {
    pub fn make(
        time: String,
        wind_speed_meters_per_second: f32,
        temp_celsius: f32,
        temp_fahrenheit: f32,
    ) -> Self {
        Datapoint {
            time,
            wind_speed_meters_per_second,
            temp: (Celsius(temp_celsius), Fahrenheit(temp_fahrenheit)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Datapoint {
    pub time: String,
    pub wind_speed_meters_per_second: f32,
    pub temp: (Celsius, Fahrenheit),
}
