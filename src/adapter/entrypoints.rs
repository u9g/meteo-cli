use itertools::Itertools;
use trustfall::provider::{ResolveInfo, VertexInfo, VertexIterator};

use crate::adapter::{
    helpers::optim::{filter_down_candidate_value_of_float, filter_down_edge, SelectAndFilter},
    vertex::Datapoint,
};

use super::vertex::Vertex;

pub(super) fn datapoint<'a>(
    tower_name: &str,
    resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    let mut select_and_filter = SelectAndFilter::default();

    filter_down_edge(
        &mut resolve_info.edges_with_name("temp"),
        |edge, mut sf| {
            let destination = edge.destination();
            if let Some(fahrenheit) = destination.statically_required_property("fahrenheit") {
                filter_down_candidate_value_of_float(
                    fahrenheit,
                    &mut sf,
                    "airc",
                    "((tbl.airt_2m_avg - 32) * 5.0 / 9) as airc".to_owned(),
                )
            }
            if let Some(celsius) = destination.statically_required_property("celsius") {
                filter_down_candidate_value_of_float(
                    celsius,
                    &mut sf,
                    "airt_2m_avg",
                    "airt_2m_avg".to_owned(),
                );
            }
        },
        &mut select_and_filter,
    );

    filter_down_edge(
        &mut resolve_info.edges_with_name("wind_speed"),
        |edge, mut sf| {
            let destination = edge.destination();
            if let Some(celsius) = destination.statically_required_property("meters_per_second") {
                filter_down_candidate_value_of_float(
                    celsius,
                    &mut sf,
                    "ws_2m_avg",
                    "ws_2m_avg".to_owned(),
                );
            }
        },
        &mut select_and_filter,
    );

    println!(
        "select {} FROM {tower_name} tbl WHERE {}",
        select_and_filter.select.iter().join(", "),
        select_and_filter.filter.join(" AND ")
    );
    // println!("wind_speed_meters_per_second outputted: {:#?}",);
    let datapoints = vec![
        Datapoint::make("12-9-23".into(), 19.5, 25., 30.),
        Datapoint::make("10-10-22".into(), 17.32, -29., -35.),
    ];

    Box::new(datapoints.into_iter().map(|x| Vertex::Datapoint(x)))
}
