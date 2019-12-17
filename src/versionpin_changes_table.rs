use crate::constants::*;
use crate::table_headers::setup_table_headers;
use qt_core::{QFlags, QString};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    q_abstract_item_view::{EditTrigger, SelectionBehavior, SelectionMode},
    q_header_view::ResizeMode,
    q_size_policy::Policy,
    QPushButton, QSizePolicy, QSplitter, QTableWidget, QToolBar, QVBoxLayout, QWidget,
};

//---------------------------//
// Create pinchanges widget  //
//---------------------------//
pub fn create_pinchanges_widget(
    splitter: &mut MutPtr<QSplitter>,
) -> (MutPtr<QTableWidget>, MutPtr<QPushButton>) {
    unsafe {
        // create widget
        let mut pinchanges_widget = QWidget::new_0a();
        // create vertical layout owned by widget
        let mut pc_vlayout = QVBoxLayout::new_0a();
        let mut pc_vlayout_ptr = pc_vlayout.as_mut_ptr();
        pinchanges_widget.set_layout(pc_vlayout.into_ptr());
        // create the pinchanges toolbar
        let mut pinchanges_bar = QToolBar::new();
        //pinchanges_bar.set_tool_button_style(ToolButtonStyle::ToolButtonTextBesideIcon);

        let mut pinchanges_bar_ptr = pinchanges_bar.as_mut_ptr();
        // Add the pinchanges toolbar to the vertical layout
        pc_vlayout_ptr.add_widget(pinchanges_bar.into_ptr());
        // create a spacer widget to attempt to push
        // future buttons to right side
        let mut spacer = QWidget::new_0a();
        let sp = QSizePolicy::new_2a(Policy::Expanding, Policy::Fixed);
        spacer.set_size_policy_1a(sp.as_ref());
        // set up the pinchanges table.
        let mut pinchanges = setup_pinchanges_table();
        let pinchanges_ptr = pinchanges.as_mut_ptr();
        //pc_vlayout_ptr.add_widget(spacer.into_ptr());
        pc_vlayout_ptr.add_widget(pinchanges.into_ptr());
        //let save_action = pinchanges_bar_ptr.add_action_1a(&QString::from_std_str("Save"));
        let mut save_button = QPushButton::from_q_string(&QString::from_std_str("Save"));
        let save_button_ptr = save_button.as_mut_ptr();
        pinchanges_bar_ptr.add_widget(spacer.into_ptr());
        pinchanges_bar_ptr.add_widget(save_button.into_ptr());
        splitter.add_widget(pinchanges_widget.into_ptr());

        (pinchanges_ptr, save_button_ptr)
    }
}
//--------------------------
// Setup Pin Changes Table
//--------------------------
fn setup_pinchanges_table() -> CppBox<QTableWidget> {
    unsafe {
        let mut pinchanges = QTableWidget::new_2a(0, PC_HEADERS.len() as i32);
        let mut pinchanges_ptr = pinchanges.as_mut_ptr();
        setup_table_headers(&mut pinchanges_ptr, &PC_HEADERS);
        pinchanges.vertical_header().hide();
        pinchanges.horizontal_header().hide();
        pinchanges.set_selection_behavior(SelectionBehavior::SelectRows);
        pinchanges.set_edit_triggers(QFlags::from(EditTrigger::NoEditTriggers));
        pinchanges.set_selection_mode(SelectionMode::SingleSelection);
        pinchanges
            .horizontal_header()
            .set_stretch_last_section(true);
        pinchanges
            .horizontal_header()
            .set_section_resize_mode_1a(ResizeMode::Stretch);
        pinchanges.set_show_grid(false);
        // The following two statements are responsible for the spacing
        // between entries in the pinchanges table
        pinchanges.vertical_header().set_maximum_section_size(20);
        pinchanges
            .vertical_header()
            .set_section_resize_mode_1a(ResizeMode::ResizeToContents);

        pinchanges
    }
}
