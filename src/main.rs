use std::{collections::HashMap, sync::Arc};

use trustfall::{execute_query, Schema, TransparentValue};

use crate::adapter::Adapter;

mod adapter;

fn main() {
    let schema = Schema::parse(std::include_str!("./adapter/schema.graphql")).unwrap();
    let adapter = Arc::new(Adapter::new());

    let query_results = execute_query(&schema, adapter, std::include_str!("../query.graphql"), {
        let transparent_value: HashMap<Arc<str>, TransparentValue> =
            serde_json::from_str(std::include_str!("../args.json")).unwrap();

        transparent_value
            .into_iter()
            .map(|x| (x.0, x.1).into())
            .collect()
    });

    let query_results_ran = query_results.unwrap().collect::<Vec<_>>();

    println!("{:#?}", query_results_ran);
}
