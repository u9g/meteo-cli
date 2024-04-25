use trustfall::provider::{ResolveInfo, VertexInfo, VertexIterator};

use crate::adapter::vertex::{Datapoint, Tower};

use super::vertex::Vertex;

pub(super) fn at_tower<'a>(
    tower_name: &str,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    println!(
        "wind_speed_meters_per_second outputted: {:#?}",
        _resolve_info
            .query()
            .statically_required_property("wind_speed_meters_per_second")
            .unwrap()
    );
    Box::new(std::iter::once(Vertex::Tower(Tower {
        tower_name: tower_name.to_owned(),
        datapoint: Some(vec![
            Datapoint {
                time: "12-9-23".into(),
                wind_speed_m_s: 19.5,
                temp_c: 25.,
            },
            Datapoint {
                time: "10-10-22".into(),
                wind_speed_m_s: 17.32,
                temp_c: -29.,
            },
        ])
        .into(),
    })))
}
