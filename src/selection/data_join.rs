use crate::selection::selection::Selection;

/// D3-like DataJoin: holds update, enter, and exit selections
pub struct DataJoin {
    pub update: Selection,
    pub enter: Selection,
    pub exit: Selection,
}

impl DataJoin {
    /// Create a new DataJoin from a parent selection and new data
    pub fn new(parent: &Selection, _data: &[impl Clone]) -> Self {
        // Pseudocode: actual logic depends on Selection internals
        // 1. Find existing nodes and match to data
        // 2. update: nodes with data
        // 3. enter: data without nodes
        // 4. exit: nodes without data
        let update = parent.clone(); // placeholder
        let enter = parent.clone(); // placeholder
        let exit = parent.clone(); // placeholder
        // TODO: implement matching logic
        DataJoin {
            update,
            enter,
            exit,
        }
    }

    /// Get the enter selection
    pub fn enter(&self) -> &Selection {
        &self.enter
    }
    /// Get the update selection
    pub fn update(&self) -> &Selection {
        &self.update
    }
    /// Get the exit selection
    pub fn exit(&self) -> &Selection {
        &self.exit
    }
}
