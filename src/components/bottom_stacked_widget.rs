use super::revisions_table::setup_revisions_table;
use super::versionpin_changes_table::setup_pinchanges_table;
use crate::utility::qs;
use qt_core::QString;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    q_size_policy::Policy,
    QHBoxLayout, QPushButton, QSizePolicy, QSplitter, QStackedWidget, QTableWidget, QVBoxLayout,
    QWidget,
};
//---------------------------//
// Create pinchanges widget  //
//---------------------------//
pub fn create_bottom_stacked_widget(
    splitter: &mut MutPtr<QSplitter>,
) -> (
    MutPtr<QTableWidget>,
    MutPtr<QTableWidget>,
    MutPtr<QPushButton>,
    MutPtr<QStackedWidget>,
    MutPtr<QPushButton>,
    MutPtr<QPushButton>,
) {
    unsafe {
        // create widget
        let mut bottom_stacked_widget = QWidget::new_0a();
        // create vertical layout owned by widget
        let mut pc_vlayout = create_vlayout();
        bottom_stacked_widget.set_object_name(&qs("ContainerWidget"));
        let mut pc_vlayout_ptr = pc_vlayout.as_mut_ptr();
        bottom_stacked_widget.set_layout(pc_vlayout.into_ptr());
        // create another layout
        let mut top_hlayout = QHBoxLayout::new_0a();
        top_hlayout.set_spacing(10);
        top_hlayout.set_contents_margins_4a(10, 10, 10, 10);
        // pin changes button
        let mut pinchanges_button = create_check_button("Pin Changes", true);
        let pinchanges_button_ptr = pinchanges_button.as_mut_ptr();
        top_hlayout.add_widget(pinchanges_button.into_ptr());
        //history button
        let mut history_button = create_check_button("History", false);
        let history_button_ptr = history_button.as_mut_ptr();
        top_hlayout.add_widget(history_button.into_ptr());
        // spacer
        let mut spacer = QWidget::new_0a();
        let sp = QSizePolicy::new_2a(Policy::Expanding, Policy::Fixed);
        spacer.set_size_policy_1a(sp.as_ref());
        top_hlayout.add_widget(spacer.into_ptr());
        // add the top horizontal layout to the vertical layout
        pc_vlayout_ptr.add_layout_1a(top_hlayout.into_ptr());
        let mut stacked = QStackedWidget::new_0a();
        let mut stacked_ptr = stacked.as_mut_ptr();
        pc_vlayout_ptr.add_widget(stacked.into_ptr());
        //
        // set up the first page of the stacked widget
        //
        let mut pg1_widget = QWidget::new_0a();
        let mut pg1_layout = create_vlayout();
        let mut pg1_layout_ptr = pg1_layout.as_mut_ptr();
        pg1_widget.set_layout(pg1_layout.into_ptr());
        stacked_ptr.add_widget(pg1_widget.into_ptr());
        // create a spacer widget to attempt to push
        // future buttons to right side
        let mut spacer = QWidget::new_0a();
        let sp = QSizePolicy::new_2a(Policy::Expanding, Policy::Fixed);
        spacer.set_size_policy_1a(sp.as_ref());
        //
        // set up the pinchanges table
        //
        let mut pinchanges = setup_pinchanges_table();
        let pinchanges_ptr = pinchanges.as_mut_ptr();
        pg1_layout_ptr.add_widget(pinchanges.into_ptr());
        // bottom layout
        let mut bottom_hlayout = create_hlayout();
        let mut bottom_vlayout_ptr = bottom_hlayout.as_mut_ptr();
        pg1_layout_ptr.add_layout_1a(bottom_hlayout.into_ptr());
        // save button
        let mut save_button = QPushButton::from_q_string(&QString::from_std_str("Save"));
        let save_button_ptr = save_button.as_mut_ptr();
        bottom_vlayout_ptr.add_widget(spacer.into_ptr());
        bottom_vlayout_ptr.add_widget(save_button.into_ptr());
        //
        // set up the second page of the stacked widget
        //
        let mut pg2_widget = QWidget::new_0a();
        let mut pg2_layout = create_vlayout();
        let mut pg2_layout_ptr = pg2_layout.as_mut_ptr();
        pg2_widget.set_layout(pg2_layout.into_ptr());
        stacked_ptr.add_widget(pg2_widget.into_ptr());
        splitter.add_widget(bottom_stacked_widget.into_ptr());
        //
        // Add revisions table
        //
        let mut revisions_table = setup_revisions_table();
        let revisions_table_ptr = revisions_table.as_mut_ptr();
        pg2_layout_ptr.add_widget(revisions_table.into_ptr());
        (
            pinchanges_ptr,
            revisions_table_ptr,
            save_button_ptr,
            stacked_ptr,
            pinchanges_button_ptr,
            history_button_ptr,
        )
    }
}
// create a check button which controls the stack widget
fn create_check_button(label: &'static str, checked: bool) -> CppBox<QPushButton> {
    unsafe {
        let mut check_button = QPushButton::from_q_string(&qs(label));
        check_button.set_object_name(&qs("StackWidgetButton"));
        check_button.set_auto_exclusive(true);
        check_button.set_checkable(true);
        check_button.set_checked(checked);
        check_button
    }
}

fn create_vlayout() -> CppBox<QVBoxLayout> {
    unsafe {
        let mut pc_vlayout = QVBoxLayout::new_0a();
        pc_vlayout.set_margin(0);
        pc_vlayout.set_contents_margins_4a(0, 0, 0, 0);
        pc_vlayout.set_spacing(0);
        pc_vlayout
    }
}

fn create_hlayout() -> CppBox<QHBoxLayout> {
    unsafe {
        let mut pc_hlayout = QHBoxLayout::new_0a();
        pc_hlayout.set_margin(0);
        pc_hlayout.set_contents_margins_4a(0, 0, 0, 0);
        pc_hlayout.set_spacing(0);
        pc_hlayout
    }
}
