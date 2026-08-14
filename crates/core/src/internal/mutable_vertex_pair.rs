use crate::internal::types::MutableVertexReferences;

pub(crate) struct MutableVertexPair {
    pub left: MutableVertexReferences,
    pub right: MutableVertexReferences,
}