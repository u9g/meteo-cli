use itertools::Itertools;
use trustfall::{provider::CandidateValue, FieldValue};

pub fn convert_field_value_to_string(field_value: &FieldValue) -> String {
    match field_value {
        trustfall::FieldValue::Float64(f) => f.to_string(),
        trustfall::FieldValue::Uint64(uint) => uint.to_string(),
        trustfall::FieldValue::Int64(int) => int.to_string(),
        _ => todo!(),
    }
}

pub fn filter_down_candidate_value_of_float(
    candidate_value: CandidateValue<FieldValue>,
    select: &mut Vec<String>,
    filter: &mut Vec<String>,
    name_of_outputted_field: &str,
    select_string: Option<String>,
) {
    let select_string = select_string.unwrap_or_else(|| name_of_outputted_field.to_owned());

    match candidate_value {
        trustfall::provider::CandidateValue::Impossible => {}
        trustfall::provider::CandidateValue::Single(single) => {
            let value_as_string = convert_field_value_to_string(&single);
            filter.push(format!("{name_of_outputted_field} = {}", value_as_string));
            select.push(select_string);
        }
        trustfall::provider::CandidateValue::Multiple(multiple) => {
            filter.push(format!(
                "({})",
                multiple
                    .iter()
                    .map(|x| format!(
                        "{name_of_outputted_field} = {}",
                        convert_field_value_to_string(x)
                    ))
                    .join(" OR ")
            ));
            select.push(select_string);
        }
        trustfall::provider::CandidateValue::Range(range) => {
            // range.
            match range.start_bound() {
                std::ops::Bound::Included(included) => filter.push(format!(
                    "{name_of_outputted_field} >= {}",
                    convert_field_value_to_string(included)
                )),
                std::ops::Bound::Excluded(excluded) => filter.push(format!(
                    "{name_of_outputted_field} > {}",
                    convert_field_value_to_string(excluded)
                )),
                std::ops::Bound::Unbounded => {}
            }
            match range.end_bound() {
                std::ops::Bound::Included(included) => filter.push(format!(
                    "{name_of_outputted_field} <= {}",
                    convert_field_value_to_string(included)
                )),
                std::ops::Bound::Excluded(excluded) => filter.push(format!(
                    "{name_of_outputted_field} < {}",
                    convert_field_value_to_string(excluded)
                )),
                std::ops::Bound::Unbounded => {}
            }
            select.push(select_string);
        }
        trustfall::provider::CandidateValue::All => {
            select.push(select_string);
        }
        _ => todo!(),
    }
}
