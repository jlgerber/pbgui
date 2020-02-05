use log;
use qt_core::Slot;
use qt_widgets::{cpp_core::MutPtr, QAction, QMainWindow, QMenu, QMenuBar};
use rustqt_utils::qs;
use std::rc::Rc;

pub struct InnerMenuBar {
    menubar: MutPtr<QMenuBar>,
    pub file_menu: MutPtr<QMenu>,
    pub help_menu: MutPtr<QMenu>,
    pub edit_menu: MutPtr<QMenu>,
    pub view_menu: MutPtr<QMenu>,
    pub save_packages_action: MutPtr<QAction>,
    pub clear_edits_action: MutPtr<QAction>,
    // pub toggle_distributions_action: MutPtr<QAction>,
    // pub toggle_withs_action: MutPtr<QAction>,
    // pub toggle_history_action: MutPtr<QAction>,
    pub documentation_action: MutPtr<QAction>,
}

impl InnerMenuBar {
    /// Create a menubar for the provided main window
    pub fn create(main_window: MutPtr<QMainWindow>) -> InnerMenuBar {
        unsafe {
            let mut menubar: MutPtr<QMenuBar> = main_window.menu_bar();
            let mut file_menu = menubar.add_menu_q_string(&qs("File"));
            let save_packages_action = file_menu.add_action_q_string(&qs("save packages.xml"));

            let mut edit_menu = menubar.add_menu_q_string(&qs("Edit"));
            let clear_edits_action = edit_menu.add_action_q_string(&qs("clear edits"));
            let view_menu = menubar.add_menu_q_string(&qs("View"));
            // let mut toggle_distributions_action =
            //     view_menu.add_action_q_string(&qs("Distributions"));
            // toggle_distributions_action.set_checkable(true);

            // let mut toggle_withs_action = view_menu.add_action_q_string(&qs("Withs"));
            // toggle_withs_action.set_checkable(true);

            // let mut toggle_history_action = view_menu.add_action_q_string(&qs("History"));
            // toggle_history_action.set_checkable(true);

            let mut help_menu = menubar.add_menu_q_string(&qs("Help"));
            let documentation_action = help_menu.add_action_q_string(&qs("Documentation"));

            InnerMenuBar {
                menubar,
                file_menu,
                edit_menu,
                view_menu,
                help_menu,
                save_packages_action,
                clear_edits_action,
                // toggle_distributions_action,
                // toggle_withs_action,
                // toggle_history_action,
                documentation_action,
            }
        }
    }

    pub fn menubar(&self) -> MutPtr<QMenuBar> {
        self.menubar
    }

    pub fn file_menu(&self) -> MutPtr<QMenu> {
        self.file_menu
    }

    pub fn save_packages_action(&self) -> MutPtr<QAction> {
        self.save_packages_action
    }

    pub fn view_action_at_idx(&self, idx: i32) -> Option<MutPtr<QAction>> {
        unsafe {
            let mut actions = self.view_menu.actions();
            let lookup = actions.index(idx);
            if lookup.is_null() {
                None
            } else {
                let raw = lookup.as_mut_raw_ptr();
                let mut_ptr = MutPtr::from_raw(*raw);
                if mut_ptr.is_null() {
                    log::error!("Unable to convert MutRef<QAction> to MutPtr<QAction> via *mut T");
                    return None;
                }
                Some(mut_ptr)
            }
        }
    }
}

pub struct MenuBar<'a> {
    inner: Rc<InnerMenuBar>,
    _save_packages: Slot<'a>,
}

impl<'a> MenuBar<'a> {
    pub fn create(main_window: MutPtr<QMainWindow>) -> MenuBar<'a> {
        let inner = Rc::new(InnerMenuBar::create(main_window));
        let menubar = MenuBar {
            inner,
            _save_packages: Slot::new(move || {}),
        };
        menubar
    }

    pub fn inner(&self) -> Rc<InnerMenuBar> {
        self.inner.clone()
    }
}
