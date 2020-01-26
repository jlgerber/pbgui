use qt_core::QString;
use qt_widgets::{
    cpp_core::{MutPtr, StaticUpcast},
    QComboBox, QFrame, QLabel, QLayout, QPushButton,
};
use rustqt_utils::{create_hlayout, qs};

//------------------------//
// build the combo boxes  //
//------------------------//
pub fn create<'b, T>(
    layout: &mut MutPtr<T>,
) -> (
    MutPtr<QComboBox>,
    MutPtr<QComboBox>,
    MutPtr<QComboBox>,
    MutPtr<QComboBox>,
    MutPtr<QComboBox>,
)
where
    T: StaticUpcast<QLayout>,
{
    unsafe {
        let mut layout = layout.static_upcast_mut();
        //results
        let level_cb_ptr = setup_levels_cb(&mut layout);
        // Roles
        let role_cb_ptr = setup_roles_cb(&mut layout);
        // Platform
        let platform_cb_ptr = setup_platforms_cb(&mut layout);
        // Site
        let site_cb_ptr = setup_sites_cb(&mut layout);
        // Direction
        let dir_cb_ptr = setup_directions_cb(&mut layout);

        (
            level_cb_ptr,
            role_cb_ptr,
            platform_cb_ptr,
            site_cb_ptr,
            dir_cb_ptr,
        )
    }
}

// Setup Levels Combobox
//
unsafe fn setup_levels_cb<'b>(layout: &mut MutPtr<QLayout>) -> MutPtr<QComboBox> {
    //results
    let mut level_combobox = QComboBox::new_0a();
    level_combobox.set_object_name(&qs("LevelCB"));
    let level_cb_ptr = level_combobox.as_mut_ptr();
    level_combobox.add_item_q_string(&QString::from_std_str("facility"));
    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("FirstComboFrame"));
    let mut pxlabel = QPushButton::from_q_string(&QString::from_std_str(""));
    pxlabel.set_object_name(&qs("LevelIcon"));
    let label = QLabel::from_q_string(&qs("Level"));
    let mut hlayout = create_hlayout();
    //let hlayout_ptr = hlayout.as_mut_ptr();
    hlayout.add_widget(pxlabel.into_ptr());
    hlayout.add_widget(label.into_ptr());
    // assign owner of level
    // let align: QFlags<AlignmentFlag> = AlignmentFlag::AlignBottom.into();
    hlayout.add_widget(level_combobox.into_ptr());
    // hlayout.set_alignment_q_layout_q_flags_alignment_flag(hlayout_ptr, align);
    //hlayout.set_alignment_q_widget_q_flags_alignment_flag(level_cb_ptr, align);
    grpbox.set_layout(hlayout.into_ptr());
    layout.add_widget(grpbox.into_ptr());
    level_cb_ptr
}

//
// set up the roles combobox
//
unsafe fn setup_roles_cb<'b>(layout: &mut MutPtr<QLayout>) -> MutPtr<QComboBox> {
    let mut role_combobox = QComboBox::new_0a();
    role_combobox.set_object_name(&qs("RoleCB"));
    let role_cb_ptr = role_combobox.as_mut_ptr();
    role_combobox.add_item_q_string(&QString::from_std_str("any"));
    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("ComboFrame"));
    let mut pxlabel = QPushButton::from_q_string(&QString::from_std_str(""));
    pxlabel.set_object_name(&qs("RoleIcon"));
    let label = QLabel::from_q_string(&qs("Role"));
    let mut hlayout = create_hlayout();
    hlayout.add_widget(pxlabel.into_ptr());
    hlayout.add_widget(label.into_ptr());
    hlayout.add_widget(role_combobox.into_ptr());
    grpbox.set_layout(hlayout.into_ptr());
    layout.add_widget(grpbox.into_ptr());
    role_cb_ptr
}
//------------------//
// setup Platforms  //
//------------------//
unsafe fn setup_platforms_cb<'b>(layout: &mut MutPtr<QLayout>) -> MutPtr<QComboBox> {
    let mut platform_combobox = QComboBox::new_0a();
    platform_combobox.set_object_name(&qs("PlatformCB"));
    let platform_cb_ptr = platform_combobox.as_mut_ptr();
    platform_combobox.add_item_q_string(&QString::from_std_str("any"));

    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("ComboFrame"));

    let mut pxlabel = QPushButton::from_q_string(&QString::from_std_str(""));
    pxlabel.set_object_name(&qs("PlatformIcon"));

    let label = QLabel::from_q_string(&qs("Platform"));

    let mut hlayout = create_hlayout();
    hlayout.add_widget(pxlabel.into_ptr());
    hlayout.add_widget(label.into_ptr());
    hlayout.add_widget(platform_combobox.into_ptr());

    grpbox.set_layout(hlayout.into_ptr());

    layout.add_widget(grpbox.into_ptr());
    platform_cb_ptr
}
//
// Site Combobox
//
unsafe fn setup_sites_cb<'b>(layout: &mut MutPtr<QLayout>) -> MutPtr<QComboBox> {
    let mut site_combobox = QComboBox::new_0a();
    site_combobox.set_object_name(&qs("SiteCB"));
    let site_cb_ptr = site_combobox.as_mut_ptr();
    site_combobox.add_item_q_string(&QString::from_std_str("any"));

    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("ComboFrame"));

    let mut pxlabel = QPushButton::from_q_string(&QString::from_std_str(""));
    pxlabel.set_object_name(&qs("SiteIcon"));
    let label = QLabel::from_q_string(&qs("Site"));

    let mut hlayout = create_hlayout();
    hlayout.add_widget(pxlabel.into_ptr());
    hlayout.add_widget(label.into_ptr());
    hlayout.add_widget(site_combobox.into_ptr());
    grpbox.set_layout(hlayout.into_ptr());
    layout.add_widget(grpbox.into_ptr());
    site_cb_ptr
}
//
// Set up the directions combobox
//
unsafe fn setup_directions_cb<'b>(layout: &mut MutPtr<QLayout>) -> MutPtr<QComboBox> {
    let mut dir_combobox = QComboBox::new_0a();
    dir_combobox.set_object_name(&qs("DirCB"));
    let dir_cb_ptr = dir_combobox.as_mut_ptr();
    for r in &["ancestor", "exact", "descendant"] {
        dir_combobox.add_item_q_string(&QString::from_std_str(r));
    }
    dir_combobox.set_current_index(2);
    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("ComboFrame"));
    let label = QLabel::from_q_string(&qs("Direction"));
    let mut hlayout = create_hlayout();
    hlayout.add_widget(label.into_ptr());
    hlayout.add_widget(dir_combobox.into_ptr());
    grpbox.set_layout(hlayout.into_ptr());
    layout.add_widget(grpbox.into_ptr());
    dir_cb_ptr
}
