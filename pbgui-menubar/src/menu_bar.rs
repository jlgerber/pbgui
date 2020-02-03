use log;
use qt_core::{QObject, Slot};
use qt_widgets::{
    cpp_core::{MutPtr, StaticDowncast},
    QAction, QComboBox, QFileDialog, QMainWindow, QMenu, QMenuBar,
};
use rustqt_utils::qs;
use std::rc::Rc;

pub struct InnerMenuBar {
    menubar: MutPtr<QMenuBar>,
    windows_menu: MutPtr<QMenu>,
    save_packages_action: MutPtr<QAction>,
}

impl InnerMenuBar {
    /// Create a menubar for the provided main window
    pub fn create(main_window: MutPtr<QMainWindow>) -> InnerMenuBar {
        unsafe {
            let mut menubar: MutPtr<QMenuBar> = main_window.menu_bar();
            let mut windows_menu = menubar.add_menu_q_string(&qs("File"));
            let save_packages_action = windows_menu.add_action_q_string(&qs("save packages.xml"));

            InnerMenuBar {
                menubar,
                windows_menu,
                save_packages_action,
            }
        }
    }

    pub fn menubar(&self) -> MutPtr<QMenuBar> {
        self.menubar
    }

    pub fn windows_menu(&self) -> MutPtr<QMenu> {
        self.windows_menu
    }

    pub fn save_packages_action(&self) -> MutPtr<QAction> {
        self.save_packages_action
    }
}

pub struct MenuBar<'a> {
    inner: Rc<InnerMenuBar>,
    save_packages: Slot<'a>,
}

impl<'a> MenuBar<'a> {
    pub fn create(main_window: MutPtr<QMainWindow>) -> MenuBar<'a> {
        let mwp = main_window.clone();
        let inner = Rc::new(InnerMenuBar::create(main_window));
        let menubar = MenuBar {
            inner,
            save_packages: Slot::new(move || {})
            
            // save_packages: Slot::new(move || {
            //     log::debug!("save packages triggered");
            //     unsafe {
            //         let combo = mwp.find_child_q_object_1a(&qs("LevelCB"));
            //         if combo.is_null() {
            //             log::error!("Unable to get combobox pointer");
            //         } else {
            //             let cb: MutPtr<QComboBox> = QObject::static_downcast_mut(combo);
            //             if cb.is_null() {
            //                 log::error!("Unable to cast qobject pointer to qcombobox");
            //                 return;
            //             }
            //             let level = cb.current_text().to_std_string();
            //             log::info!("current show: {}", &level);
            //             // now I need qdialog
            //             let output_path = QFileDialog::get_save_file_name_4a(
            //                 mwp,
            //                 &qs("save packages.xml"),
            //                 &qs(""),
            //                 &qs("*.xml"),
            //             );
            //             if output_path.is_null() {
            //                 log::debug!("packages.xml save cancelled by user");
            //                 return;
            //             }
            //             let output_path = output_path.to_std_string();
            //             log::debug!("saving packages.xml to {}", output_path);
            //         }
            //     }
            // }),
        };
        // unsafe {
        //     menubar
        //         .inner()
        //         .save_packages_action()
        //         .triggered()
        //         .connect(&menubar.save_packages);
        // }
        menubar
    }

    pub fn inner(&self) -> Rc<InnerMenuBar> {
        self.inner.clone()
    }

    // pub fn save_packages_action(&self) -> MutPtr<QAction> {
    //     self.inner.save_packages_action
    // }
}
