use trustfall::{
    provider::{
        field_property, resolve_property_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveInfo,
    },
    FieldValue,
};

use super::vertex::Vertex;

pub(super) fn resolve_datapoints_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "time" => resolve_property_with::<Vertex, _>(contexts, field_property!(as_datapoint, time)),
        "wind_speed_meters_per_second" => resolve_property_with::<Vertex, _>(
            contexts,
            field_property!(as_datapoint, wind_speed_m_s, {
                FieldValue::Float64(*wind_speed_m_s)
            }),
        ),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Datapoints'"
            )
        }
    }
}
