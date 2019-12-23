use crate::utility::qs;
use qt_core::ToolBarArea;
use qt_gui::QIcon;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QAction, QActionGroup, QMainWindow, QToolBar,
};

pub struct LeftToolBarActions {
    //_mode_action_group: CppBox<QActionGroup>,
    pub search_shows: CppBox<QAction>,
    _search_shows_icon: CppBox<QIcon>,
    pub search_properties: CppBox<QAction>,
    _search_properties_icon: CppBox<QIcon>,
    pub change_pins: CppBox<QAction>,
    _change_pins_icon: CppBox<QIcon>,
    //pub view_revisions: MutPtr<QAction>,
    //pub view_withs: MutPtr<QAction>
}

impl LeftToolBarActions {
    pub fn new(
        //mode_action_group: CppBox<QActionGroup>,
        search_shows: CppBox<QAction>,
        search_shows_icon: CppBox<QIcon>,
        search_properties: CppBox<QAction>,
        search_properties_icon: CppBox<QIcon>,
        change_pins: CppBox<QAction>,
        change_pins_icon: CppBox<QIcon>, //view_revisions: MutPtr<QAction>,
                                         //view_withs: MutPtr<QAction>
    ) -> Self {
        Self {
            //_mode_action_group: mode_action_group,
            search_shows,
            _search_shows_icon: search_shows_icon,
            search_properties,
            _search_properties_icon: search_properties_icon,
            change_pins,
            _change_pins_icon: change_pins_icon,
            //view_revisions,
            //view_withs,
        }
    }
}

/// Create the left toolbar and return the resulting actions
pub fn create(main_window: &mut MutPtr<QMainWindow>) -> LeftToolBarActions {
    unsafe {
        let mut left_toolbar = QToolBar::new();
        //let mut mode_action_group = QActionGroup::new(left_toolbar.as_mut_ptr());
        //let mode_action_group_ptr = mode_action_group.as_mut_ptr();
        //shows
        let search_shows_icon = QIcon::from_q_string(&qs(":/images/wheel_us.png"));
        let mut search_shows_action = QAction::from_q_icon_q_string(
            //_q_object(
            &search_shows_icon,
            &qs("Shows"),
            // mode_action_group_ptr,
        );
        search_shows_action.set_tool_tip(&qs("Set the search mode to be show-centric"));
        search_shows_action.set_checkable(true);
        left_toolbar.add_action(search_shows_action.as_mut_ptr());

        //properties
        let search_properties_icon = QIcon::from_q_string(&qs(":/images/spyglass_us.png"));
        let mut search_properties_action = QAction::from_q_icon_q_string(
            //_q_object(
            &search_properties_icon,
            &qs("Props"),
            //mode_action_group_ptr,
        );
        search_properties_action.set_tool_tip(&qs("Set the search mode to be property-centric"));
        search_properties_action.set_checkable(true);
        left_toolbar.add_action(search_properties_action.as_mut_ptr());

        //change pins
        let change_pins_icon = QIcon::from_q_string(&qs(":/images/anchor_us.png"));
        let mut change_pins_action = QAction::from_q_icon_q_string(
            //_q_object(
            &change_pins_icon,
            &qs("ChangeVpins"),
            //mode_action_group_ptr,
        );
        change_pins_action.set_tool_tip(&qs("Show / Hide Versionpin Change List"));
        change_pins_action.set_checkable(true);
        left_toolbar.add_action(change_pins_action.as_mut_ptr());

        main_window.add_tool_bar_tool_bar_area_q_tool_bar(
            ToolBarArea::LeftToolBarArea,
            left_toolbar.into_ptr(),
        );

        LeftToolBarActions::new(
            //mode_action_group,
            search_shows_action,
            search_shows_icon,
            search_properties_action,
            search_properties_icon,
            change_pins_action,
            change_pins_icon,
        )
    }
}
