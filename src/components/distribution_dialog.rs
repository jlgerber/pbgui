//! The DistributionDialog allows the user to generate one or more pins for a distribution
use crate::utility::{create_hlayout, create_vlayout, qs};
use qt_widgets::{
    cpp_core::{CastInto, CppBox, MutPtr},
    QComboBox, QDialog, QDialogButtonBox, QFrame, QLabel, QTreeView, QWidget,
};
pub fn create(distribution: &str, parent: impl CastInto<MutPtr<QWidget>>) -> CppBox<QDialog> {
    unsafe {
        let mut dialog = QDialog::new_1a(parent);
        dialog.set_window_title(&qs("Add Version-Pin"));

        // layout is the top level layout for the dialog
        let mut layout = create_vlayout();
        let mut layout_ptr = layout.as_mut_ptr();

        // add label
        let mut entry_frame = QFrame::new_0a();
        let mut entry_frame_ptr = entry_frame.as_mut_ptr();
        layout_ptr.add_widget(entry_frame.into_ptr());
        let mut add_entries = QLabel::from_q_string(&qs("Add Entry"));
        add_entries.set_object_name(&qs("AddEntriesLabel"));

        let mut add_entry_layout = create_vlayout();
        add_entry_layout.add_widget(add_entries.into_ptr());
        entry_frame_ptr.set_layout(add_entry_layout.into_ptr());
        // hlayout will contain the two column  vertical layouts (left and right)
        let mut hlayout = create_hlayout();
        let mut hlayout_ptr = hlayout.as_mut_ptr();
        layout_ptr.add_layout_1a(hlayout.into_ptr());

        let mut layout_left = create_vlayout();
        let mut layout_left_ptr = layout_left.as_mut_ptr();
        hlayout_ptr.add_layout_1a(layout_left.into_ptr());

        let mut layout_right = create_vlayout();
        let layout_right_ptr = layout_right.as_mut_ptr();
        hlayout_ptr.add_layout_1a(layout_right.into_ptr());

        dialog.set_layout(layout.into_ptr());

        dialog
    }
}
