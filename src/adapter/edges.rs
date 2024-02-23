use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

use super::vertex::Vertex;

pub(super) fn resolve_tower_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "Datapoints" => {
            let interval_value: i64 = parameters
                .get("interval_value")
                .expect(
                    "failed to find parameter 'interval_value' for edge 'Datapoints' on type 'Tower'",
                )
                .as_i64()
                .expect(
                    "unexpected null or other incorrect datatype for Trustfall type 'Int!'",
                );
            let interval_unit: &str = parameters
                .get("interval_unit")
                .expect(
                    "failed to find parameter 'interval_unit' for edge 'Datapoints' on type 'Tower'",
                )
                .as_str()
                .expect(
                    "unexpected null or other incorrect datatype for Trustfall type 'String!'",
                );
            tower::datapoints(contexts, interval_value, interval_unit, resolve_info)
        }
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Tower'")
        }
    }
}

mod tower {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn datapoints<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        interval_value: i64,
        interval_unit: &str,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with::<Vertex, _>(contexts, move |vertex| {
            let vertex = vertex
                .as_tower()
                .expect("conversion failed, vertex was not a Tower");
            Box::new(
                vertex
                    .tower_datapoints
                    .clone()
                    .into_iter()
                    .map(Vertex::Datapoint),
            )
            // todo!("get neighbors along edge 'Datapoints' for type 'Tower'")
        })
    }
}
