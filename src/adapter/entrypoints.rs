use std::collections::HashSet;

use itertools::Itertools;
use sqlx::Row;
use trustfall::provider::{ResolveInfo, VertexInfo, VertexIterator};

use crate::adapter::{
    helpers::optim::{filter_down_candidate_value_of_float, filter_down_edge, SelectAndFilter},
    vertex::Datapoint,
    Adapter,
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

            let wanted_fields = destination
                .required_properties()
                .into_iter()
                .map(|x| x.name)
                .collect::<HashSet<_>>();

            if wanted_fields.contains("fahrenheit") {
                sf.select
                    .insert("((tbl.airt_2m_avg - 32) * 5.0 / 9) as airc".to_owned());
            }

            if let Some(fahrenheit) = destination.statically_required_property("fahrenheit") {
                filter_down_candidate_value_of_float(fahrenheit, &mut sf, "airc")
            }

            if wanted_fields.contains("celsius") {
                sf.select.insert("airt_2m_avg".to_owned());
            }

            if let Some(celsius) = destination.statically_required_property("celsius") {
                filter_down_candidate_value_of_float(celsius, &mut sf, "airt_2m_avg");
            }
        },
        &mut select_and_filter,
    );

    filter_down_edge(
        &mut resolve_info.edges_with_name("wind_speed"),
        |edge, mut sf| {
            let destination = edge.destination();

            let wanted_fields = destination
                .required_properties()
                .into_iter()
                .map(|x| x.name)
                .collect::<HashSet<_>>();

            if wanted_fields.contains("meters_per_second") {
                sf.select.insert("ws_2m_avg".to_owned());
            }

            if let Some(celsius) = destination.statically_required_property("meters_per_second") {
                filter_down_candidate_value_of_float(celsius, &mut sf, "ws_2m_avg");
            }
        },
        &mut select_and_filter,
    );

    let query = format!(
        "select {} FROM {tower_name} tbl HAVING {} LIMIT 1",
        select_and_filter.select.iter().join(", "),
        select_and_filter.filter.join(" AND ")
    );
    // println!("wind_speed_meters_per_second outputted: {:#?}",);

    println!("\n{query}\n");

    let output = Adapter::runtime()
        .block_on(sqlx::query(&query).fetch_one(Adapter::pool()))
        .unwrap();

    let value: f32 = output.try_get("airc").unwrap();

    println!("value='{:#?}'", value);

    let datapoints = vec![
        Datapoint::make("12-9-23".into(), 19.5, 25., 30.),
        Datapoint::make("10-10-22".into(), 17.32, -29., -35.),
    ];

    Box::new(datapoints.into_iter().map(|x| Vertex::Datapoint(x)))
}
