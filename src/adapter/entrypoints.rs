use std::fmt::format;

use itertools::Itertools;
use trustfall::provider::{ResolveInfo, VertexInfo, VertexIterator};

use crate::adapter::vertex::Datapoint;

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
    match wind_speed_meters_per_second {
        trustfall::provider::CandidateValue::Impossible => {}
        trustfall::provider::CandidateValue::Single(single) => {
            let value_as_string = convert_field_value_to_string(&single);
            filter.push(format!("ws_2m_avg_hourly = {}", value_as_string));
            select.push("ws_2m_avg_hourly");
        }
        trustfall::provider::CandidateValue::Multiple(multiple) => {
            filter.push(format!(
                "({})",
                multiple
                    .iter()
                    .map(|x| format!("ws_2m_avg_hourly = {}", convert_field_value_to_string(x)))
                    .join(" OR ")
            ));
            select.push("ws_2m_avg_hourly");
        }
        trustfall::provider::CandidateValue::Range(range) => {
            // range.
            match range.start_bound() {
                std::ops::Bound::Included(included) => filter.push(format!(
                    "ws_2m_avg_hourly >= {}",
                    convert_field_value_to_string(included)
                )),
                std::ops::Bound::Excluded(excluded) => filter.push(format!(
                    "ws_2m_avg_hourly > {}",
                    convert_field_value_to_string(excluded)
                )),
                std::ops::Bound::Unbounded => {}
            }
            match range.end_bound() {
                std::ops::Bound::Included(included) => filter.push(format!(
                    "ws_2m_avg_hourly <= {}",
                    convert_field_value_to_string(included)
                )),
                std::ops::Bound::Excluded(excluded) => filter.push(format!(
                    "ws_2m_avg_hourly < {}",
                    convert_field_value_to_string(excluded)
                )),
                std::ops::Bound::Unbounded => {}
            }
            select.push("ws_2m_avg_hourly");
        }
        trustfall::provider::CandidateValue::All => {
            select.push("ws_2m_avg_hourly");
        }
        _ => todo!(),
    }
    println!(
        "select {} FROM my_table WHERE {}",
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
