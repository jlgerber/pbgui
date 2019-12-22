use crate::utility::qs;
use qt_core::ToolBarArea;
use qt_gui::QIcon;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QAction, QMainWindow, QToolBar,
};

pub struct LeftToolBarActions {
    pub search_shows: CppBox<QAction>,
    _search_shows_icon: CppBox<QIcon>,
    pub search_properties: CppBox<QAction>,
    _search_properties_icon: CppBox<QIcon>, //pub change_pins: MutPtr<QAction>,
                                            //pub view_revisions: MutPtr<QAction>,
                                            //pub view_withs: MutPtr<QAction>
}

impl LeftToolBarActions {
    pub fn new(
        search_shows: CppBox<QAction>,
        search_shows_icon: CppBox<QIcon>,
        search_properties: CppBox<QAction>,
        search_properties_icon: CppBox<QIcon>,
        //change_pins: MutPtr<QAction>,
        //view_revisions: MutPtr<QAction>,
        //view_withs: MutPtr<QAction>,
    ) -> Self {
        Self {
            search_shows,
            _search_shows_icon: search_shows_icon,
            search_properties,
            _search_properties_icon: search_properties_icon,
            //change_pins,
            //view_revisions,
            //view_withs,
        }
    }
}

/// Create the left toolbar and return the resulting actions
pub fn create(main_window: &mut MutPtr<QMainWindow>) -> LeftToolBarActions {
    unsafe {
        let mut left_toolbar = QToolBar::new();
        //shows
        let search_shows_icon = QIcon::from_q_string(&qs(":/images/wheel_us.png"));
        let mut search_shows_action =
            QAction::from_q_icon_q_string(&search_shows_icon, &qs("Shows"));
        search_shows_action.set_checkable(true);
        left_toolbar.add_action(search_shows_action.as_mut_ptr());

        //properties
        let search_properties_icon = QIcon::from_q_string(&qs(":/images/spyglass_us.png"));
        let mut search_properties_action =
            QAction::from_q_icon_q_string(&search_properties_icon, &qs("Props"));
        search_properties_action.set_checkable(true);
        left_toolbar.add_action(search_properties_action.as_mut_ptr());

        main_window.add_tool_bar_tool_bar_area_q_tool_bar(
            ToolBarArea::LeftToolBarArea,
            left_toolbar.into_ptr(),
        );

        LeftToolBarActions::new(
            search_shows_action,
            search_shows_icon,
            search_properties_action,
            search_properties_icon,
        )
    }
}
