use qt_widgets::{cpp_core::MutPtr, QAction, QMainWindow, QMenuBar};
use rustqt_utils::qs;

#[allow(dead_code)]
/// Create a menubar for the provided main window
pub fn create<'a>(main_window: &mut MutPtr<QMainWindow>) -> MutPtr<QAction> {
    unsafe {
        let mut menubar: MutPtr<QMenuBar> = main_window.menu_bar();
        let mut windows_menu = menubar.add_menu_q_string(&qs("Windows"));
        let mut withs_action = windows_menu.add_action_q_string(&qs("withs"));
        withs_action.set_checkable(true);

        withs_action
    }
}
