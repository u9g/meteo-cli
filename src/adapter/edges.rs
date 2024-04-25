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
        "datapoint" => tower::datapoint(contexts, resolve_info),
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

    pub(super) fn datapoint<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with::<Vertex, _>(contexts, move |vertex| {
            let vertex = vertex
                .as_tower()
                .expect("conversion failed, vertex was not a Tower");
            Box::new(
                vertex
                    .datapoint
                    .borrow_mut()
                    .take()
                    .unwrap()
                    .into_iter()
                    .map(|x| Vertex::Datapoint(x)),
            )
        })
    }
}
