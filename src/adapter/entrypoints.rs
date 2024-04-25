use std::collections::HashSet;

use itertools::Itertools;
use sqlx::{types::chrono::NaiveDateTime, Row};
use trustfall::provider::{ResolveInfo, VertexInfo, VertexIterator};

use crate::adapter::{
    helpers::optim::{filter_down_candidate_value_of_float, filter_down_edge, SelectAndFilter},
    vertex::{Celsius, Datapoint, Fahrenheit},
    Adapter,
};

use super::vertex::Vertex;

const TIMESTAMP_FIELD_NAME: &str = "datetime";

pub(super) fn datapoint<'a>(
    tower_name: &str,
    resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    let mut select_and_filter = SelectAndFilter::default();

    let mut temp_exists_in_query = false;
    let mut wind_speed_exists_in_query = false;
    let mut date_exists_in_query = false;

    let top_level_wanted_fields = resolve_info
        .required_properties()
        .into_iter()
        .map(|x| x.name)
        .collect::<HashSet<_>>();

    if top_level_wanted_fields.contains("time") {
        date_exists_in_query = true;
        select_and_filter
            .select
            .insert(TIMESTAMP_FIELD_NAME.to_owned());
    }

    filter_down_edge(
        &mut resolve_info.edges_with_name("temp"),
        &mut |edge, mut sf| {
            let destination = edge.destination();

            let wanted_fields = destination
                .required_properties()
                .into_iter()
                .map(|x| x.name)
                .collect::<HashSet<_>>();

            if wanted_fields.contains("fahrenheit") {
                sf.select
                    .insert("((tbl.airt_2m_avg * 9 / 5.0)+32) as airf".to_owned());
                sf.select.insert("airt_2m_avg".to_owned());
                temp_exists_in_query = true;
            }

            if let Some(fahrenheit) = destination.statically_required_property("fahrenheit") {
                filter_down_candidate_value_of_float(fahrenheit, &mut sf, "airf")
            }

            if wanted_fields.contains("celsius") {
                sf.select
                    .insert("((tbl.airt_2m_avg - 32) * 5.0 / 9) as airc".to_owned());
                sf.select.insert("airt_2m_avg".to_owned());
                temp_exists_in_query = true;
            }

            if let Some(celsius) = destination.statically_required_property("celsius") {
                filter_down_candidate_value_of_float(celsius, &mut sf, "airc");
            }
        },
        &mut select_and_filter,
    );

    filter_down_edge(
        &mut resolve_info.edges_with_name("wind_speed"),
        &mut |edge, mut sf| {
            let destination = edge.destination();

            let wanted_fields = destination
                .required_properties()
                .into_iter()
                .map(|x| x.name)
                .collect::<HashSet<_>>();

            if wanted_fields.contains("meters_per_second") {
                sf.select.insert("ws_2m_avg".to_owned());
                wind_speed_exists_in_query = true;
            }

            if let Some(celsius) = destination.statically_required_property("meters_per_second") {
                filter_down_candidate_value_of_float(celsius, &mut sf, "ws_2m_avg");
            }
        },
        &mut select_and_filter,
    );

    let query = format!(
        "select {} FROM {tower_name} tbl {}{} LIMIT 10",
        select_and_filter.select.iter().join(", "),
        if !select_and_filter.filter.is_empty() {
            "HAVING "
        } else {
            ""
        },
        select_and_filter.filter.join(" AND ")
    );
    // println!("wind_speed_meters_per_second outputted: {:#?}",);

    println!("\n{query}\n");

    let output = Adapter::runtime()
        .block_on(sqlx::query(&query).fetch_all(Adapter::pool()))
        .unwrap();

    Box::new(
        output
            .into_iter()
            .map(move |row| {
                let mut datapoint = Datapoint::default();
                if temp_exists_in_query {
                    let airf: f32 = row.try_get("airf").unwrap();
                    let airc: f32 = row.try_get("airt_2m_avg").unwrap();

                    datapoint.temp = Some((Celsius(airc), Fahrenheit(airf)));
                }
                if wind_speed_exists_in_query {
                    let wind_speed_meters_per_second: f32 = row.try_get("ws_2m_avg").unwrap();
                    datapoint.wind_speed_meters_per_second = Some(wind_speed_meters_per_second);
                }
                if date_exists_in_query {
                    let date: NaiveDateTime = row.try_get(TIMESTAMP_FIELD_NAME).unwrap();
                    datapoint.time = Some(date.to_string());
                }
                datapoint
            })
            .map(|x| Vertex::Datapoint(x)),
    )
}
