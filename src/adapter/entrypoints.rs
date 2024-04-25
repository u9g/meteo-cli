use std::fmt::format;

use itertools::Itertools;
use trustfall::provider::{ResolveInfo, VertexInfo, VertexIterator};

use crate::adapter::{helpers::optim::filter_down_candidate_value_of_float, vertex::Datapoint};

use super::{helpers::optim::convert_field_value_to_string, vertex::Vertex};

pub(super) fn datapoint<'a>(
    tower_name: &str,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    let wind_speed_meters_per_second = _resolve_info
        .statically_required_property("wind_speed_meters_per_second")
        .unwrap();
    let mut select = vec![];
    let mut filter = vec![];

    filter_down_candidate_value_of_float(
        wind_speed_meters_per_second,
        &mut select,
        &mut filter,
        "ws_2m_avg_hourly",
        None,
    );

    println!(
        "select {} FROM {tower_name} WHERE {}",
        select.join(", "),
        filter.join(" AND ")
    );
    // println!("wind_speed_meters_per_second outputted: {:#?}",);
    let datapoints = vec![
        Datapoint::make("12-9-23".into(), 19.5, 25., 30.),
        Datapoint::make("10-10-22".into(), 17.32, -29., -35.),
    ];

    Box::new(datapoints.into_iter().map(|x| Vertex::Datapoint(x)))
}
