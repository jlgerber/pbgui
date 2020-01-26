use super::utility::qs;
pub use crate::traits::*;
use qt_core::QSize;
use qt_core::ToolButtonStyle;
use qt_gui::{
    q_icon::{Mode, State},
    QIcon,
};
use qt_widgets::cpp_core::Ref as QRef;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    q_size_policy::Policy,
    QAction, QActionGroup, QSizePolicy, QToolBar, QToolButton, QWidget,
};
//
// ITEMLIST TOOLBAR
//
/// A struct holding pointers to the QToolbar instance,
/// along with the action group, all of the actions for the
/// buttons on the toolbar, as well as any internal slots
pub struct ItemListModeToolbar {
    pub toolbar: MutPtr<QToolBar>,
    pub action_group: MutPtr<QActionGroup>,
    pub add_mode_action: MutPtr<QAction>,
    pub find_mode_action: MutPtr<QAction>,
    _mode_icon: CppBox<QIcon>,
}

impl ItemListModeToolbar {
    /// New up an ItemListModeToolbar, and regiter it with it
    /// parent's layout, given it's parent widget.
    ///
    /// # Argument
    /// * `parent` - MutPtr wrapped QWidget
    ///
    /// # Returns
    /// * Instance of ItemListModelToolbar
    pub fn new(parent: &mut MutPtr<QWidget>) -> Self {
        unsafe {
            let mut toolbar = Self::create_toolbar("WithPackage Toolbar");
            let mut action_group = QActionGroup::new(toolbar.as_mut_ptr());
            let action_group_ptr = action_group.as_mut_ptr();
            // add spacer widget
            let spacer = Self::create_spacer();
            let mut mode_icon = QIcon::new();
            let size = QSize::new_2a(24, 24);
            mode_icon.add_file_4a(
                &qs(":images/radio_btn.svg"),
                &size,
                Mode::Normal,
                State::Off,
            );
            mode_icon.add_file_4a(
                &qs(":images/radio_btn_sel.svg"),
                &size,
                Mode::Normal,
                State::On,
            );

            // ADD
            let (add_mode_action, _add_btn) = Self::create_mode_action(
                "Add",
                action_group_ptr,
                &mut toolbar.as_mut_ptr(),
                false,
                Some(mode_icon.as_ref()),
            );

            // Find
            let (find_mode_action, _find_button_ref) = Self::create_mode_action(
                "Find",
                action_group_ptr,
                &mut toolbar.as_mut_ptr(),
                false,
                Some(mode_icon.as_ref()),
            );

            // add in spacer
            toolbar.add_widget(spacer.into_ptr());

            let toolbar_ptr = toolbar.as_mut_ptr();
            parent.layout().add_widget(toolbar.into_ptr());

            let tb = Self {
                toolbar: toolbar_ptr,
                action_group: action_group.into_ptr(),
                find_mode_action: find_mode_action.into_ptr(),
                add_mode_action: add_mode_action.into_ptr(),
                _mode_icon: mode_icon,
            };

            tb
        }
    }

    pub fn toolbar(&self) -> MutPtr<QToolBar> {
        self.toolbar
    }
    #[allow(dead_code)]
    /// Determine if the find mode is active
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * bool indicating whether or not the find mode is active
    pub fn is_find_active(&self) -> bool {
        unsafe { self.find_mode_action.is_checked() }
    }

    #[allow(dead_code)]
    /// Determine whether the add mode is active
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * bool indicating whether or not the add mode is active
    pub fn is_add_active(&self) -> bool {
        unsafe { self.add_mode_action.is_checked() }
    }

    // Create and configure the QToolBar internal instance, provided a name
    //
    // # Arguments
    // * `name` - The proposed name of the new toolbar
    //
    // # Returns
    // * CppBoxed QToolBar instance
    unsafe fn create_toolbar(name: &str) -> CppBox<QToolBar> {
        let mut toolbar = QToolBar::from_q_string(&qs(name));
        toolbar.set_floatable(false);
        toolbar.set_movable(false);
        toolbar.set_object_name(&qs("WithsToolBar"));
        toolbar
    }

    // Create a widget that serves as a spacer for the toolbar.
    //
    // # Arguments
    // * None
    //
    // # Returns
    // * CppBoxed QWidget
    unsafe fn create_spacer() -> CppBox<QWidget> {
        let mut spacer = QWidget::new_0a();
        let sp = QSizePolicy::new_2a(Policy::Expanding, Policy::Fixed);
        spacer.set_size_policy_1a(sp.as_ref());
        spacer
    }

    #[allow(dead_code)]
    // Given a name, and the host toolbar, create and return an action.
    //
    // # Arguments
    // * `name` - The name of the action
    // * `toolbar` A mutable pointer to the QToolBar instance which will
    // host the action as a QToolButton
    //
    // # Returns tuple of
    // * New action,
    // * toolbutton that hosts the action on the toolbar
    unsafe fn create_action(
        name: &str,
        toolbar: &mut MutPtr<QToolBar>,
    ) -> (MutPtr<QAction>, MutPtr<QToolButton>) {
        let mode_action = toolbar.add_action_1a(&qs(name));
        let mut button: MutPtr<QToolButton> =
            toolbar.widget_for_action(mode_action).dynamic_cast_mut();
        button.set_object_name(&qs("WithsToolbarButton"));

        (mode_action, button)
    }

    #[allow(dead_code)]
    // Create a grouped action given a name, the group, toolbar, and an
    // indication of whether the action starts out checked. There should
    // be only one checked action per group.
    //
    // # Arguments
    // * `name` - The name of the action to be created
    // * `action_grp_ptr` - A pointer to the QActionGroup
    // * `toolbar` - A mutable reference to the MutPtr wrapped QToolbar instance
    // we wish to attach our action to
    // * `checked` - an indication of whether the action should be in the checked state
    //
    // # Returns Tuple of
    // * CppBoxed QAction instance created
    // * MutPtr wrapped QToolButton that hosts the action on the toolbar
    unsafe fn create_mode_action(
        name: &str,
        action_grp_ptr: MutPtr<QActionGroup>,
        toolbar: &mut MutPtr<QToolBar>,
        checked: bool,
        icon: Option<QRef<QIcon>>,
    ) -> (CppBox<QAction>, MutPtr<QToolButton>) {
        let mut mode_action = if let Some(icon) = icon {
            QAction::from_q_icon_q_string_q_object(icon, &qs(name), action_grp_ptr)
        } else {
            QAction::from_q_string_q_object(&qs(name), action_grp_ptr)
        };
        mode_action.set_checkable(true);
        mode_action.set_checked(checked);

        toolbar.add_action(mode_action.as_mut_ptr());

        let mut button: MutPtr<QToolButton> = toolbar
            .widget_for_action(mode_action.as_mut_ptr())
            .dynamic_cast_mut();

        button.set_object_name(&qs("WithsToolbarModeButton"));
        button.set_tool_button_style(ToolButtonStyle::ToolButtonTextBesideIcon);

        (mode_action, button)
    }
}
