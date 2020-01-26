#![allow(dead_code)]
//! The DistributionDialog allows the user to generate one or more pins for a distribution
use qt_core::{FocusPolicy, QString, Signal, SlotOfInt};
use qt_widgets::{
    cpp_core::{CastInto, CppBox, MutPtr, Ptr},
    q_abstract_item_view::SelectionMode,
    q_dialog_button_box::StandardButton,
    QCheckBox, QComboBox, QDialog, QDialogButtonBox, QFrame, QGroupBox, QHBoxLayout, QLabel,
    QLayout, QLineEdit, QListWidget, QVBoxLayout, QWidget,
};
use std::cell::RefCell;
use std::collections::HashMap;

/// LevelMap maps a sequence to a list of shots
pub type LevelMap = HashMap<String, Vec<String>>;

pub use rustqt_utils::{create_hlayout, create_vlayout, qs, set_stylesheet_from_str};

const STYLE_STR: &'static str = include_str!("../resources/pbgui_vpin.qss");
const DEFAULT_SEQ: &'static str = "All Sequences";
const DEFAULT_SHOT: &'static str = "All Shots";
pub const DEFAULT_SITE: &'static str = "any";
pub struct InnerVpinDialog<'a> {
    dialog: CppBox<QDialog>,
    show: RefCell<String>,
    distribution: MutPtr<QLabel>,
    roles_checkbox: MutPtr<QCheckBox>,
    roles_filter: MutPtr<QLineEdit>,
    roles_list: MutPtr<QListWidget>,
    seqs_cbox: MutPtr<QComboBox>,
    seq_shot_checkbox: MutPtr<QCheckBox>,
    shots_cbox: MutPtr<QComboBox>,
    sites_cbox: MutPtr<QComboBox>,
    buttons: MutPtr<QDialogButtonBox>,
    levels: LevelMap,
    seq_shot_cb_slot: SlotOfInt<'a>,
    roles_cb_slot: SlotOfInt<'a>,
}

impl<'a> InnerVpinDialog<'a> {
    /// Create a new InnerVpinDialog
    pub fn create<I: Into<String>>(
        show: I,
        distribution: &str,
        parent: impl CastInto<MutPtr<QWidget>>,
    ) -> InnerVpinDialog {
        unsafe {
            let mut dialog = QDialog::new_1a(parent);
            dialog.set_object_name(&qs("AddVersionPinDialog"));
            dialog.set_window_title(&qs("Add Version-Pin"));
            let mut layout = create_vlayout();
            let mut layout_ptr = layout.as_mut_ptr();

            Self::add_entry_label(layout_ptr);

            let distribution_label = Self::add_distribution_label(distribution, layout_ptr);

            // hlayout will contain the two column  vertical layouts (left and right)
            let mut hlayout = create_hlayout();
            let hlayout_ptr = hlayout.as_mut_ptr();
            layout_ptr.add_layout_1a(hlayout.into_ptr());

            // Left side controls
            let left_layout = Self::add_left_layout(hlayout_ptr);
            let roles_checkbox = Self::add_roles_checkbox(left_layout);
            let mut group_box = Self::add_select_roles_groupbox(left_layout);
            let roles_filter = Self::add_roles_filter(group_box.layout());
            let roles_list = Self::add_roles_listwidget(group_box.layout());
            let _roles_list_cpy = roles_list.as_ptr();

            // right side controls
            let mut right_layout = Self::add_right_layout(hlayout_ptr);
            //
            let seq_shot_checkbox = Self::add_seq_shot_checkbox(right_layout);
            let seq_shot_group_box = Self::add_select_level_groupbox(right_layout);
            let seqs_cbox = Self::add_seq_cbox(seq_shot_group_box.layout());
            let mut seqs_cbox_ref = seqs_cbox
                .as_mut_ref()
                .expect("unable to get mut ref to seq combobox from mutptr");
            let shots_cbox = Self::add_shot_cbox(seq_shot_group_box.layout());
            let mut shots_cbox_ref = shots_cbox
                .as_mut_ref()
                .expect("unable to get mut ref to shot combobox from mutptr");

            let sel_sites_group_box = Self::add_select_site_groupbox(right_layout);
            let sites_cbox = Self::add_site_cbox(sel_sites_group_box.layout());
            right_layout.add_stretch_1a(1);
            layout.add_stretch_1a(1);
            let mut strut = QFrame::new_0a();
            strut.set_fixed_size_2a(100, 50);
            strut.set_object_name(&qs("SelectVpinStrut"));
            layout.add_widget(strut.into_ptr());
            // Ok and Cancel buttons via QDialogButtonBox
            let mut button_box = QDialogButtonBox::from_q_flags_standard_button(
                StandardButton::Ok | StandardButton::Cancel,
            );
            let buttons = button_box.as_mut_ptr();
            layout.add_widget(button_box.into_ptr());
            dialog.set_layout(layout.into_ptr());
            dialog.set_modal(true);
            // create some references to components so that we can use them
            // in Slots
            let mut roles_list_ref = roles_list
                .as_mut_ref()
                .expect("unable to get reference to roles list");
            let mut roles_filter_ref = roles_filter
                .as_mut_ref()
                .expect("unable to get ref to roles_filter");
            // default to disabled
            seqs_cbox_ref.set_enabled(false);
            shots_cbox_ref.set_enabled(false);

            roles_list_ref.set_enabled(false);
            roles_filter_ref.set_enabled(false);
            // create the dialog
            let mut dialog = InnerVpinDialog {
                dialog,
                show: RefCell::new(show.into()),
                distribution: distribution_label,
                roles_checkbox,
                roles_filter: roles_filter,
                roles_list,
                seq_shot_checkbox,
                seqs_cbox,
                shots_cbox,
                sites_cbox,
                buttons,
                levels: LevelMap::new(),
                seq_shot_cb_slot: SlotOfInt::new(move |active: std::os::raw::c_int| {
                    if active > 0 {
                        seqs_cbox_ref.set_enabled(true);
                        shots_cbox_ref.set_enabled(true);
                    } else {
                        seqs_cbox_ref.set_enabled(false);
                        shots_cbox_ref.set_enabled(false);
                    }
                }),
                roles_cb_slot: SlotOfInt::new(move |active: std::os::raw::c_int| {
                    if active > 0 {
                        roles_list_ref.set_enabled(true);
                        roles_filter_ref.set_enabled(true);
                        group_box.set_enabled(true);
                        roles_list_ref.set_focus_0a();
                    } else {
                        roles_list_ref.set_enabled(false);
                        group_box.set_enabled(false);
                        roles_filter_ref.set_enabled(false);
                    }
                }),
            };
            // set up internal signals and slots
            // Enable / Disable roles list and filter
            dialog
                .seq_shot_checkbox
                .state_changed()
                .connect(&dialog.seq_shot_cb_slot);
            dialog
                .roles_checkbox
                .state_changed()
                .connect(&dialog.roles_cb_slot);
            // connect the Cancel button to a slot that dismisses the dialog
            buttons.rejected().connect(dialog.dialog.slot_reject());
            // set teh roles_lsit focus
            dialog.roles_list.set_focus_0a();
            // clear the roles_filter focus
            dialog.roles_filter.clear_focus();

            // return the dialog
            dialog
        }
    }
    /// retrieve the show_name
    pub fn show_name(&self) -> String {
        self.show.borrow().clone()
    }
    /// set the show name
    pub fn set_show_name(&self, new_name: String) {
        self.show.replace(new_name);
    }
    /// retrieve the current distribution as a string
    pub unsafe fn distribution(&self) -> String {
        self.distribution.text().to_std_string()
    }
    /// Retrieve an owned QString of the distribution text
    pub unsafe fn distribution_qs(&self) -> CppBox<QString> {
        self.distribution.text()
    }
    /// Set the distribution given a distribution &str
    pub unsafe fn set_distribution(&self, distribution: &str) {
        let mut distribution_mut = self.distribution;
        distribution_mut.set_text(&qs(distribution));
    }
    /// Return the accepted signal from the button. This is provided as a convenience
    /// for hooking up a slot from this struct.
    pub unsafe fn accepted(&self) -> Signal<()> {
        self.buttons.accepted()
    }

    /// Dismiss the dialog using accept. This is a convenience for consumrs
    /// of this struct, to avoid having to drill down
    pub unsafe fn accept(&mut self) {
        self.dialog.accept()
    }

    /// Get a ponter to the dialog
    pub fn dialog(&self) -> Ptr<QDialog> {
        unsafe { self.dialog.as_ptr() }
    }

    /// Get a mutable pointer to the dialog
    pub fn dialog_mut(&mut self) -> MutPtr<QDialog> {
        unsafe { self.dialog.as_mut_ptr() }
    }

    /// Return the rejected signal
    pub unsafe fn rejected(&self) -> Signal<()> {
        self.buttons.rejected()
    }

    /// Return a lsit of selected item names
    pub unsafe fn selected_roles(&self) -> Option<Vec<String>> {
        if !self.roles_checkbox.is_checked() {
            return None;
        };
        let mut results = Vec::new();

        if self.roles_list.is_null() {
            panic!("roles_list pointer is null")
        };
        let mut items = self.roles_list.selected_items();
        if !items.is_empty() {
            for _c in 0..items.length() {
                let item = items.take_first();
                results.push(item.text().to_std_string());
            }
        }
        Some(results)
    }

    /// Retrieve the current site
    pub unsafe fn selected_site(&self) -> String {
        self.sites_cbox.current_text().to_std_string()
    }

    /// Load the stylesheet
    pub unsafe fn set_default_stylesheet(&mut self) {
        set_stylesheet_from_str(STYLE_STR, self.dialog.as_mut_ptr());
    }

    /// Set the sites
    pub fn set_sites(&self, sites: Vec<&str>) {
        unsafe {
            let mut sites_cbox = self.sites_cbox;
            sites_cbox.clear();
            sites_cbox.add_item_q_string(&qs(DEFAULT_SITE));
            for site in sites {
                sites_cbox.add_item_q_string(&qs(site));
            }
        }
    }

    /// set the list of rols
    pub fn set_roles(&self, roles: Vec<&str>) {
        unsafe {
            let mut roles_list = self.roles_list;
            roles_list.clear();
            for role in roles {
                roles_list.add_item_q_string(&qs(role));
            }
            roles_list.select_all();
            roles_list.set_focus_policy(FocusPolicy::StrongFocus);
        }
    }

    /// provide a vector of strings representing the sequences stored in the map
    pub fn seqs(&self) -> Vec<String> {
        self.levels
            .keys()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    pub unsafe fn selected_seq(&self) -> Option<String> {
        if !self.seqs_cbox.is_enabled() {
            return None;
        }
        let result = self.seqs_cbox.current_text().to_std_string();
        if result == DEFAULT_SEQ {
            None
        } else {
            Some(result)
        }
    }

    pub unsafe fn selected_shot(&self) -> Option<String> {
        if !self.shots_cbox.is_enabled() {
            return None;
        }
        let result = self.shots_cbox.current_text().to_std_string();
        if result == DEFAULT_SHOT {
            None
        } else {
            Some(result)
        }
    }

    /// Return the selected Sequence/shot if applicable
    pub unsafe fn selected_level(&self) -> Option<String> {
        if !self.seq_shot_checkbox.is_checked() {
            None
        } else {
            if let Some(seq) = self.selected_seq() {
                if let Some(shot) = self.selected_shot() {
                    Some(format!("{}.{}.{}", self.show.borrow(), seq, shot))
                } else {
                    Some(format!("{}.{}", self.show.borrow(), seq))
                }
            } else {
                None
            }
        }
    }
    /// Given a new LevelMap, repalace the existing one
    pub fn set_levels_map(&mut self, levels: LevelMap) {
        std::mem::replace(&mut self.levels, levels);
    }

    // pub fn set_levels(&self, levels: Vec<String>) {
    //     unsafe {
    //         let mut seqs_cbox = self.seqs_cbox;
    //         let mut shots_cbox = self.shots_cbox;
    //         seqs_cbox.clear();
    //         seqs_cbox.add_item_q_string(&qs(DEFAULT_SEQ));
    //         for seq in levels {
    //             seqs_cbox.add_item_q_string(&qs(seq));
    //         }
    //         shots_cbox.clear();
    //         shots_cbox.add_item_q_string(&qs(DEFAULT_SHOT));
    //     }
    // }

    pub fn set_levels_from_map(&self) {
        unsafe {
            let mut seqs_cbox = self.seqs_cbox;
            let mut shots_cbox = self.shots_cbox;
            seqs_cbox.clear();
            seqs_cbox.add_item_q_string(&qs(DEFAULT_SEQ));
            for seq in self.levels.keys() {
                seqs_cbox.add_item_q_string(&qs(seq));
            }
            shots_cbox.clear();
            shots_cbox.add_item_q_string(&qs(DEFAULT_SHOT));
        }
    }

    pub unsafe fn clear_shots(&self) {
        let mut shots_cbox = self.shots_cbox;
        shots_cbox.clear();
    }
    /// Given a sequence from a selection, populate the shot combobox
    pub unsafe fn set_shots_for_seq(&self, sequence: &str) {
        let mut shots_cbox = self.shots_cbox;
        shots_cbox.clear();
        shots_cbox.add_item_q_string(&qs(DEFAULT_SHOT));
        if let Some(shots) = self.levels.get(sequence) {
            for shot in shots {
                shots_cbox.add_item_q_string(&qs(shot));
            }
        }
    }

    pub unsafe fn set_roles_focus(&mut self) {
        self.roles_filter.set_focus_0a();
    }

    /// display the dialog
    pub fn show(&mut self) {
        unsafe {
            self.dialog.show();
        }
    }

    pub fn exec(&mut self) -> std::os::raw::c_int {
        unsafe { self.dialog.exec() }
    }

    pub fn open(&mut self) {
        unsafe { self.dialog.open() }
    }

    pub fn result(&self) -> std::os::raw::c_int {
        unsafe { self.dialog.result() }
    }

    pub fn close(&mut self) -> bool {
        unsafe { self.dialog.close() }
    }

    pub fn finished(&self) -> qt_core::Signal<(std::os::raw::c_int,)> {
        self.dialog.finished()
    }
    pub fn seqs_cb(&self) -> MutPtr<QComboBox> {
        self.seqs_cbox
    }

    unsafe fn add_site_cbox(mut parent: MutPtr<QLayout>) -> MutPtr<QComboBox> {
        let mut sites_cbox = QComboBox::new_0a();
        sites_cbox.set_object_name(&qs("SelectLocationComboBox"));
        let sites_cbox_ptr = sites_cbox.as_mut_ptr();
        parent.add_widget(sites_cbox.into_ptr());
        sites_cbox_ptr
    }

    unsafe fn add_select_site_groupbox(mut parent: MutPtr<QVBoxLayout>) -> MutPtr<QGroupBox> {
        let mut label = QLabel::from_q_string(&qs("Select Site"));
        label.set_object_name(&qs("SelectSiteLabel"));
        parent.add_widget(label.into_ptr());
        let mut group_box = QGroupBox::new();
        let group_box_ptr = group_box.as_mut_ptr();
        group_box.set_object_name(&qs("SelectSiteGroupBox"));
        let layout = create_vlayout();
        group_box.set_layout(layout.into_ptr());
        parent.add_widget(group_box.into_ptr());
        group_box_ptr
    }

    unsafe fn add_seq_shot_checkbox(mut parent: MutPtr<QVBoxLayout>) -> MutPtr<QCheckBox> {
        let mut cb = QCheckBox::from_q_string(&qs("Specify Seq(s) / Shot(s)"));
        cb.set_object_name(&qs("SeqShotCheckBox"));
        let cb_ptr = cb.as_mut_ptr();
        parent.add_widget(cb.into_ptr());
        cb_ptr
    }

    unsafe fn add_seq_cbox(mut parent: MutPtr<QLayout>) -> MutPtr<QComboBox> {
        let mut seqs_cbox = QComboBox::new_0a();
        seqs_cbox.set_object_name(&qs("AddSeqsComboBox"));
        seqs_cbox.add_item_q_string(&qs("All Sequences"));
        let seqs_cbox_ptr = seqs_cbox.as_mut_ptr();
        parent.add_widget(seqs_cbox.into_ptr());
        seqs_cbox_ptr
    }

    unsafe fn add_shot_cbox(mut parent: MutPtr<QLayout>) -> MutPtr<QComboBox> {
        let mut shots_cbox = QComboBox::new_0a();
        shots_cbox.set_object_name(&qs("AddShotsComboBox"));
        shots_cbox.add_item_q_string(&qs("All Shots"));
        let shots_cbox_ptr = shots_cbox.as_mut_ptr();
        parent.add_widget(shots_cbox.into_ptr());
        shots_cbox_ptr
    }

    unsafe fn add_select_level_groupbox(mut parent: MutPtr<QVBoxLayout>) -> MutPtr<QGroupBox> {
        let mut group_box = QGroupBox::new();
        let group_box_ptr = group_box.as_mut_ptr();
        group_box.set_object_name(&qs("SelectLevelsGroupBox"));
        let layout = create_vlayout();
        group_box.set_layout(layout.into_ptr());
        parent.add_widget(group_box.into_ptr());
        group_box_ptr
    }

    unsafe fn add_roles_checkbox(mut parent: MutPtr<QVBoxLayout>) -> MutPtr<QCheckBox> {
        let mut cb = QCheckBox::from_q_string(&qs("Specify Roles"));
        let cb_ptr = cb.as_mut_ptr();
        parent.add_widget(cb.into_ptr());
        cb_ptr
    }
    unsafe fn add_roles_listwidget(mut parent: MutPtr<QLayout>) -> MutPtr<QListWidget> {
        let mut list_widget = QListWidget::new_0a();
        list_widget.set_selection_mode(SelectionMode::ExtendedSelection);
        let list_widget_ptr = list_widget.as_mut_ptr();
        parent.add_widget(list_widget.into_ptr());
        list_widget_ptr
    }

    unsafe fn add_roles_filter(parent: MutPtr<QLayout>) -> MutPtr<QLineEdit> {
        let mut hlayout = create_hlayout();
        let mut hlayout_ptr = hlayout.as_mut_ptr();
        let mut parent: MutPtr<QVBoxLayout> = parent.dynamic_cast_mut();
        if parent.is_null() {
            panic!("unable to cast layout");
        }
        parent.add_layout_1a(hlayout.into_ptr());
        hlayout_ptr.add_widget(QLabel::from_q_string(&qs("Filter:")).into_ptr());
        let mut line_edit = QLineEdit::new();
        line_edit.set_object_name(&qs("RolesFilterLineEdit"));
        let line_edit_ptr = line_edit.as_mut_ptr();
        hlayout_ptr.add_widget(line_edit.into_ptr());
        line_edit_ptr
    }

    unsafe fn add_select_roles_groupbox(mut parent: MutPtr<QVBoxLayout>) -> MutPtr<QGroupBox> {
        let mut group_box = QGroupBox::new();
        let mut group_box_ptr = group_box.as_mut_ptr();
        group_box.set_object_name(&qs("SelectRolesGroupBox"));
        parent.add_widget(group_box.into_ptr());
        let layout = create_vlayout();
        group_box_ptr.set_layout(layout.into_ptr());
        // we default to disabled
        group_box_ptr.set_enabled(false);
        group_box_ptr
    }
    unsafe fn add_right_layout(mut parent: MutPtr<QHBoxLayout>) -> MutPtr<QVBoxLayout> {
        let mut layout_right = create_vlayout();
        let layout_right_ptr = layout_right.as_mut_ptr();
        parent.add_layout_1a(layout_right.into_ptr());
        layout_right_ptr
    }
    unsafe fn add_left_layout(mut parent: MutPtr<QHBoxLayout>) -> MutPtr<QVBoxLayout> {
        let mut layout_left = create_vlayout();
        let layout_left_ptr = layout_left.as_mut_ptr();
        parent.add_layout_1a(layout_left.into_ptr());
        layout_left_ptr
    }
    // add the add_entry label to the left hand side
    unsafe fn add_entry_label(mut parent: MutPtr<QVBoxLayout>) {
        // add label
        let mut entry_frame = QFrame::new_0a();
        let mut entry_frame_ptr = entry_frame.as_mut_ptr();
        parent.add_widget(entry_frame.into_ptr());
        let mut add_entries = QLabel::from_q_string(&qs("Add Entry"));
        add_entries.set_object_name(&qs("AddEntriesLabel"));
        let mut add_entry_layout = create_vlayout();
        add_entry_layout.add_widget(add_entries.into_ptr());
        entry_frame_ptr.set_layout(add_entry_layout.into_ptr());
    }

    // add the distribution label in the middle of the dialog
    unsafe fn add_distribution_label(
        distribution: &str,
        mut parent: MutPtr<QVBoxLayout>,
    ) -> MutPtr<QLabel> {
        // layout is the top level layout for the dialog
        let mut dist_frame = QFrame::new_0a();
        let mut distribution = QLabel::from_q_string(&qs(distribution));
        let distribution_ptr = distribution.as_mut_ptr();
        distribution.set_object_name(&qs("DistributionLabel"));
        let mut add_entry_layout = create_hlayout();
        add_entry_layout.add_stretch_1a(1);
        add_entry_layout.add_widget(distribution.into_ptr());
        add_entry_layout.add_stretch_1a(1);
        dist_frame.set_layout(add_entry_layout.into_ptr());
        parent.add_widget(dist_frame.into_ptr());
        distribution_ptr
    }
}
