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
        "time" => resolve_property_with::<Vertex, _>(contexts, field_property!(as_datapoint, time)),
        "wind_speed_meters_per_second" => resolve_property_with::<Vertex, _>(contexts, |v| {
            FieldValue::Float64(
                v.as_datapoint()
                    .unwrap()
                    .wind_speed_meters_per_second
                    .into(),
            )
        }),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Datapoint'"
            )
        }
    }
}

pub(super) fn resolve_temperature_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        // value is retrieved in the type it's wanted as already
        "fahrenheit" => resolve_property_with::<Vertex, _>(contexts, |v| {
            FieldValue::Float64((*v.as_temperature().unwrap()).1 .0.into())
        }),
        "celsius" => resolve_property_with::<Vertex, _>(contexts, |v| {
            FieldValue::Float64((*v.as_temperature().unwrap()).0 .0.into())
        }),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Temperature'"
            )
        }
    }
}
