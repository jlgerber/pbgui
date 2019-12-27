use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// Caches versionpin changes that the user has selected
/// in the versionpin popup menu, so that the Pin Changes
/// table can stay in sync before the user  hits `save`
#[derive(Debug)]
pub struct PinChangesCache {
    /// The number of rows in the changes ui widget
    row_count: Cell<i32>,
    /// a cache of distribution id => changes ui cache
    dist_index: RefCell<HashMap<i32, i32>>,
}

impl PinChangesCache {
    /// Generate a new PinChangesCache instance.
    ///
    /// # Example
    /// ```
    /// use pbgui::cache::PinChangesCache;
    /// use std::rc::Rc;
    /// let pinchanges_cache = Rc::new(PinChangesCache::new());
    /// ```
    pub fn new() -> Self {
        Self {
            row_count: Cell::new(0),
            dist_index: RefCell::new(HashMap::new()),
        }
    }
    /// Reset the instance to its initial value, with the row_count at `0`
    /// and the dist_index empty
    pub fn reset(&self) {
        self.row_count.set(0);
        self.dist_index.borrow_mut().clear();
    }

    /// Return the number of rows in the ui
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * The number of rows in the ui element
    pub fn row_count(&self) -> i32 {
        self.row_count.get()
    }
    /// Increment the rowcount by 1
    pub fn increment_rowcount(&self) {
        let cnt = self.row_count.get();
        self.row_count.set(cnt + 1);
    }
    /// Retreive the index in the cache for the provided distribution id.
    ///
    /// # Arguments
    /// * `dist_id` The distribution's id
    ///
    /// # Returns
    /// * A Some(index) if exant
    /// * Otherwise None
    pub fn index(&self, dist_id: i32) -> Option<i32> {
        match self.dist_index.borrow().get(&dist_id) {
            None => None,
            Some(v) => Some(*v),
        }
    }
    /// Inserts a distribution's id and index into the cache
    ///
    /// # Argument
    /// * `dist_id` - The distribution's id
    /// * `dist_idx - THe distribution's index in the ui element
    pub fn cache_dist(&self, dist_id: i32, dist_idx: i32) {
        self.dist_index.borrow_mut().insert(dist_id, dist_idx);
    }
    /// Test to see if the cache has the distribution id
    ///
    /// # arguments
    /// * `dist_id` - The distribution id to test
    ///
    /// # Returns
    /// * true if dist_id in cache.
    /// * false if dist_id is not in cache
    pub fn has_key(&self, dist_id: i32) -> bool {
        self.dist_index.borrow().contains_key(&dist_id)
    }
}
