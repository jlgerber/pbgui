use crate::constants::*;
use crate::table_headers;
use qt_core::QFlags;
use qt_widgets::{
    cpp_core::MutPtr,
    q_abstract_item_view::{EditTrigger, SelectionBehavior, SelectionMode},
    q_header_view::ResizeMode,
    qt_core::ContextMenuPolicy,
    qt_core::QString,
    QSplitter, QTableWidget,
};

macro_rules! dark_grey_stripe {
    () => {
        "rgb(40,40,40)"
    };
}
macro_rules! light_grey_stripe {
    () => {
        "rgb(50,50,50)"
    };
}
macro_rules! table_text_color {
    () => {
        "rgb(200,200,200)"
    };
}
macro_rules! table_header_bg_color {
    () => {
        "rgb(80,80,80)"
    };
}
macro_rules! table_header_text_color {
    () => {
        "white"
    };
}

/// Setup the main VersionPin Table.
///
/// This table presents VersionPins, populated in response to the user's request via
/// the top push button.
///
/// # Arguments
/// * `vsplit_ptr` Pointer to the main vertical splitter (running horizontally)
///
/// # Returns
/// * `MutPtr<QTableWidget>` - a mutable pointer to the main table
pub fn create(vsplit_ptr: &mut MutPtr<QSplitter>) -> MutPtr<QTableWidget> {
    unsafe {
        // create the tablewidget
        let mut vpin_tablewidget = QTableWidget::new_2a(0, HEADERS.len() as i32);
        let mut tablewidget_ptr = vpin_tablewidget.as_mut_ptr();
        vsplit_ptr.add_widget(vpin_tablewidget.into_ptr());
        // configure the tablewidget
        tablewidget_ptr.vertical_header().hide();
        tablewidget_ptr.set_selection_behavior(SelectionBehavior::SelectRows);
        tablewidget_ptr.set_edit_triggers(QFlags::from(EditTrigger::NoEditTriggers));
        tablewidget_ptr.set_selection_mode(SelectionMode::SingleSelection);
        tablewidget_ptr.set_show_grid(false);
        tablewidget_ptr.set_alternating_row_colors(true);
        tablewidget_ptr.set_style_sheet(&QString::from_std_str(concat!(
            "alternate-background-color:",
            light_grey_stripe!(),
            ";color:",
            table_text_color!(),
            ";background-color:",
            dark_grey_stripe!(),
            ";"
        )));
        tablewidget_ptr.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
        tablewidget_ptr
            .horizontal_header()
            .set_style_sheet(&QString::from_std_str(concat!(
                "background-color:",
                table_header_bg_color!(),
                ";color:",
                table_header_text_color!(),
                ";border: none; outline:none; border-left: 0px; border-right: 0px;"
            )));
        table_headers::setup(&mut tablewidget_ptr, &HEADERS);
        tablewidget_ptr
            .horizontal_header()
            .set_section_resize_mode_1a(ResizeMode::Stretch); //Stretch
        tablewidget_ptr
            .horizontal_header()
            .set_section_resize_mode_2a(COL_ID, ResizeMode::ResizeToContents);
        tablewidget_ptr
            .horizontal_header()
            .set_section_resize_mode_2a(COL_WITHS, ResizeMode::ResizeToContents);
        tablewidget_ptr
    }
}
