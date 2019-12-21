use crate::utility::qs;
use qt_widgets::{cpp_core::MutPtr, QAction, QMainWindow, QMenuBar};

pub fn setup<'a>(
    main_window: &mut MutPtr<QMainWindow>,
    //withs_dockwidget: &mut MutPtr<QDockWidget>,
) -> MutPtr<QAction> {
    unsafe {
        let mut menubar: MutPtr<QMenuBar> = main_window.menu_bar();
        let mut windows_menu = menubar.add_menu_q_string(&qs("Windows"));
        let mut withs_action = windows_menu.add_action_q_string(&qs("withs"));
        withs_action.set_checkable(true);

        withs_action
    }
}
