use std::cell::RefCell;

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Datapoint(Datapoint),
    Tower(Tower),
}
#[derive(Debug, Clone)]
pub struct Tower {
    pub tower_name: String,
    pub datapoint: RefCell<Option<Vec<Datapoint>>>,
}

#[derive(Debug, Clone)]
pub struct Datapoint {
    pub time: String,
    pub wind_speed_m_s: f32,
    pub temp_c: f32,
}
