use super::bottom_context_widget;
use super::revision_changes_table;
use super::revisions_table;
use super::versionpin_changes_table;
use crate::utility::{create_hlayout, create_vlayout, qs};
use pbgui_logger::LogWin;
use pbgui_menubar::menu_bar::InnerMenuBar;
use qt_core::QSize;
use qt_core::{Orientation, QString, ToolButtonStyle};
use qt_gui::{
    q_icon::{Mode, State},
    QIcon,
};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    q_size_policy::Policy,
    QAction, QFrame, QHBoxLayout, QPushButton, QSizePolicy, QSplitter, QStackedWidget,
    QTableWidget, QToolButton, QWidget,
};

use std::rc::Rc;
//
// Create pinchanges widget
//
pub fn create_bottom_stacked_widget<'a>(
    mut splitter: MutPtr<QSplitter>,
    menubar: Rc<InnerMenuBar>,
) -> (
    MutPtr<QTableWidget>,
    MutPtr<QTableWidget>,
    MutPtr<QTableWidget>,
    LogWin<'a>,
    MutPtr<QPushButton>,
    MutPtr<QStackedWidget>,
    MutPtr<QToolButton>,
    MutPtr<QToolButton>,
    MutPtr<QToolButton>,
    MutPtr<QPushButton>,
    MutPtr<QStackedWidget>,
    CppBox<QIcon>,
) {
    unsafe {
        // create widget
        let mut bottom_stacked_widget = QWidget::new_0a();
        // create vertical layout owned by widget
        let mut pc_vlayout = create_vlayout();
        bottom_stacked_widget.set_object_name(&qs("ContainerWidget"));
        let mut pc_vlayout_ptr = pc_vlayout.as_mut_ptr();
        bottom_stacked_widget.set_layout(pc_vlayout.into_ptr());

        // Create top horizontal layout for hosting switches for the stacked
        // layout as well as context controls.
        let mut top_hlayout = QHBoxLayout::new_0a();
        let mut top_hlayout_ptr = top_hlayout.as_mut_ptr();
        top_hlayout.set_spacing(10);
        top_hlayout.set_contents_margins_4a(10, 10, 10, 10);
        // pin changes button
        // let mut pinchanges_button = create_check_button("Pin Changes", true);

        let action = menubar
            .view_action_at_idx(7)
            .expect("unable to get action from menubar");
        let mut pinchanges_button = create_toolbutton(action, true);

        let pinchanges_button_ptr = pinchanges_button.as_mut_ptr();
        top_hlayout.add_widget(pinchanges_button.into_ptr());
        //history button

        let action = menubar
            .view_action_at_idx(8)
            .expect("unable to get action from menubar");
        let mut history_button = create_toolbutton(action, false);

        let history_button_ptr = history_button.as_mut_ptr();
        top_hlayout.add_widget(history_button.into_ptr());
        // logger button
        let action = menubar
            .view_action_at_idx(9) // index of button in toolbar
            .expect("unable to get action from menubar");
        let mut log_button = create_toolbutton(action, false);

        let log_button_ptr = log_button.as_mut_ptr();
        top_hlayout.add_widget(log_button.into_ptr());
        top_hlayout.add_stretch_0a();

        pc_vlayout_ptr.add_layout_1a(top_hlayout.into_ptr());
        //
        //  stacked widget
        //
        let mut stacked = QStackedWidget::new_0a();
        let mut stacked_ptr = stacked.as_mut_ptr();
        pc_vlayout_ptr.add_widget(stacked.into_ptr());
        pc_vlayout_ptr.set_stretch_factor_q_widget_int(stacked_ptr, 1);
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
        let mut controls = Vec::new();
        //
        // set up the pinchanges table
        //
        let mut pinchanges = versionpin_changes_table::create();
        let pinchanges_ptr = pinchanges.as_mut_ptr();
        pg1_layout_ptr.add_widget(pinchanges.into_ptr());
        // save button
        let mut save_widget = QWidget::new_0a();
        let mut save_layout = create_hlayout();
        save_layout.insert_stretch_2a(0, 1);
        let mut save_layout_ptr = save_layout.as_mut_ptr();
        save_widget.set_layout(save_layout.into_ptr());
        let mut save_button = QPushButton::from_q_string(&QString::from_std_str("Save"));
        let save_button_ptr = save_button.as_mut_ptr();
        save_layout_ptr.add_widget(save_button.into_ptr());
        controls.push(save_widget);
        //
        // set up the second page of the stacked widget
        //
        let mut pg2_widget = QWidget::new_0a();
        pg2_widget.set_object_name(&qs("HistoryWidget"));
        let mut pg2_layout = create_vlayout();
        let mut pg2_layout_ptr = pg2_layout.as_mut_ptr();
        pg2_widget.set_layout(pg2_layout.into_ptr());
        stacked_ptr.add_widget(pg2_widget.into_ptr());
        splitter.add_widget(bottom_stacked_widget.into_ptr());
        // page2 context widget
        let pg2_context_widget = QWidget::new_0a();
        //nothing in it
        controls.push(pg2_context_widget);
        //
        // Add revisions table
        //
        let mut revisions_widget = QWidget::new_0a();
        //let mut revisions_widget_ptr = revisions_widget.as_mut_ptr();
        let mut rsplitter = QSplitter::new();
        rsplitter.set_orientation(Orientation::Horizontal);
        let mut rsplitter_ptr = rsplitter.as_mut_ptr();
        let mut rw_layout = create_hlayout();
        rw_layout.add_widget(rsplitter.into_ptr());
        revisions_widget.set_layout(rw_layout.into_ptr());
        pg2_layout_ptr.add_widget(revisions_widget.into_ptr());

        let mut revisions_table = revisions_table::create();
        let revisions_table_ptr = revisions_table.as_mut_ptr();
        let mut revision_changes_table = revision_changes_table::create();
        let changes_table_ptr = revision_changes_table.as_mut_ptr();
        rsplitter_ptr.add_widget(revisions_table.into_ptr());
        rsplitter_ptr.add_widget(revision_changes_table.into_ptr());
        //
        // add logging
        //
        let mut pg1_frame = QFrame::new_0a();
        let pg1_frame_ptr = pg1_frame.as_mut_ptr();
        let pg1_layout = create_vlayout();
        //let pg1_layout_ptr = pg1_layout.as_mut_ptr();
        pg1_frame.set_layout(pg1_layout.into_ptr());
        stacked_ptr.add_widget(pg1_frame.into_ptr());
        // now for the win
        let log_win = LogWin::new(pg1_frame_ptr);

        // log widget
        let mut log_widget = QWidget::new_0a();
        let mut log_layout = create_hlayout();
        log_layout.insert_stretch_2a(0, 1);
        let mut log_layout_ptr = log_layout.as_mut_ptr();
        log_widget.set_layout(log_layout.into_ptr());
        // add controls button
        let mut mode_icon = QIcon::new();
        let size = QSize::new_2a(24, 24);
        mode_icon.add_file_4a(
            &qs(":images/gear_grey.svg"),
            &size,
            Mode::Normal,
            State::Off,
        );
        mode_icon.add_file_4a(
            &qs(":images/gear_white.svg"),
            &size,
            Mode::Normal,
            State::On,
        );
        let mut log_ctrls_button =
            QPushButton::from_q_icon_q_string(&mode_icon, &QString::from_std_str(""));
        log_ctrls_button.set_object_name(&qs("LogCtrlsBtn"));
        let log_ctrls_button_ptr = log_ctrls_button.as_mut_ptr();

        log_ctrls_button.set_checkable(true);
        log_layout_ptr.add_widget(log_ctrls_button.into_ptr());
        controls.push(log_widget);

        // add the bottom_context_widget which gives us the ablitity
        // to add controls per page
        let controls_widget_ptr = bottom_context_widget::create(&mut top_hlayout_ptr, controls);
        (
            pinchanges_ptr,
            revisions_table_ptr,
            changes_table_ptr,
            log_win,
            save_button_ptr,
            stacked_ptr,
            pinchanges_button_ptr,
            history_button_ptr,
            log_button_ptr,
            log_ctrls_button_ptr,
            controls_widget_ptr,
            mode_icon,
        )
    }
}

/*
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
*/
unsafe fn create_toolbutton(action: MutPtr<QAction>, checked: bool) -> CppBox<QToolButton> {
    let mut check_button = QToolButton::new_0a();
    check_button.set_object_name(&qs("StackWidgetToolButton"));
    check_button.set_default_action(action);
    check_button.set_auto_exclusive(true);
    check_button.set_checkable(true);
    check_button.set_checked(checked);

    check_button.set_tool_button_style(ToolButtonStyle::ToolButtonTextOnly);
    check_button
}
