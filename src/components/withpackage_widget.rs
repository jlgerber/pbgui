use crate::utility::{create_vlayout, qs};
use qt_widgets::q_abstract_item_view::DragDropMode;
use qt_widgets::{
    cpp_core::MutPtr, q_size_policy::Policy, QAction, QFrame, QListWidget, QSizePolicy, QSplitter,
    QToolBar, QToolButton, QWidget,
};
pub struct WithToolbar {
    pub list: MutPtr<QListWidget>,
    pub edit: MutPtr<QAction>,
    pub save: MutPtr<QAction>,
}
impl WithToolbar {
    /// New up a WIthToolbar instance
    pub fn new(list: MutPtr<QListWidget>, edit: MutPtr<QAction>, save: MutPtr<QAction>) -> Self {
        Self { list, edit, save }
    }
}
/// create and return the withpackage list widget, given the parent splitter.
///
/// # Arguments
/// * `splitter` - The  splitter which we will give ownership of the widget to.
///
/// # Returns
/// * A pointer to the Withs List Widget
pub fn create(splitter: &mut MutPtr<QSplitter>) -> WithToolbar {
    unsafe {
        // create the inner withpackage
        let mut withpackage_listwidget = QListWidget::new_0a();
        withpackage_listwidget.set_object_name(&qs("WithsListWidget"));
        withpackage_listwidget.set_drag_enabled(true);
        withpackage_listwidget.set_drag_drop_mode(DragDropMode::InternalMove);
        // create a pointer to it
        let withpackage_listwidget_ptr = withpackage_listwidget.as_mut_ptr();
        // create an outer frame widget
        let mut frame = QFrame::new_0a();
        // hold a pointer to it
        let mut frame_ptr = frame.as_mut_ptr();
        // transfer ownership to the splitter
        splitter.add_widget(frame.into_ptr());
        let mut layout = create_vlayout();
        //
        // toolbar
        //
        let mut toolbar = QToolBar::from_q_string(&qs("WithPackage Toolbar"));
        toolbar.set_floatable(false);
        toolbar.set_movable(false);
        // add spacer widget
        let mut spacer = QWidget::new_0a();
        let sp = QSizePolicy::new_2a(Policy::Expanding, Policy::Fixed);
        spacer.set_size_policy_1a(sp.as_ref());
        toolbar.add_widget(spacer.into_ptr());
        // add actions
        let edit_action = toolbar.add_action_1a(&qs("Edit"));
        let save_action = toolbar.add_action_1a(&qs("Save"));
        // configure buttons
        let mut edit_button: MutPtr<QToolButton> =
            toolbar.widget_for_action(edit_action).dynamic_cast_mut();
        edit_button.set_object_name(&qs("WithpackagesToolbarButton"));
        let mut save_button: MutPtr<QToolButton> =
            toolbar.widget_for_action(save_action).dynamic_cast_mut();
        save_button.set_object_name(&qs("WithpackagesToolbarButton"));
        // update the layout
        layout.add_widget(toolbar.into_ptr());
        layout.add_widget(withpackage_listwidget.into_ptr());
        frame_ptr.set_layout(layout.into_ptr());
        WithToolbar::new(withpackage_listwidget_ptr, edit_action, save_action)
    }
}
