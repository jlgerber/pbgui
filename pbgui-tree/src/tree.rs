use crate::api::{ClientProxy, PackratDb};
use crate::inner_tree::InnerTreeView;
use packybara::traits::*;
use qt_core::{QModelIndex, QString, Signal, SlotOfBool, SlotOfQModelIndex, SlotOfQString};
use qt_gui::{QStandardItem, QStandardItemModel};
use qt_widgets::{
    cpp_core::{MutPtr, Ref, StaticUpcast},
    QPushButton, QWidget,
};

use rustqt_utils::{enclose, ToQStringOwned};
use std::rc::Rc;

/// DistributionTreeView provides a tree of packages -> versions -> platforms
/// per site, along with a set of signals/slots that handle expanding &
/// collapsing components
pub struct DistributionTreeView<'a> {
    view: Rc<InnerTreeView>,
    //clicked: SlotOfQModelIndex<'a>,
    expanded: SlotOfQModelIndex<'a>,
    collapsed: SlotOfQModelIndex<'a>,
    filter_visible: SlotOfBool<'a>,
    filter_slot: SlotOfQString<'a>,
}

// filter using is any
fn is_not_any(item: &str) -> Option<&str> {
    if item == "any" {
        None
    } else {
        Some(item)
    }
}

impl<'a> DistributionTreeView<'a> {
    /// create a treeview given a main window of any type that can be cast to QWidget
    ///
    /// # Arguments
    /// * `parent_widget` - The parent of the tree view
    ///
    /// # Returns
    /// * `DistributionTreeView instance
    pub fn create<T>(parent_widget: MutPtr<T>) -> DistributionTreeView<'a>
    where
        T: StaticUpcast<QWidget>,
    {
        unsafe {
            let treeview = Rc::new(InnerTreeView::create(parent_widget));

            //let tv = treeview.clone();
            let filter_slot =
                SlotOfQString::new(enclose! { (treeview) move |new_str: Ref<QString>| {
                    let model_ptr = treeview.model();
                    if new_str.to_std_string() == "" {
                        for cnt in (0..model_ptr.row_count_0a()).rev() {
                            treeview.set_row_hidden(cnt,  false)
                        }
                    } else {
                        for cnt in (0..model_ptr.row_count_0a()).rev() {
                            let item = model_ptr.item_2a(cnt, 0);
                            let txt = item.text();
                            if txt.contains_q_string(new_str) {
                                treeview.set_row_hidden(cnt, false)
                            } else {
                                treeview.set_row_hidden(cnt, true)
                            }
                        }
                    }
                }});
            let dtv = DistributionTreeView {
                view: treeview.clone(),
                // Slots
                // clicked: SlotOfQModelIndex::new(move |_idx: Ref<QModelIndex>| {
                //     tv.clear_selection();
                // }),
                expanded: SlotOfQModelIndex::new(
                    enclose! { (treeview) move |idx: Ref<QModelIndex>| {
                        let model = treeview.model();
                        let row_cnt = model.row_count_1a(idx);
                        if  row_cnt > 1 { return; }

                        // what if we only have 1 item? Lets make sure that it isnt
                        // an intended child (eg a single version or platform)
                        let child = idx.child(0,0);
                        if !child.is_valid() || model.item_from_index(child.as_ref()).text().to_std_string() != "" {
                            return;
                        }

                        let item = model.item_from_index(idx);
                        let item_str = item.text().to_std_string();

                        let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
                        let mut db = PackratDb::new(client);

                        // we are a child of the root. Our parent is not "valid"
                        if idx.parent().is_valid() == false {
                            let results = db
                                .find_all_distributions()
                                .package(&item_str)
                                .query()
                                .expect("unable to find_all_distributions");
                            let results = results.iter().map(|s| s.version.as_str()).collect::<Vec<_>>();
                            if results.len() > 0 {
                                treeview.model().remove_rows_3a(0,1, idx);
                                treeview.set_children(item, results, true);
                            }
                        } else {
                            // if we are not the child of the root, we must be the version, revealing
                            // the platform
                            let results = db
                                .find_all_platforms()
                                .query()
                                .expect("unable to find_all_platforms");
                            let results = results.iter().filter_map(|s| is_not_any(s.name.as_str())).collect::<Vec<_>>();
                            if results.len() > 0 {
                                treeview.model().remove_rows_3a(0,1, idx);
                                treeview.set_children(item, results, false);
                            }
                        }
                    }},
                ),

                collapsed: SlotOfQModelIndex::new(
                    enclose! { (treeview) move |idx: Ref<QModelIndex>| {
                        if treeview.model().row_count_1a(idx) == 1 {
                            treeview.set_row_hidden(idx.row(), false);
                        }
                    }},
                ),
                filter_visible: SlotOfBool::new(enclose! { (treeview) move |vis: bool| {
                    treeview.set_filter_visibility(vis);
                }}),
                filter_slot,
            };

            // Set up signals & slots
            //treeview.view().clicked().connect(&dtv.clicked);
            treeview.view().expanded().connect(&dtv.expanded);
            treeview.view().collapsed().connect(&dtv.collapsed);
            treeview.filter().text_changed().connect(&dtv.filter_slot);

            dtv.filter_check_box()
                .toggled()
                .connect(&dtv.filter_visible);
            dtv
        }
    }
    /// Retrieve the Filter button (which is acting as a checkbox)
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * mutable pointer to the Filter QPushButton instance
    pub fn filter_check_box(&self) -> MutPtr<QPushButton> {
        self.view.filter_cb()
    }

    /// Set the stylesheet to the internal stylesheet
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * None
    pub fn set_default_stylesheet(&self) {
        self.view.set_default_stylesheet();
    }

    /// Retreive the model from the view
    ///
    /// # Aeguments
    /// * None
    ///
    /// # Returns
    /// * A mutable pointer to the QStandardItemModel
    pub fn model(&self) -> MutPtr<QStandardItemModel> {
        self.view.model()
    }

    /// Retrieve the clicked Signal so that we may connect it to a slot
    pub fn clicked(&self) -> Signal<(*const QModelIndex,)> {
        self.view.view().clicked()
    }
    /// Given a type that implements ToQstringOwned, append a distribution.
    ///
    /// # Arguments
    /// * `input` - Instance of any type that implements the ToQStringOwned trait.
    /// (this includes &str, String and QString)
    ///
    /// # Returns
    /// * None
    pub fn add_package<T: ToQStringOwned>(&self, input: T) {
        self.view.add_package(input);
    }

    /// Clear the list of packages
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * None
    pub fn clear_packages(&self) {
        self.view.clear_packages();
    }

    /// Clear the tree selection, if there is any.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * None
    pub fn clear_selection(&self) {
        self.view.clear_selection()
    }

    /// Given a vector of a type that implements the ToQstringOwned trait, set the packages
    /// to match the list.
    ///
    /// # Arguments
    /// * `inputs` - A vecctor of package names (&str or String or QString or...)
    ///
    /// # Returns
    /// * None
    pub fn set_packages<T: ToQStringOwned>(&self, inputs: Vec<T>) {
        self.view.set_packages(inputs);
    }

    /// Add a child to the provided parent.
    ///
    /// # Arguments
    /// * `parent` - a mutable pointer to a QStandardItem rep of a package
    /// * `child` - a disribution version, represented by any type implementing the ToQStringOwned trait.
    ///
    /// # Returns
    /// * None
    pub fn add_child<I>(&self, parent: MutPtr<QStandardItem>, child: I)
    where
        I: ToQStringOwned,
    {
        self.view.add_child(parent, child);
    }

    #[allow(dead_code)]
    /// Set combobox sites, replacing any extant sites
    ///
    /// # Arguments
    /// * `items` - Vector of items
    ///
    /// # Returns
    /// * None
    pub fn set_sites<'c, I>(&self, items: Vec<I>, current: I)
    where
        I: AsRef<str>,
    {
        self.view.set_sites(items, current);
    }

    #[allow(dead_code)]
    /// Remove all sites from the combobox
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns None
    pub fn remove_sites(&self) {
        self.view.remove_sites();
    }

    /// Change the max number of items displayed in the combobox's dropdown
    /// list
    ///
    /// # Arguments
    /// * `max` - Maximum number of visible items in the comobobox's dropdown
    ///
    /// # Returns
    /// * None
    pub fn set_cb_max_visible_items(&self, max: i32) {
        self.view.set_cb_max_visible_items(max);
    }
}
