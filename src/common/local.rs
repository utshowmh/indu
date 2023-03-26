pub(crate) struct Local {
    pub(crate) identifier: String,
    pub(crate) scope_index: usize,
}

impl Local {
    pub(crate) fn new(identifier: String, scope_index: usize) -> Self {
        Self {
            identifier,
            scope_index,
        }
    }
}
