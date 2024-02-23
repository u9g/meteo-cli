use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::{Datapoint, Tower, Vertex};

pub(super) fn at_tower<'a>(
    tower_name: &str,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    Box::new(std::iter::once(Vertex::Tower(Tower {
        tower_name: tower_name.to_owned(),
        tower_datapoints: vec![
            Datapoint {
                time: "12-9-23".into(),
                wind_speed_m_s: 19.5,
            },
            Datapoint {
                time: "10-10-22".into(),
                wind_speed_m_s: 17.32,
            },
        ],
    })))
}
