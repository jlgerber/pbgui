use qt_core::QString;
use qt_core::{QSize, ToolButtonStyle};
use qt_gui::{
    q_icon::{Mode, State},
    QIcon,
};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QComboBox, QFrame, QToolBar, QToolButton,
};
use rustqt_utils::{create_hlayout, qs};

//
// Setup Levels Combobox
//
pub(crate) fn setup_levels_cb<'b>(
    toolbar: &mut MutPtr<QToolBar>,
) -> (MutPtr<QComboBox>, CppBox<QIcon>) {
    //results
    unsafe {
        let mut level_combobox = QComboBox::new_0a();
        level_combobox.set_object_name(&qs("LevelCB"));
        let level_cb_ptr = level_combobox.as_mut_ptr();
        level_combobox.add_item_q_string(&QString::from_std_str("facility"));
        let mut grpbox = QFrame::new_0a();
        grpbox.set_object_name(&qs("FirstComboFrame"));

        let (level_button, level_icon) = create_label("Show", ":images/volume.png");
        let mut hlayout = create_hlayout();
        hlayout.add_widget(level_button.into_ptr());
        // assign owner of level
        hlayout.add_widget(level_combobox.into_ptr());
        grpbox.set_layout(hlayout.into_ptr());
        toolbar.add_widget(grpbox.into_ptr());
        (level_cb_ptr, level_icon)
    }
}

//
// set up the roles combobox
//
pub(crate) fn setup_roles_cb<'b>(
    toolbar: &mut MutPtr<QToolBar>,
) -> (MutPtr<QComboBox>, CppBox<QIcon>) {
    unsafe {
        let mut role_combobox = QComboBox::new_0a();
        role_combobox.set_object_name(&qs("RoleCB"));
        let role_cb_ptr = role_combobox.as_mut_ptr();
        role_combobox.add_item_q_string(&QString::from_std_str("any"));
        let mut grpbox = QFrame::new_0a();
        grpbox.set_object_name(&qs("ComboFrame"));

        let (role_button, role_icon) = create_label("Role", ":images/facehappy.png");
        let mut hlayout = create_hlayout();
        hlayout.add_widget(role_button.into_ptr());

        hlayout.add_widget(role_combobox.into_ptr());
        grpbox.set_layout(hlayout.into_ptr());
        toolbar.add_widget(grpbox.into_ptr());
        (role_cb_ptr, role_icon)
    }
}
//
// setup Platforms
//
pub(crate) fn setup_platforms_cb<'b>(
    toolbar: &mut MutPtr<QToolBar>,
) -> (MutPtr<QComboBox>, CppBox<QIcon>) {
    unsafe {
        let mut platform_combobox = QComboBox::new_0a();
        platform_combobox.set_object_name(&qs("PlatformCB"));
        let platform_cb_ptr = platform_combobox.as_mut_ptr();
        platform_combobox.add_item_q_string(&QString::from_std_str("any"));

        let mut grpbox = QFrame::new_0a();
        grpbox.set_object_name(&qs("ComboFrame"));

        let (platform_button, platform_icon) = create_label("Platform", ":images/computer2.png");
        let mut hlayout = create_hlayout();
        hlayout.add_widget(platform_button.into_ptr());
        hlayout.add_widget(platform_combobox.into_ptr());

        grpbox.set_layout(hlayout.into_ptr());

        toolbar.add_widget(grpbox.into_ptr());
        (platform_cb_ptr, platform_icon)
    }
}
//
// Site Combobox
//
pub(crate) fn setup_sites_cb<'b>(
    toolbar: &mut MutPtr<QToolBar>,
) -> (MutPtr<QComboBox>, CppBox<QIcon>) {
    unsafe {
        let mut site_combobox = QComboBox::new_0a();
        site_combobox.set_object_name(&qs("SiteCB"));
        let site_cb_ptr = site_combobox.as_mut_ptr();
        site_combobox.add_item_q_string(&QString::from_std_str("any"));

        let mut grpbox = QFrame::new_0a();
        grpbox.set_object_name(&qs("ComboFrame"));

        let (site_button, site_icon) = create_label("Site", ":images/earth.png");
        let mut hlayout = create_hlayout();
        hlayout.add_widget(site_button.into_ptr());

        hlayout.add_widget(site_combobox.into_ptr());
        grpbox.set_layout(hlayout.into_ptr());
        toolbar.add_widget(grpbox.into_ptr());
        (site_cb_ptr, site_icon)
    }
}
//
// Set up the directions combobox
//
pub(crate) fn setup_directions_cb<'b>(
    toolbar: &mut MutPtr<QToolBar>,
) -> (MutPtr<QComboBox>, CppBox<QIcon>) {
    unsafe {
        let mut dir_combobox = QComboBox::new_0a();
        dir_combobox.set_object_name(&qs("DirCB"));
        let dir_cb_ptr = dir_combobox.as_mut_ptr();
        for r in &["ancestor", "exact", "descendant"] {
            dir_combobox.add_item_q_string(&QString::from_std_str(r));
        }
        dir_combobox.set_current_index(2);
        let mut grpbox = QFrame::new_0a();
        grpbox.set_object_name(&qs("ComboFrame"));

        let (dir_button, dir_icon) = create_label("Direction", ":images/wheel.png");
        let mut hlayout = create_hlayout();
        hlayout.add_widget(dir_button.into_ptr());

        hlayout.add_widget(dir_combobox.into_ptr());
        grpbox.set_layout(hlayout.into_ptr());
        toolbar.add_widget(grpbox.into_ptr());
        (dir_cb_ptr, dir_icon)
    }
}

unsafe fn create_label(name: &str, icon: &str) -> (CppBox<QToolButton>, CppBox<QIcon>) {
    let mut btn_icon = QIcon::new();
    let size = QSize::new_0a();
    btn_icon.add_file_4a(&qs(icon), &size, Mode::Disabled, State::On);
    let mut button = QToolButton::new_0a();
    button.set_object_name(&qs("CBToolButtonAsLabel"));
    button.set_tool_button_style(ToolButtonStyle::ToolButtonTextBesideIcon);
    button.set_icon(btn_icon.as_ref());
    button.set_text(&qs(name));
    button.set_enabled(false);

    (button, btn_icon)
}
