use crate::utility::qs;
use qt_core::{QSize, ToolBarArea};
use qt_gui::{
    q_icon::{Mode, State},
    QIcon,
};
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
    pub view_vpin_changes: MutPtr<QAction>,
    _view_vpin_changes_icon: CppBox<QIcon>,
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
        view_vpin_changes: CppBox<QAction>,
        view_vpin_changes_icon: CppBox<QIcon>,
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
                view_vpin_changes: view_vpin_changes.into_ptr(),
                _view_vpin_changes_icon: view_vpin_changes_icon,
            }
        }
    }

    /// Check/uncheck the withs button via its action
    pub fn view_withs(&mut self, checked: bool) {
        unsafe {
            self.view_withs.set_checked(checked);
        }
    }

    /// Check/uncheck the packages button via its action
    pub fn view_packages(&mut self, checked: bool) {
        unsafe {
            self.view_packages.set_checked(checked);
        }
    }
}

/// Create the left toolbar and return the resulting actions
pub fn create(main_window: &mut MutPtr<QMainWindow>) -> LeftToolBarActions {
    unsafe {
        let mut left_toolbar = QToolBar::from_q_string(&qs("Left Toolbar"));
        left_toolbar.set_object_name(&qs("LeftToolBar"));
        let mut mode_action_group = QActionGroup::new(left_toolbar.as_mut_ptr());
        let mode_action_group_ptr = mode_action_group.as_mut_ptr();
        let mut bottom_mode_action_group = QActionGroup::new(left_toolbar.as_mut_ptr());
        let bottom_mode_action_group_ptr = bottom_mode_action_group.as_mut_ptr();

        //shows
        let mut search_shows_icon = QIcon::new(); //from_q_string(&qs(":/images/wheel_us.png"));
        let size = QSize::new_0a();
        search_shows_icon.add_file_4a(&qs(":/images/wheel.png"), &size, Mode::Normal, State::On);
        search_shows_icon.add_file_4a(
            &qs(":/images/wheel_us.png"),
            &size,
            Mode::Normal,
            State::Off,
        );
        let mut search_shows_action = QAction::from_q_icon_q_string_q_object(
            &search_shows_icon,
            &qs("Shows"),
            mode_action_group_ptr,
        );
        search_shows_action.set_tool_tip(&qs("Set the search mode to be show-centric"));
        search_shows_action.set_checkable(true);
        search_shows_action.set_checked(true);
        left_toolbar.add_action(search_shows_action.as_mut_ptr());

        //properties
        let mut search_properties_icon = QIcon::new(); //from_q_string(&qs(":/images/spyglass_us.png"));
        let size = QSize::new_0a();
        search_properties_icon.add_file_4a(
            &qs(":/images/spyglass_us.png"),
            &size,
            Mode::Normal,
            State::Off,
        );
        search_properties_icon.add_file_4a(
            &qs(":/images/spyglass.png"),
            &size,
            Mode::Normal,
            State::On,
        );
        let mut search_properties_action = QAction::from_q_icon_q_string_q_object(
            &search_properties_icon,
            &qs("Props"),
            mode_action_group_ptr,
        );
        search_properties_action.set_tool_tip(&qs("Set the search mode to be property-centric"));
        search_properties_action.set_checkable(true);
        //search_properties_action.set_checked(true);
        left_toolbar.add_action(search_properties_action.as_mut_ptr());

        //packages
        let mut view_packages_icon = QIcon::new(); //from_q_string(&qs(":/images/openbox_us.png"));
        view_packages_icon.add_file_4a(
            &qs(":/images/openbox_us.png"),
            &size,
            Mode::Normal,
            State::Off,
        );
        view_packages_icon.add_file_4a(&qs(":/images/openbox.png"), &size, Mode::Normal, State::On);
        let mut view_packages_action = QAction::from_q_icon_q_string(
            //_q_object(
            &view_packages_icon,
            &qs("Packages"),
            //mode_action_group_ptr,
        );

        view_packages_action.set_tool_tip(&qs("Display / Hide Packages tree"));
        view_packages_action.set_checkable(true);
        view_packages_action.set_checked(true);
        left_toolbar.add_action(view_packages_action.as_mut_ptr());

        //withs
        let mut view_withs_icon = QIcon::new(); //from_q_string(&qs(":/images/box_us.png"));
        view_withs_icon.add_file_4a(&qs(":/images/box_us.png"), &size, Mode::Normal, State::Off);
        view_withs_icon.add_file_4a(&qs(":/images/box.png"), &size, Mode::Normal, State::On);
        let mut view_withs_action = QAction::from_q_icon_q_string(
            //_q_object(
            &view_withs_icon,
            &qs("Withs"),
            //mode_action_group_ptr,
        );
        view_withs_action.set_tool_tip(&qs("Display / Hide Withs List"));
        view_withs_action.set_checkable(true);
        view_withs_action.set_checked(true);
        left_toolbar.add_action(view_withs_action.as_mut_ptr());
        //view_vpin_changes
        let mut view_vpin_changes_icon = QIcon::new();
        view_vpin_changes_icon.add_file_4a(
            &qs(":/images/pin_us.png"),
            &size,
            Mode::Normal,
            State::Off,
        );
        view_vpin_changes_icon.add_file_4a(&qs(":/images/pin.png"), &size, Mode::Normal, State::On);
        let mut view_vpin_changes_action =
            QAction::from_q_icon_q_string(&view_vpin_changes_icon, &qs("VpinChanges"));
        view_vpin_changes_action.set_tool_tip(&qs("Display / Hide Withs List"));
        view_vpin_changes_action.set_checkable(true);
        view_vpin_changes_action.set_checked(true);
        left_toolbar.add_action(view_vpin_changes_action.as_mut_ptr());

        //change pins
        let mut change_pins_icon = QIcon::new();
        change_pins_icon.add_file_4a(
            &qs(":/images/anchor_us.png"),
            &size,
            Mode::Normal,
            State::Off,
        );
        change_pins_icon.add_file_4a(&qs(":/images/anchor.png"), &size, Mode::Normal, State::On);
        let mut change_pins_action = QAction::from_q_icon_q_string_q_object(
            &change_pins_icon,
            &qs("ChangeVpins"),
            bottom_mode_action_group_ptr,
        );
        change_pins_action.set_tool_tip(&qs("Show / Hide Versionpin Change List"));
        change_pins_action.set_checkable(true);
        left_toolbar.add_action(change_pins_action.as_mut_ptr());

        //revisiions pins
        let mut view_revisions_icon = QIcon::new(); //from_q_string(&qs(":/images/tea_madleine_us.png"));
        view_revisions_icon.add_file_4a(
            &qs(":/images/tea_madleine_us.png"),
            &size,
            Mode::Normal,
            State::Off,
        );
        view_revisions_icon.add_file_4a(
            &qs(":/images/tea_madleine.png"),
            &size,
            Mode::Normal,
            State::On,
        );
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
            view_vpin_changes_action,
            view_vpin_changes_icon,
        )
    }
}
