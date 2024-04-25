use std::collections::HashSet;

use itertools::Itertools;
use trustfall::{
    provider::{CandidateValue, EdgeInfo},
    FieldValue,
};

pub fn convert_field_value_to_string(field_value: &FieldValue) -> String {
    match field_value {
        trustfall::FieldValue::Float64(f) => f.to_string(),
        trustfall::FieldValue::Uint64(uint) => uint.to_string(),
        trustfall::FieldValue::Int64(int) => int.to_string(),
        _ => todo!(),
    }
}

#[derive(Default, Debug)]
pub struct SelectAndFilter {
    pub select: HashSet<String>,
    pub filter: Vec<String>,
}

pub fn filter_down_candidate_value_of_float(
    candidate_value: CandidateValue<FieldValue>,
    select_and_filter: &mut SelectAndFilter,
    name_of_outputted_field: &str,
    select_string: String,
) {
    let select_string = select_string;

    match candidate_value {
        trustfall::provider::CandidateValue::Impossible => {}
        trustfall::provider::CandidateValue::Single(single) => {
            let value_as_string = convert_field_value_to_string(&single);
            select_and_filter
                .filter
                .push(format!("{name_of_outputted_field} = {}", value_as_string));
            select_and_filter.select.insert(select_string);
        }
        trustfall::provider::CandidateValue::Multiple(multiple) => {
            select_and_filter.filter.push(format!(
                "({})",
                multiple
                    .iter()
                    .map(|x| format!(
                        "{name_of_outputted_field} = {}",
                        convert_field_value_to_string(x)
                    ))
                    .join(" OR ")
            ));
            select_and_filter.select.insert(select_string);
        }
        trustfall::provider::CandidateValue::Range(range) => {
            // range.
            match range.start_bound() {
                std::ops::Bound::Included(included) => select_and_filter.filter.push(format!(
                    "{name_of_outputted_field} >= {}",
                    convert_field_value_to_string(included)
                )),
                std::ops::Bound::Excluded(excluded) => select_and_filter.filter.push(format!(
                    "{name_of_outputted_field} > {}",
                    convert_field_value_to_string(excluded)
                )),
                std::ops::Bound::Unbounded => {}
            }
            match range.end_bound() {
                std::ops::Bound::Included(included) => select_and_filter.filter.push(format!(
                    "{name_of_outputted_field} <= {}",
                    convert_field_value_to_string(included)
                )),
                std::ops::Bound::Excluded(excluded) => select_and_filter.filter.push(format!(
                    "{name_of_outputted_field} < {}",
                    convert_field_value_to_string(excluded)
                )),
                std::ops::Bound::Unbounded => {}
            }
            select_and_filter.select.insert(select_string);
        }
        trustfall::provider::CandidateValue::All => {
            select_and_filter.select.insert(select_string);
        }
        _ => todo!(),
    }
}

pub fn filter_down_edge(
    edges: &mut dyn Iterator<Item = EdgeInfo>,
    filter_down_values: fn(EdgeInfo, &mut SelectAndFilter),
    select_and_filter: &mut SelectAndFilter,
) {
    let mut filter = vec![];

    edges
        .map(|edge| {
            let mut select_and_filter = SelectAndFilter::default();
            filter_down_values(edge, &mut select_and_filter);
            select_and_filter
        })
        .for_each(|x| {
            select_and_filter.select.extend(x.select);
            if !x.filter.is_empty() {
                filter.push(format!("({})", x.filter.join(" AND ")));
            }
        });

    if !filter.is_empty() {
        select_and_filter
            .filter
            .push(format!("({})", filter.join(" OR ")));
    }
}
