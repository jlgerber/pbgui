use super::utility::qs;
use crate::inner_withs_list::InnerWithsList;
pub use crate::traits::*;
use log;
use qt_core::{Key, QModelIndex, QString, Slot};
use qt_gui::{q_key_sequence::StandardKey, QKeySequence, QStandardItem, QStandardItemModel};
use qt_widgets::{
    cpp_core::CastInto, cpp_core::MutPtr, cpp_core::Ref as QRef, QListView, QPushButton, QShortcut,
    QWidget,
};
pub use rustqt_utils::{as_mut_ref, as_ref, enclose, enclose_all};
use std::rc::Rc;

/// Struct which holds configuration for the WithsList
#[derive(Debug)]
pub struct WithsListConfig {
    /// The find shortcut as a string (eg Ctrl+f)
    find_shortcut: String,
    /// The add shortcut as a string
    add_shortcut: String,
}

impl Default for WithsListConfig {
    fn default() -> Self {
        Self {
            find_shortcut: "Ctrl+f".to_string(),
            add_shortcut: "Ctrl+a".to_string(),
        }
    }
}
/// The WithsList provides a listview with a toolbar allowing you
/// to switch between adding and finding members.
/// It stores the main components that are interesting to
/// its clients, including the toolbar, the model, the view,
/// the actual items backing data, and various slots
pub struct WithsList<'l> {
    inner: Rc<InnerWithsList>,
    enter_shortcut: MutPtr<QShortcut>,
    delete_shortcut: MutPtr<QShortcut>,
    cut_shortcut: MutPtr<QShortcut>,
    find_shortcut: MutPtr<QShortcut>,
    add_shortcut: MutPtr<QShortcut>,
    rm: Slot<'l>,
    find_mode: Slot<'l>,
    add_mode: Slot<'l>,
    enter_sc: Slot<'l>,
    find_shortcut_slot: Slot<'l>,
    add_shortcut_slot: Slot<'l>,
}

impl<'l> WithsList<'l> {
    /// New up an WithsList given a parent
    ///
    /// # Arguments
    /// * `parent` - MutPtr to the parent QWidget
    /// * `config` - Instance of WithsListConfig. (which implements default)
    ///
    /// # Returns
    /// * An WithsList instance
    pub fn new(parent: impl CastInto<MutPtr<QWidget>>, config: WithsListConfig) -> WithsList<'l> {
        unsafe {
            let inner = Rc::new(InnerWithsList::new(parent.cast_into()));

            // shortcuts
            let enter_key_seq = QKeySequence::from_int(Key::KeyReturn.to_int());
            let enter_shortcut = QShortcut::new_2a(enter_key_seq.as_ref(), inner.main());

            let key_seq = QKeySequence::from_int(Key::KeyBackspace.to_int());
            let delete_shortcut = QShortcut::new_2a(key_seq.as_ref(), inner.main());

            let cut_key_seq = QKeySequence::from_standard_key(StandardKey::Cut);
            let cut_shortcut = QShortcut::new_2a(cut_key_seq.as_ref(), inner.main());

            let key_seq = QKeySequence::from_q_string(&qs(&config.find_shortcut));
            let find_shortcut = QShortcut::new_2a(key_seq.as_ref(), inner.main());

            let key_seq = QKeySequence::from_q_string(&qs(&config.add_shortcut));
            let add_shortcut = QShortcut::new_2a(key_seq.as_ref(), inner.main());

            // Slots
            let inner_view = inner.view();
            let rm_slot = Slot::new(enclose_all! { () (mut inner_view) move || {
                let selected = inner_view.selection_model().selected_indexes();
                if selected.length() == 0 {
                    return;
                }
                // we need to sort the indexes first. Otherwise, depending upon selection order, we
                // may not
                let mut indexes = (0..selected.size()).into_iter().map(|x| selected.at(x).row()).collect::<Vec<_>>();
                indexes.sort();
                indexes.iter().rev().for_each(|c| {inner_view.model().remove_row_1a(*c); });
            }});
            // store off some references so that we can move them into teh closure
            let mut cbox_ptr = inner.add_combobox();
            let mut listview_ptr = inner.view();
            let model_ptr = inner.model();

            let enter_sc = Slot::new(
                // changed from enclose_mut to enclose since I have to make copies of the
                // variables (as i do above) anyway. No reason to create additional copies of the pointers
                enclose! { (inner) move || {
                    let text = cbox_ptr.current_text();
                    if inner.is_find_active() {
                        if inner.scroll_to_item(text.as_ref(), true) {
                            cbox_ptr.clear_edit_text();
                        }
                        return;
                    }
                    // bail if text is ""
                    if QString::compare_2_q_string(&text, &qs("")) == 0 {return;}
                    // validate that text is in the list
                    let mut found = false;
                    for cnt in 0..cbox_ptr.count() {
                        let item = cbox_ptr.item_text(cnt);
                        if QString::compare_2_q_string(&text,&item) == 0 {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        log::info!("user's entry not valid");
                        return;
                    }
                    if model_ptr.find_items_1a(&text).length() > 0 {
                        if inner.scroll_to_item(text.as_ref(),  true) {

                            cbox_ptr.clear_edit_text();
                        }
                        return;
                    }

                    inner.add_item_to(text.to_std_string().as_str());
                    cbox_ptr.clear_edit_text();
                    listview_ptr.scroll_to_bottom();

                }},
            );
            let find_shortcut_slot = Slot::new(enclose! { (inner) move || {
                inner.set_find_mode();
            }});
            let add_shortcut_slot = Slot::new(enclose! { (inner) move || {
                inner.set_add_mode();
            }});
            let cblabel = inner.add_label();
            let f = Self {
                inner,
                enter_shortcut: enter_shortcut.into_ptr(),
                delete_shortcut: delete_shortcut.into_ptr(),
                cut_shortcut: cut_shortcut.into_ptr(),
                find_shortcut: find_shortcut.into_ptr(),
                add_shortcut: add_shortcut.into_ptr(),
                rm: rm_slot,

                find_mode: Slot::new(as_mut_ref! { (cblabel) enclose! { (cbox_ptr) move || {
                    let mut cbox_ptr = cbox_ptr;
                    cbox_ptr.set_enabled(true);
                    if let Some(mut cblabel) = cblabel {cblabel.set_text(&qs("Find Item"))};
                }}}),

                add_mode: Slot::new(as_mut_ref! {(cblabel) enclose! { ( cbox_ptr) move || {
                    let mut cbox_ptr = cbox_ptr;
                    cbox_ptr.set_enabled(true);
                    if let Some(mut cblabel) = cblabel {cblabel.set_text(&qs("Add Item"))};
                }}}),

                enter_sc,
                find_shortcut_slot,
                add_shortcut_slot,
            };
            // Wire up signals and slots
            f.inner()
                .find_mode_action()
                .triggered()
                .connect(&f.find_mode);

            f.inner().add_mode_action().triggered().connect(&f.add_mode);
            f.enter_shortcut.activated().connect(&f.enter_sc);
            f.delete_shortcut.activated().connect(&f.rm);
            f.cut_shortcut.activated().connect(&f.rm);
            f.find_shortcut.activated().connect(&f.find_shortcut_slot);
            f.add_shortcut.activated().connect(&f.add_shortcut_slot);
            f
        }
    }

    // Retrieve an RC wrapped InnerWithsList instance. This used to be public
    // but it really has no place as part of the public api. All use cases
    // should be covered by explicit methods.
    //
    // # Arguments
    // * None
    //
    // # Returns
    // * Rc of the InnerWithsList instance
    fn inner(&self) -> Rc<InnerWithsList> {
        self.inner.clone()
    }

    /// Retrieve a mutable pointer to the component's top QWidget. That is
    /// the widget contained within that is the parent of the other internal
    /// widgets.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Mutable Pointer to the main QWidget
    pub fn main(&self) -> MutPtr<QWidget> {
        self.inner().main()
    }

    #[allow(dead_code)]
    /// Clear the listview and its backng model
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub fn clear(&self) {
        self.inner().clear();
    }

    #[allow(dead_code)]
    /// Sets the contents to items, removing any pre-existing
    /// items.
    ///
    /// # Arguments
    /// * `items` - a Vector of &str or String
    ///
    /// # Returns
    /// None
    pub fn set_items<I>(&self, items: Vec<I>)
    where
        I: AsRef<str>,
    {
        self.clear();
        let inner = self.inner();
        for item in items {
            inner.add_item(item.as_ref());
        }
    }

    /// Retrieve the model for the component
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr wrapping QStandardItemModel
    pub fn model(&self) -> MutPtr<QStandardItemModel> {
        self.inner().model()
    }

    /// Retrieve the primary list view.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to QListView
    pub fn view(&self) -> MutPtr<QListView> {
        self.inner().view()
    }

    /// Retrieve a MutPtr to the save button
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to QPushButton
    pub fn save_button(&self) -> MutPtr<QPushButton> {
        self.inner().save_button()
    }
    /// add an item to the pulldown
    ///
    /// # Arguments
    /// * The item to be added, as a &str or String
    ///
    /// # Returns
    /// * None
    pub fn add_item<I>(&self, item: I)
    where
        I: AsRef<str>,
    {
        self.inner().add_item_to(item.as_ref()); //, &mut self.model.as_mut_ptr());
    }

    /// add an item to the pulldown
    ///
    /// # Arguments
    /// * The item to be found, as a &MutPtr<QString>
    pub fn find_item<'a>(&self, item: QRef<QString>) -> Option<MutPtr<QStandardItem>> {
        self.inner().find_item(item)
    }

    /// scroll to the provided item in the list
    ///
    /// # Arguments
    /// * `item` - A Ref wrapped QString.
    /// * `select1 - a boolean indicating whether the item should be selected as well as centered
    /// in the view
    ///
    /// # Returns
    /// * None
    pub fn scroll_to_item<'a>(&self, item: QRef<QString>, select_item: bool) {
        self.inner().scroll_to_item(item, select_item);
    }

    /// Select the provided item given a Ref wrapped QModelIndex
    ///
    /// # Arguments
    /// * `item` - QModelIndex of the item we wish to select
    ///
    /// # Returns
    /// * None
    #[allow(dead_code)]
    pub fn select_item(&self, item: QRef<QModelIndex>) {
        self.inner().select_item(item);
    }

    #[allow(dead_code)]
    /// Delete selected items from the list.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub fn delete_sel_items(&self) {
        unsafe {
            let selected = self.view().selection_model().selected_indexes();
            if selected.length() == 0 {
                return;
            }
            let mut view_model = self.view().model();
            for c in 0..selected.length() {
                view_model.remove_row_1a(c);
            }
        }
    }

    /// Get the items as a vector of Strings.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Vector of String
    pub fn items(&self) -> Vec<String> {
        self.inner().items()
    }

    #[allow(dead_code)]
    /// Set comboboc items, replacing any extant items
    ///
    /// # Arguments
    /// * `items` - Vector of items
    ///
    /// # Returns
    /// * None
    pub fn set_cb_items<'c, I>(&self, items: Vec<I>)
    where
        I: AsRef<str>,
    {
        self.inner().set_cb_items(items);
    }

    #[allow(dead_code)]
    /// Remove all items from the combobox
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns None
    pub fn remove_cb_items(&self) {
        self.inner().remove_cb_items();
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
        self.inner().set_cb_max_visible_items(max);
    }

    /// Given a path as a &str to a stylesheet, apply it to the components.
    ///
    /// # Arguments
    /// * `sheet` - Path to the qss stylesheet
    ///
    /// # Returns
    /// * None
    pub fn set_stylesheet(&self, sheet: &str) {
        self.inner().set_stylesheet(sheet);
    }

    /// Set the component to add mode
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * None
    pub fn set_add_mode(&self) {
        self.inner().set_add_mode();
    }

    #[allow(dead_code)]
    /// Set the component to find mode
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * None
    pub fn set_find_mode(&self) {
        self.inner().set_find_mode();
    }
}
