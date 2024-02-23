#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Datapoints(()),
    Tower(Tower),
    Datapoint(Datapoint),
}

#[derive(Debug, Clone)]
pub struct Tower {
    pub tower_name: String,
    pub tower_datapoints: Vec<Datapoint>,
}

#[derive(Debug, Clone)]
pub struct Datapoint {
    pub time: String,
    pub wind_speed_m_s: f64,
}
