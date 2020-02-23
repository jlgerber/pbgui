//! The cache is used to store versionpin changes/additions requested
//! by the user, either through the dialog allowing the user to change the distribution
//! for a given row in the versionpin table, or the dialog allowing the user to inject
//! one or more distributions into the current show from the list of packages from
//! the tree to the left of the versionpin table.
//!
//! These change requests are accumulated as the user operates, and are reset once the
//! user applies the stored changes via the save button found between the versionpin
//! table and the versionpin changes table below it (ultimately triggering the
//! save_versionpin_changes slot_function ).
use crate::change_type::{Change, ChangeType};
use packybara::types::IdType;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
type ChangeIdx = usize;
/// Caches versionpin changes that the user has selected
/// in the versionpin popup menu, so that the Pin Changes
/// table can stay in sync before the user  hits `save`
#[derive(Debug)]
pub struct PinChangesCache {
    /// A mapping of pkgcoord_id to row number in the pinchanges table
    pkgcoord_index: RefCell<HashMap<IdType, i32>>,
    /// A mapping of versionpin_id to version (string) which stores the original version of the distribution
    /// that we are tracking in the versionpin_changes table. We use this value to identify, for example,
    /// if a suggested change ultimately matches the original value (we don't have to change the db in this case)
    original_version: RefCell<HashMap<IdType, String>>,
    /// A vector of Change instances - These are the core change requests, with enough information to
    /// construct a change db request.
    change_vec: RefCell<Vec<Change>>,
    /// A mapping of vpinchanges_table row number to index of Change
    /// in the  change_vec field. We can use this to apply changes in order
    /// as well as account for changes removed from the table. ( ie when we remove
    /// a change from the vpin change table, we dont have to perform an O(n) move of
    /// items in the change_vec to account for the removal. We can simply remove the
    /// row in the vpinchange table. When applying the changes, we loop over the rows,
    /// look up the Change at the row, and apply it.)
    changes: RefCell<HashMap<i32, ChangeIdx>>,
    /// A mapping of change index to row number, allowing us to look up the vpinchanges table
    /// row in which the change appears. The ChangeIdx is the index in the change_vec
    /// for the change.
    changes_row: RefCell<HashMap<ChangeIdx, i32>>,
    /// stores fake ids for new rows. We use negative values to indicate that a row does not
    /// have a database analog. We keep a counter so that we hand out a uniqe one
    fake_row_id: Cell<i32>,
}

impl PinChangesCache {
    /// Generate a new PinChangesCache instance.
    ///
    /// # Example
    ///
    /// ```
    /// use pbgui::cache::PinChangesCache;
    /// use std::rc::Rc;
    /// let pinchanges_cache = Rc::new(PinChangesCache::new());
    /// ```
    pub fn new() -> Self {
        Self {
            pkgcoord_index: RefCell::new(HashMap::new()),
            original_version: RefCell::new(HashMap::new()),
            change_vec: RefCell::new(Vec::new()),
            changes: RefCell::new(HashMap::new()),
            changes_row: RefCell::new(HashMap::new()),
            fake_row_id: Cell::new(-1),
        }
    }

    /// Reset the instance to its initial value
    pub fn reset(&self) {
        self.pkgcoord_index.borrow_mut().clear();
        self.original_version.borrow_mut().clear();
        self.change_vec.borrow_mut().clear();
        self.changes.borrow_mut().clear();
        self.changes_row.borrow_mut().clear();
        self.fake_row_id.set(-1);
    }
    /// Retrieve the next fake id. Fake ids are used to store new rows that have not yet been
    /// added to the database. Unlike a real entry in the database, fake rows have negative ids.
    pub fn next_fake_row_id(&self) -> i32 {
        let next_id = self.fake_row_id.get();
        self.fake_row_id.set(next_id - 1);
        next_id
    }

    /// Return the number of rows in the versionpin_changes_table
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * The number of rows in the versionpin_changes table
    pub fn row_count(&self) -> i32 {
        self.changes.borrow().len() as i32
    }

    /// look up the index of the row in the versionpin_changes   table for the
    /// provided source pkgcoord_id
    ///
    /// # Arguments
    ///
    /// * `pkgcoord_id` The distribution's package coordinate id
    ///
    /// # Returns
    ///
    /// * A Some(index) if exant
    /// * Otherwise None
    pub fn index(&self, pkgcoord_id: IdType) -> Option<i32> {
        match self.pkgcoord_index.borrow().get(&pkgcoord_id) {
            None => None,
            Some(v) => Some(*v),
        }
    }

    /// Cache the original version of a versionpin's distribution
    ///
    /// # Argument
    ///
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

    /// Get the original version for the distribution at given versionpin id
    ///
    /// # Arguments
    ///
    /// * `vpin_id` - The versionpin id we want to use to look up the original
    ///               version for
    ///
    /// # Returns
    ///
    /// * Some of version string if vpin_id exists
    /// * None otherwise
    pub fn orig_version_for(&self, vpin_id: IdType) -> Option<String> {
        match self.original_version.borrow().get(&vpin_id) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    /// Retrieve the change instance stored at the vpinchange table row.
    /// Note that this has to clone under the hood.
    ///
    /// # Arguments
    ///
    /// * `changes_row` - the row index to retrieve the change at
    ///
    /// # Returns
    ///
    /// * Some of Change if successful
    /// * None otherwise
    pub fn change_at(&self, changes_row: i32) -> Option<Change> {
        match self.changes.borrow().get(&changes_row) {
            Some(c) => Some(self.change_vec.borrow()[*c].clone()),
            None => None,
        }
    }

    // /// Retrieve the change for the distribution at a given index,
    // /// removing it in the process
    // ///
    // /// # Arguments
    //
    // /// * `idx` - The row index to retrieve the Change at, removing it in the process
    // ///
    // /// # Returns

    // /// * Some wrapped Change if successful (removing it from self in the proxess)
    // /// * None otherwise
    // pub fn remove_change_at(&self, idx: i32) -> Option<Change> {
    //     self.changes.borrow_mut().remove(&idx)
    // }

    /// Return a vector of change indexes stored in the cache. The keys are
    /// returned in numeric sorted order.
    ///
    /// Storing the index of the change allows us to delete rows
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * Vector of change indexes
    pub fn change_indexes(&self) -> Vec<i32> {
        let mut v: Vec<i32> = self.changes.borrow().keys().map(|x| x.clone()).collect();
        v.sort();
        v
    }
    /// Retrieve an Option wrapping the last change index if it exists
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * Some wrapped index, if `self` stores any changes
    /// * None otherwise
    pub fn last_change_idx(&self) -> Option<i32> {
        match self.change_indexes().last() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
    /// Retrieve the row in the vpinchanges table that a change is in,
    /// if it is in fact in cached
    ///
    /// # Arguments
    ///
    /// * `change` - A reference to a Change instance
    ///
    /// # Returns
    ///
    /// * An Option wrapped index of the row that matches the supplied chnage
    pub fn change_row(&self, change: &Change) -> Option<i32> {
        for (idx, value) in self.change_vec.borrow().iter().enumerate() {
            if value == change {
                return match self.changes_row.borrow().get(&idx) {
                    Some(v) => Some(*v),
                    _ => None,
                };
            }
        }
        None
    }

    /// Retrieve the row of the change from the vpinchanges table using whose value matches
    /// the supplied ChangeType and whose value.id matches the supplied id.
    ///
    /// # Arguments
    ///
    /// * `id` - The change's id
    /// * `ctype` - The ChangeType instance
    ///
    /// # Returns
    ///
    /// * Some row number if the id represents a Change that is in the table
    /// * None otherwise
    pub fn change_row_from_id(&self, id: u64, ctype: &ChangeType) -> Option<i32> {
        for (idx, value) in self.change_vec.borrow().iter().enumerate() {
            if value.is_a(&ctype) {
                if value.id() == id {
                    return match self.changes_row.borrow().get(&idx) {
                        Some(v) => Some(*v),
                        _ => None,
                    };
                }
            }
        }
        None
    }

    /// Insert a change into the cache, incrementing the
    /// last index in the process.
    ///
    /// # Arguments
    ///
    /// * `change` - A Change instance to cache
    pub fn cache_change(&self, change: Change) {
        let idx = self.last_change_idx().map_or(0, |x| x + 1);
        self.cache_change_at(change, idx);
    }

    /// Inserts a change at a specific index. Raises an exception
    ///
    /// # Arguments
    ///
    /// * `change` - The Change instance to cache.
    /// * `idx - The index to cache the Change at.
    pub fn cache_change_at(&self, change: Change, idx: i32) {
        self.change_vec.borrow_mut().push(change);
        let change_idx = self.change_vec.borrow().len() - 1;
        self.changes.borrow_mut().insert(idx, change_idx);
        self.changes_row.borrow_mut().insert(change_idx, idx);
    }

    /// Inserts a distribution's id and index into the cache
    ///
    /// # Argument
    ///
    /// * `pkgcoord_id` - The distribution's package coordinate id
    /// * `dist_idx - THe distribution's index in the pinchanges table
    pub fn cache_dist(&self, pkgcoord_id: IdType, dist_idx: i32) {
        self.pkgcoord_index
            .borrow_mut()
            .insert(pkgcoord_id, dist_idx);
    }

    /// Test to see if the cache has the pkgcoord_id
    ///
    /// # arguments
    ///
    /// * `pkgcoord_id` - The distribution's package coordinate id to test
    ///
    /// # Returns
    ///
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
        let mut keys = cache
            .changes
            .borrow()
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<i32>>();
        keys.sort();
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
    #[test]
    fn fake_row_works() {
        let cache = PinChangesCache::new();
        let row = cache.next_fake_row_id();
        assert_eq!(row, -1);
        let row = cache.next_fake_row_id();
        assert_eq!(row, -2);
        let row = cache.next_fake_row_id();
        assert_eq!(row, -3);
        cache.reset();
        let row = cache.next_fake_row_id();
        assert_eq!(row, -1);
    }
}
