use packybara::types::IdType;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// Caches versionpin changes that the user has selected
/// in the versionpin popup menu, so that the Pin Changes
/// table can stay in sync before the user  hits `save`
#[derive(Debug)]
pub struct PinChangesCache {
    /// The number of rows in the changes ui widget
    row_count: Cell<i32>,
    /// a cache of pkgcoord id => changes ui cache
    // TODO: pkgcoord_index needs to get more sophisticated once I
    // start storing different changes. Specifically, any addition
    // of new distributions.. perhaps i need to change this out for
    // an enum Update{ Change{pkgcoord_id}, NewDistribution{dist_id,pkgcoords}}
    pkgcoord_index: RefCell<HashMap<IdType, i32>>,
    with_updates: RefCell<HashMap<IdType, Vec<String>>>,
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
            pkgcoord_index: RefCell::new(HashMap::new()),
            with_updates: RefCell::new(HashMap::new()),
        }
    }
    /// Reset the instance to its initial value, with the row_count at `0`
    /// and the pkgcoord_index empty
    pub fn reset(&self) {
        self.row_count.set(0);
        self.pkgcoord_index.borrow_mut().clear();
        self.with_updates.borrow_mut().clear();
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
    /// * `pkgcoord_id` The distribution's id
    ///
    /// # Returns
    /// * A Some(index) if exant
    /// * Otherwise None
    pub fn index(&self, pkgcoord_id: IdType) -> Option<i32> {
        match self.pkgcoord_index.borrow().get(&pkgcoord_id) {
            None => None,
            Some(v) => Some(*v),
        }
    }
    /// Inserts a distribution's id and index into the cache
    ///
    /// # Argument
    /// * `pkgcoord_id` - The distribution's package coordinate id
    /// * `dist_idx - THe distribution's index in the ui element
    pub fn cache_dist(&self, pkgcoord_id: IdType, dist_idx: i32) {
        self.pkgcoord_index
            .borrow_mut()
            .insert(pkgcoord_id, dist_idx);
    }
    /// Test to see if the cache has the distribution id
    ///
    /// # arguments
    /// * `pkgcoord_id` - The distribution's package coordinate id to test
    ///
    /// # Returns
    /// * true if pkgcoord_id in cache.
    /// * false if pkgcoord_id is not in cache
    pub fn has_key(&self, pkgcoord_id: IdType) -> bool {
        self.pkgcoord_index.borrow().contains_key(&pkgcoord_id)
    }

    /// Check to see if the cache has withs for a particular distribution, given its id
    ///
    /// # Arguments
    /// * `pkgcoord_id` The distribution's package coordinate id of interest
    ///
    /// # Returns
    /// * bool - Indicating the presence or absence of withs for a given distribution
    pub fn has_withs(&self, pkgcoord_id: IdType) -> bool {
        self.with_updates.borrow().contains_key(&pkgcoord_id)
    }
    /// Set the withs for
    ///
    /// # Arguments
    /// * `pkgcoord_id` - The pkgcoord id for which we are recording withs
    /// * `withs` - a vector of with name strings
    pub fn cache_withs(&self, pkgcoord_id: IdType, withs: Vec<String>) {
        self.with_updates.borrow_mut().insert(pkgcoord_id, withs);
    }
    /// Return the withs as either a Some wrapped vec of &str or None
    ///
    /// # Arguments
    /// * `pkgcoord_id` - The pkgcoord id we wish to lookup withs for
    ///
    /// # Returns
    /// * If pkgcoord_id is extant, a vector of package names wrapped in a Some
    /// * if non-extant, None
    pub fn withs(&self, pkgcoord_id: IdType) -> Option<Vec<String>> {
        match self.with_updates.borrow().get(&pkgcoord_id) {
            None => None,
            Some(vals) => {
                let vals_ref = vals.iter().map(|x| x.clone()).collect::<Vec<String>>();
                Some(vals_ref)
            }
        }
    }
    /// return a comma separated list of withs converted to a string
    ///
    /// # Arguments
    /// * `pkgcoord_id`: The pkgcoord id whose withs we want
    ///
    /// # Returns
    /// * If pkgcoord_id is extant, a Some wrapped string
    /// * If pkgcoord_id is non-extant, None
    //TODO: figure out if there is a way of returning a non-owned vec of
    // &strs so we dont have to allocate
    pub fn withs_string(&self, pkgcoord_id: IdType) -> Option<String> {
        match self.with_updates.borrow().get(&pkgcoord_id) {
            None => None,
            Some(vals) => Some(vals.join(",")),
        }
    }
}
