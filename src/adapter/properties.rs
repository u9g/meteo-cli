use trustfall::{
    provider::{
        field_property, resolve_property_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveInfo,
    },
    FieldValue,
};

use super::vertex::Vertex;

pub(super) fn resolve_datapoint_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "temp_celsius" => resolve_property_with::<Vertex, _>(contexts, |v| {
            FieldValue::Float64(v.as_datapoint().unwrap().temp_c.into())
        }),
        "time" => resolve_property_with::<Vertex, _>(contexts, field_property!(as_datapoint, time)),
        "wind_speed_meters_per_second" => resolve_property_with::<Vertex, _>(contexts, |v| {
            FieldValue::Float64(v.as_datapoint().unwrap().wind_speed_m_s.into())
        }),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Datapoint'"
            )
        }
    }
}
