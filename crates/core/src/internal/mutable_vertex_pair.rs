use crate::internal::types::WeakVertexReferences;

pub(crate) struct MutableVertexPair {
    pub left: WeakVertexReferences,
    pub right: WeakVertexReferences,
}
