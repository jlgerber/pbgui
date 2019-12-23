use crate::utility::qs;
use qt_core::ToolBarArea;
use qt_gui::QIcon;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QAction, QActionGroup, QMainWindow, QToolBar,
};

pub struct LeftToolBarActions {
    _mode_action_group: MutPtr<QActionGroup>,
    _bottom_mode_action_group: MutPtr<QActionGroup>,
    pub search_shows: MutPtr<QAction>,
    _search_shows_icon: CppBox<QIcon>,
    pub search_properties: MutPtr<QAction>,
    _search_properties_icon: CppBox<QIcon>,
    pub change_pins: MutPtr<QAction>,
    _change_pins_icon: CppBox<QIcon>,
    pub view_revisions: MutPtr<QAction>,
    _view_revisions_icon: CppBox<QIcon>,
    pub view_packages: MutPtr<QAction>,
    _view_packages_icon: CppBox<QIcon>,
    pub view_withs: MutPtr<QAction>,
    _view_withs_icon: CppBox<QIcon>,
}

impl LeftToolBarActions {
    pub fn new(
        mode_action_group: CppBox<QActionGroup>,
        bottom_mode_action_group: CppBox<QActionGroup>,
        search_shows: CppBox<QAction>,
        search_shows_icon: CppBox<QIcon>,
        search_properties: CppBox<QAction>,
        search_properties_icon: CppBox<QIcon>,
        change_pins: CppBox<QAction>,
        change_pins_icon: CppBox<QIcon>,
        view_revisions: CppBox<QAction>,
        view_revisions_icon: CppBox<QIcon>,
        view_packages: CppBox<QAction>,
        view_packages_icon: CppBox<QIcon>,
        view_withs: CppBox<QAction>,
        view_withs_icon: CppBox<QIcon>,
    ) -> Self {
        unsafe {
            Self {
                _mode_action_group: mode_action_group.into_ptr(),
                _bottom_mode_action_group: bottom_mode_action_group.into_ptr(),
                search_shows: search_shows.into_ptr(),
                _search_shows_icon: search_shows_icon,
                search_properties: search_properties.into_ptr(),
                _search_properties_icon: search_properties_icon,
                change_pins: change_pins.into_ptr(),
                _change_pins_icon: change_pins_icon,
                view_revisions: view_revisions.into_ptr(),
                _view_revisions_icon: view_revisions_icon,
                view_packages: view_packages.into_ptr(),
                _view_packages_icon: view_packages_icon,
                view_withs: view_withs.into_ptr(),
                _view_withs_icon: view_withs_icon,
            }
        }
    }
}

/// Create the left toolbar and return the resulting actions
pub fn create(main_window: &mut MutPtr<QMainWindow>) -> LeftToolBarActions {
    unsafe {
        let mut left_toolbar = QToolBar::new();
        let mut mode_action_group = QActionGroup::new(left_toolbar.as_mut_ptr());
        let mode_action_group_ptr = mode_action_group.as_mut_ptr();
        let mut bottom_mode_action_group = QActionGroup::new(left_toolbar.as_mut_ptr());
        let bottom_mode_action_group_ptr = bottom_mode_action_group.as_mut_ptr();

        //shows
        let search_shows_icon = QIcon::from_q_string(&qs(":/images/wheel_us.png"));
        let mut search_shows_action = QAction::from_q_icon_q_string_q_object(
            &search_shows_icon,
            &qs("Shows"),
            mode_action_group_ptr,
        );
        search_shows_action.set_tool_tip(&qs("Set the search mode to be show-centric"));
        search_shows_action.set_checkable(true);
        left_toolbar.add_action(search_shows_action.as_mut_ptr());

        //properties
        let search_properties_icon = QIcon::from_q_string(&qs(":/images/spyglass_us.png"));
        let mut search_properties_action = QAction::from_q_icon_q_string_q_object(
            &search_properties_icon,
            &qs("Props"),
            mode_action_group_ptr,
        );
        search_properties_action.set_tool_tip(&qs("Set the search mode to be property-centric"));
        search_properties_action.set_checkable(true);
        left_toolbar.add_action(search_properties_action.as_mut_ptr());

        //packages
        let view_packages_icon = QIcon::from_q_string(&qs(":/images/openbox_us.png"));
        let mut view_packages_action = QAction::from_q_icon_q_string(
            //_q_object(
            &view_packages_icon,
            &qs("Packages"),
            //mode_action_group_ptr,
        );
        view_packages_action.set_tool_tip(&qs("Display / Hide Withs List"));
        view_packages_action.set_checkable(true);
        left_toolbar.add_action(view_packages_action.as_mut_ptr());
        //withs
        let view_withs_icon = QIcon::from_q_string(&qs(":/images/box_us.png"));
        let mut view_withs_action = QAction::from_q_icon_q_string(
            //_q_object(
            &view_withs_icon,
            &qs("Withs"),
            //mode_action_group_ptr,
        );
        view_withs_action.set_tool_tip(&qs("Display / Hide Withs List"));
        view_withs_action.set_checkable(true);
        left_toolbar.add_action(view_withs_action.as_mut_ptr());

        //change pins
        let change_pins_icon = QIcon::from_q_string(&qs(":/images/anchor_us.png"));
        let mut change_pins_action = QAction::from_q_icon_q_string_q_object(
            &change_pins_icon,
            &qs("ChangeVpins"),
            bottom_mode_action_group_ptr,
        );
        change_pins_action.set_tool_tip(&qs("Show / Hide Versionpin Change List"));
        change_pins_action.set_checkable(true);
        left_toolbar.add_action(change_pins_action.as_mut_ptr());

        //revisiions pins
        let view_revisions_icon = QIcon::from_q_string(&qs(":/images/tea_madleine_us.png"));
        let mut view_revisions_action = QAction::from_q_icon_q_string_q_object(
            &view_revisions_icon,
            &qs("ViewRevision"),
            bottom_mode_action_group_ptr,
        );
        view_revisions_action.set_tool_tip(&qs("Show / Hide Revision History List"));
        view_revisions_action.set_checkable(true);
        left_toolbar.add_action(view_revisions_action.as_mut_ptr());
        main_window.add_tool_bar_tool_bar_area_q_tool_bar(
            ToolBarArea::LeftToolBarArea,
            left_toolbar.into_ptr(),
        );

        LeftToolBarActions::new(
            mode_action_group,
            bottom_mode_action_group,
            search_shows_action,
            search_shows_icon,
            search_properties_action,
            search_properties_icon,
            change_pins_action,
            change_pins_icon,
            view_revisions_action,
            view_revisions_icon,
            view_packages_action,
            view_packages_icon,
            view_withs_action,
            view_withs_icon,
        )
    }
}
