use std::{collections::BTreeMap, sync::Arc};

use trustfall::{execute_query, FieldValue, Schema};

use crate::adapter::Adapter;

mod adapter;

fn main() {
    let schema = Schema::parse(std::include_str!("./adapter/schema.graphql")).unwrap();
    let adapter = Arc::new(Adapter::new());

    let query_results = execute_query(&schema, adapter, std::include_str!("../query.graphql"), {
        let mut args: BTreeMap<Arc<str>, FieldValue> = BTreeMap::new();
        args.insert("wind_speed_filterer".into(), FieldValue::Float64(18.));
        args
    });

    let query_results_ran = query_results.unwrap().collect::<Vec<_>>();

    println!("{:#?}", query_results_ran);
}
