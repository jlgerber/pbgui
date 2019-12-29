use crate::change_type::Change;
use packybara::types::IdType;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
/// Caches versionpin changes that the user has selected
/// in the versionpin popup menu, so that the Pin Changes
/// table can stay in sync before the user  hits `save`
#[derive(Debug)]
pub struct PinChangesCache {
    /// a cache of pkgcoord id => changes ui cache
    // TODO: pkgcoord_index needs to get more sophisticated once I
    // start storing different changes. Specifically, any addition
    // of new distributions.. perhaps i need to change this out for
    // an enum Update{ Change{pkgcoord_id}, NewDistribution{dist_id,pkgcoords}}
    //pkgcoord_id => row #
    pkgcoord_index: RefCell<HashMap<IdType, i32>>,
    // versionpin_id => version (string)
    original_version: RefCell<HashMap<IdType, String>>,
    // row # => Change
    changes: RefCell<HashMap<i32, Change>>,
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
            pkgcoord_index: RefCell::new(HashMap::new()),
            original_version: RefCell::new(HashMap::new()),
            changes: RefCell::new(HashMap::new()),
        }
    }
    /// Reset the instance to its initial value
    pub fn reset(&self) {
        self.pkgcoord_index.borrow_mut().clear();
        self.original_version.borrow_mut().clear();
        self.changes.borrow_mut().clear();
    }
    /// Return the number of rows in the ui
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * The number of rows in the ui element
    pub fn row_count(&self) -> i32 {
        self.changes.borrow().len() as i32
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
    /// Cache the original version of a versionpin
    ///
    /// # Argument
    /// * `vpin_id` - The verionpin id to use as a key
    /// * `version` - The version to cache
    pub fn cache_original_version<S>(&self, vpin_id: IdType, version: S)
    where
        S: Into<String>,
    {
        self.original_version
            .borrow_mut()
            .insert(vpin_id, version.into());
    }
    /// Get the original version for the given versionpin id
    ///
    /// # Arguments
    /// * `vpin_id` - The versionpin id we want to use to look up the original version for.alloc
    ///
    /// # Returns
    /// * Some of version string if vpin_id exists
    /// * None otherwise
    pub fn orig_version_for(&self, vpin_id: IdType) -> Option<String> {
        match self.original_version.borrow().get(&vpin_id) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
    /// Retrieve the change for the index.
    /// Note that this has to clone under the hood.
    ///
    /// # Arguments
    /// * `idx` - the row index to retrieve the change at
    ///
    /// # Returns
    /// * Some of Change if successful
    /// * None otherwise
    pub fn change_at(&self, idx: i32) -> Option<Change> {
        match self.changes.borrow().get(&idx) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }
    /// Retrieve the change for the distribution at a given index,
    /// removing it in the process
    ///
    /// # Arguments
    /// * `idx` - The row index to retrieve the Change at, removing it in the process
    ///
    /// # Returns
    /// * Some wrapped Change if successful (removing it from self in the proxess)
    /// * None otherwise
    pub fn remove_change_at(&self, idx: i32) -> Option<Change> {
        self.changes.borrow_mut().remove(&idx)
    }
    /// Return a vector of change indexes.
    /// Storing the index of the change allows us to delete rows
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Vector of change indexes
    pub fn change_indexes(&self) -> Vec<i32> {
        let mut v: Vec<i32> = self.changes.borrow().keys().map(|x| x.clone()).collect();
        v.sort();
        v
    }
    /// Retrieve the last change index if it exsits
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Some wrapped index, if `self` stores any changes
    /// * None otherwise
    pub fn last_change_idx(&self) -> Option<i32> {
        match self.change_indexes().last() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
    /// Insert a change into the cache, incrementing the
    /// last index in the process.
    ///
    /// # Arguments
    /// * `change` - A Change instance to cache
    pub fn cache_change(&self, change: Change) {
        let idx = self.last_change_idx().map_or(0, |x| x + 1);
        self.cache_change_at(change, idx);
    }
    /// Inserts a change at a specific index. Raises an exception
    ///
    /// # Arguments
    /// * `change` - The Change instance to cache.
    /// * `idx - The index to cache the Change at.
    pub fn cache_change_at(&self, change: Change, idx: i32) {
        self.changes.borrow_mut().insert(idx, change);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cache_change_adds_key() {
        let change = Change::ChangeDistribution {
            vpin_id: 1,
            new_dist_id: 2,
        };
        let cache = PinChangesCache::new();
        cache.cache_change(change);
        let keys = cache
            .changes
            .borrow()
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<i32>>();
        assert_eq!(keys, vec![0]);
        // second change
        let change2 = Change::ChangeDistribution {
            vpin_id: 1,
            new_dist_id: 2,
        };
        cache.cache_change(change2);
        let keys = cache
            .changes
            .borrow()
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<i32>>();
        assert_eq!(keys, vec![0, 1]);
    }
    #[test]
    fn change_indexes() {
        let cache = PinChangesCache::new();
        cache.cache_change(Change::ChangeDistribution {
            vpin_id: 1,
            new_dist_id: 2,
        });
        cache.cache_change(Change::ChangeDistribution {
            vpin_id: 2,
            new_dist_id: 3,
        });
        assert_eq!(cache.change_indexes(), vec![0, 1]);
    }
    #[test]
    fn row_count_works() {
        let cache = PinChangesCache::new();
        cache.cache_change(Change::ChangeDistribution {
            vpin_id: 1,
            new_dist_id: 2,
        });
        cache.cache_change(Change::ChangeDistribution {
            vpin_id: 2,
            new_dist_id: 3,
        });
        assert_eq!(cache.row_count(), 2);
    }
    #[test]
    fn when_adding_duplicate_change_row_count_works() {
        let cache = PinChangesCache::new();
        cache.cache_change(Change::ChangeDistribution {
            vpin_id: 1,
            new_dist_id: 2,
        });
        cache.cache_change_at(
            Change::ChangeDistribution {
                vpin_id: 1,
                new_dist_id: 2,
            },
            0,
        );
        assert_eq!(cache.row_count(), 1);
    }
}
