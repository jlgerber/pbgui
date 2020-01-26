use super::utility::qs;
use crate::toolbar::ItemListModeToolbar;
use crate::traits::*;
use crate::utility::load_stylesheet;
use log;
use qt_core::{q_item_selection_model::SelectionFlag, MatchFlag, QModelIndex, QString};
use qt_gui::{QStandardItem, QStandardItemModel};
use qt_widgets::{
    cpp_core::Ref as QRef,
    cpp_core::{CppBox, MutPtr},
    q_abstract_item_view::DragDropMode,
    q_abstract_item_view::SelectionMode,
    q_action::ActionEvent,
    QAction, QComboBox, QFrame, QHBoxLayout, QLabel, QLayout, QListView, QPushButton, QToolBar,
    QWidget,
};
//use rustqt_utils::{as_mut_ref, as_ref, enclose, enclose_all};

//
// ITEMLIST
//
/// The WithsList provides a litview with a toolbar allowing you
/// to switch between reordering, deleting, and adding members.
/// It stores the main components that are interesting to
/// its clients, including the toolbar, the model, the view,
/// the actual items backing data, and various slots
pub(crate) struct InnerWithsList {
    main: MutPtr<QWidget>,
    mode_toolbar: ItemListModeToolbar,
    add_label: MutPtr<QLabel>,
    add_combobox: MutPtr<QComboBox>,
    model: CppBox<QStandardItemModel>,
    view: MutPtr<QListView>,
    save_button: MutPtr<QPushButton>,
}

impl InnerWithsList {
    /// New up an WithsList given a parent
    ///
    /// # Arguments
    /// * `parent` - MutPtr to the parent QWidget
    ///
    /// # Returns
    /// * An WithsList instance
    pub(crate) fn new(parent: MutPtr<QWidget>) -> InnerWithsList {
        unsafe {
            let parent = parent;
            let mut main_ptr = Self::setup_main_widget(&parent);

            let mut model = Self::setup_model();

            let mode_toolbar = ItemListModeToolbar::new(&mut main_ptr);

            let (cblabel, cbox) = Self::setup_combobox("ItemCombo", &mut main_ptr);

            let listview_ptr = Self::setup_listview(model.as_mut_ptr(), &mut main_ptr.layout());
            //buttons
            let save_button = Self::setup_button("Save", &mut main_ptr.layout());
            /*
             // shortcuts
            let key_seq = QKeySequence::from_int(Key::KeyReturn.to_int());
            let enter_shortcut = QShortcut::new_2a(key_seq.as_ref(), main_ptr);
            let key_seq = QKeySequence::from_int(Key::KeyBackspace.to_int());
            let delete_shortcut = QShortcut::new_2a(key_seq.as_ref(), main_ptr);

            let cut_key_seq = QKeySequence::from_standard_key(StandardKey::Cut);
            let cut_shortcut = QShortcut::new_2a(cut_key_seq.as_ref(), main_ptr);

            let rm_slot = Slot::new(enclose_all! { () (mut listview_ptr) move || {
                let selected = listview_ptr.selection_model().selected_indexes();
                if selected.length() == 0 {
                    return;
                }
                // we need to sort the indexes first. Otherwise, depending upon selection order, we
                // may not
                let mut indexes = (0..selected.size()).into_iter().map(|x| selected.at(x).row()).collect::<Vec<_>>();
                indexes.sort();
                indexes.iter().rev().for_each(|c| {listview_ptr.model().remove_row_1a(*c); });
            }});
            */
            let f = Self {
                main: main_ptr,
                model,
                mode_toolbar,
                add_label: cblabel,
                add_combobox: cbox,
                view: listview_ptr,
                save_button,
            };

            f
        }
    }

    /// Return a MutPtr to the main qwidget
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QWidget>
    pub(crate) fn main(&self) -> MutPtr<QWidget> {
        self.main
    }

    /// Return a MutPtr to the with packages QListView
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QListView>
    pub(crate) fn view(&self) -> MutPtr<QListView> {
        self.view
    }

    /// Return a MutPtr to the main QToolBar
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QToolBar>
    #[allow(dead_code)]
    pub(crate) fn toolbar(&self) -> MutPtr<QToolBar> {
        self.mode_toolbar.toolbar()
    }

    /// Returns a MutPtr to the combobox used to add and find with packages
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QComboBox>
    pub(crate) fn add_combobox(&self) -> MutPtr<QComboBox> {
        self.add_combobox
    }

    /// Returns a MutPtr to the backging QStandardItemModel
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QStandardItemModel>
    pub(crate) fn model(&self) -> MutPtr<QStandardItemModel> {
        unsafe { self.view.model().dynamic_cast_mut() }
    }

    /// Returns a MutPtr to the add/find combobox's label
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QLabel>
    pub(crate) fn add_label(&self) -> MutPtr<QLabel> {
        self.add_label
    }

    /// Returns a MutPtr to the save button
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QPushButton>
    #[allow(dead_code)]
    pub(crate) fn save_button(&self) -> MutPtr<QPushButton> {
        self.save_button
    }

    #[allow(dead_code)]
    /// Determine if the find mode is active
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * bool indicating whether or not the find mode is active
    pub(crate) fn is_find_active(&self) -> bool {
        self.mode_toolbar.is_find_active()
    }

    /// Determine whether the add mode is active
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * bool indicating whether or not the add mode is active
    #[allow(dead_code)]
    pub(crate) fn is_add_active(&self) -> bool {
        self.mode_toolbar.is_add_active()
    }

    #[allow(dead_code)]
    /// Clear the listview and its backng model
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub(crate) fn clear(&self) {
        unsafe {
            let mut model = self.model();
            model.clear();
        }
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
    pub(crate) fn set_items<I>(&self, items: Vec<I>)
    where
        I: AsRef<str>,
    {
        unsafe {
            self.model().clear();
            for item in items {
                self.add_item(item.as_ref());
            }
        }
    }

    /// Add an item to self.
    ///
    /// # Arguments
    /// * iteem - The name of the item
    ///
    /// # Returns
    /// * Noen
    pub(crate) fn add_item_to(&self, item: &str) {
        unsafe {
            let mut si = QStandardItem::new();
            si.set_text(&qs(item));
            si.set_drop_enabled(false);
            self.model().append_row_q_standard_item(si.into_ptr());
        }
    }

    /// Retrieve a vector of Strings for items
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Vec<String>
    pub(crate) fn items(&self) -> Vec<String> {
        unsafe {
            let sz = self.model.row_count_0a();
            let mut rval = Vec::with_capacity(sz as usize);
            for c in 0..sz {
                let item = self.model.item_1a(c);
                if item.is_null() {
                    log::error!("item ptr is null. skipping");
                    continue;
                }
                rval.push(item.text().to_std_string());
            }
            rval
        }
    }

    /// add an item to the pulldown
    ///
    /// # Arguments
    /// * The item to be added, as a &str or String
    pub(crate) fn add_item<I>(&self, item: I)
    where
        I: AsRef<str>,
    {
        self.add_item_to(item.as_ref());
    }

    /// add an item to the pulldown
    ///
    /// # Arguments
    /// * The item to be found, as a &MutPtr<QString>
    #[allow(dead_code)]
    pub(crate) fn find_item<'a>(&self, item: QRef<QString>) -> Option<MutPtr<QStandardItem>> {
        Self::_find_item(item, &self.model())
    }

    /// scroll to the provided item in the list
    ///
    /// # Arguments
    /// * `item` - A Ref wrapped QString.
    /// * `select1 - a boolean indicating whether the item should be selected as well as centered
    /// in the view
    #[allow(dead_code)]
    pub(crate) fn scroll_to_item<'a>(&self, item: QRef<QString>, select_item: bool) -> bool {
        Self::_scroll_to_item(item, &mut self.view(), &mut self.model(), select_item)
    }

    /// Select the provided item given a Ref wrapped QModelIndex
    ///
    /// # Arguments
    /// * `item` - QModelIndex of the item we wish to select
    ///
    /// # Returns
    /// * None
    #[allow(dead_code)]
    pub(crate) fn select_item(&self, item: QRef<QModelIndex>) {
        unsafe {
            Self::_select_item(item, &self.view);
        }
    }

    #[allow(dead_code)]
    /// Delete selected items from the list.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub(crate) fn delete_sel_items(&mut self) {
        unsafe {
            let selected = self.view.selection_model().selected_indexes();
            if selected.length() == 0 {
                return;
            }
            for c in 0..selected.length() {
                self.view.model().remove_row_1a(c);
            }
        }
    }

    #[allow(dead_code)]
    /// Set comboboc items, replacing any extant items
    ///
    /// # Arguments
    /// * `items` - Vector of items
    ///
    /// # Returns
    /// * None
    pub(crate) fn set_cb_items<'c, I>(&self, items: Vec<I>)
    where
        I: AsRef<str>,
    {
        unsafe {
            self.remove_cb_items();
            self.add_combobox().add_item_q_string(&qs(""));
            for item in items {
                self.add_combobox().add_item_q_string(&qs(item.as_ref()));
            }
        }
    }

    #[allow(dead_code)]
    /// Remove all items from the combobox
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns None
    pub(crate) fn remove_cb_items(&self) {
        unsafe {
            self.add_combobox().clear();
        }
    }

    /// Change the max number of items displayed in the combobox's dropdown
    /// list
    ///
    /// # Arguments
    /// * `max` - Maximum number of visible items in the comobobox's dropdown
    ///
    /// # Returns
    /// * None
    pub(crate) fn set_cb_max_visible_items(&self, max: i32) {
        unsafe {
            self.add_combobox().set_max_visible_items(max);
        }
    }

    /// Given a path as a &str to a stylesheet, apply it to the components.
    ///
    /// # Arguments
    /// * `sheet` - Path to the qss stylesheet
    ///
    /// # Returns
    /// * None
    pub(crate) fn set_stylesheet(&self, sheet: &str) {
        load_stylesheet(sheet, self.main);
    }

    pub(crate) fn find_mode_action(&self) -> MutPtr<QAction> {
        self.mode_toolbar.find_mode_action
    }

    pub(crate) fn add_mode_action(&self) -> MutPtr<QAction> {
        self.mode_toolbar.add_mode_action
    }

    #[allow(dead_code)]
    pub(crate) fn set_add_mode(&self) {
        unsafe {
            self.add_mode_action().activate(ActionEvent::Trigger);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn set_find_mode(&self) {
        unsafe {
            self.find_mode_action().activate(ActionEvent::Trigger);
        }
    }

    fn _find_item<'a>(
        item: QRef<QString>,
        model: &MutPtr<QStandardItemModel>,
    ) -> Option<MutPtr<QStandardItem>> {
        unsafe {
            let mut location = model.find_items_2a(item, MatchFlag::MatchCaseSensitive.into());
            if location.count() == 0 {
                return None;
            }
            let first = location.take_first();
            Some(first)
        }
    }

    fn _scroll_to_item<'a>(
        item: QRef<QString>,
        view: &mut MutPtr<QListView>,
        model: &mut MutPtr<QStandardItemModel>,
        select: bool,
    ) -> bool {
        unsafe {
            if let Some(item) = Self::_find_item(item, model) {
                let idx = item.index();
                view.scroll_to_1a(&idx);
                if select == true {
                    Self::_select_item(idx.as_ref(), &view);
                }
                return true;
            }
            false
        }
    }

    unsafe fn _select_item(item: QRef<QModelIndex>, view: &MutPtr<QListView>) {
        view.selection_model().clear();
        view.selection_model()
            .set_current_index(item, SelectionFlag::SelectCurrent.into());
    }

    // setup the main widget, performing configuration, adding a
    // layout, and registering ti with its parent, inserting it into
    // its parent's layout
    //
    // # Arguments
    // * `parent` - reference to the parent widget
    //
    // # Returns
    // * MutPtr to the main widget
    fn setup_main_widget(parent: &MutPtr<QWidget>) -> MutPtr<QWidget> {
        QWidget::create(&parent).add_layout(LayoutType::VBoxLayout)
    }

    // construct a model, configurng it for the listview
    //
    // # Arguments
    // * None
    //
    // # Returns
    // CppBoxed QStandardItemModel instance
    fn setup_model() -> CppBox<QStandardItemModel> {
        unsafe {
            let mut model = QStandardItemModel::new_0a();
            model.set_column_count(1);
            model
        }
    }

    // Given a name and a parent, construct a QComboBox and return it
    //
    // #Arguments
    // * `name` - Name of the combobox
    // * `parent` - mut reference to the parent widget. Will be used to fetch the layout
    //
    // # Returns
    // * A MutPtr wrapping the QComboBox
    fn setup_combobox(
        name: &str,
        mut parent: &mut MutPtr<QWidget>,
    ) -> (MutPtr<QLabel>, MutPtr<QComboBox>) {
        unsafe {
            let mut cb_widget = QFrame::create(&mut parent);
            cb_widget.add_layout(LayoutType::HBoxLayout);
            cb_widget.set_object_name(&qs(format!("{}Widget", name)));

            let mut cb_label = QLabel::from_q_string(&qs("Add Item"));
            cb_label.set_object_name(&qs("WithsCBLabel"));
            let cb_label_ptr = cb_label.as_mut_ptr();
            cb_widget.layout().add_widget(cb_label.into_ptr());

            let mut cbox = QComboBox::new_0a();
            cbox.set_editable(true);
            cbox.set_object_name(&qs("WithsComboBox"));
            let cbox_ptr = cbox.as_mut_ptr();
            cb_widget.layout().add_widget(cbox.into_ptr());

            let mut layout = cb_widget.layout().dynamic_cast_mut::<QHBoxLayout>();
            if layout.is_null() {
                log::error!("unable to cast layout to QHBoxLayout");
                return (cb_label_ptr, cbox_ptr);
            }
            layout.set_stretch(1, 1);

            (cb_label_ptr, cbox_ptr)
        }
    }

    // set up the ListView, configuring drag and drop, registering
    // the model, and adding it into the supplied layout
    //
    // # Arguments
    // * `model` - the instance of the QStandardItemModel, configured
    // * `layout` - The parent layout
    //
    // # Returns
    // * MutPtr wrapped QListView instance
    fn setup_listview(
        model: MutPtr<QStandardItemModel>,
        layout: &mut MutPtr<QLayout>,
    ) -> MutPtr<QListView> {
        unsafe {
            let mut qlv = QListView::new_0a();
            qlv.set_object_name(&qs("WithsListView"));
            qlv.set_model(model);
            qlv.set_drag_enabled(true);
            qlv.set_selection_mode(SelectionMode::ExtendedSelection);
            qlv.set_drag_drop_overwrite_mode(false);
            qlv.set_drag_drop_mode(DragDropMode::InternalMove);
            let qlv_ptr = qlv.as_mut_ptr();
            layout.add_widget(qlv.into_ptr());

            qlv_ptr
        }
    }

    unsafe fn setup_button(name: &str, layout: &mut MutPtr<QLayout>) -> MutPtr<QPushButton> {
        let mut button = QPushButton::from_q_string(&qs(name));
        let button_ptr = button.as_mut_ptr();
        layout.add_widget(button.into_ptr());
        button_ptr
    }
}
