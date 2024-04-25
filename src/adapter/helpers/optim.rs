use trustfall::FieldValue;

pub fn convert_field_value_to_string(field_value: &FieldValue) -> String {
    match field_value {
        trustfall::FieldValue::Float64(f) => f.to_string(),
        trustfall::FieldValue::Uint64(uint) => uint.to_string(),
        trustfall::FieldValue::Int64(int) => int.to_string(),
        _ => todo!(),
    }
}
