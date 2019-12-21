use crate::utility::{create_hlayout, qs};
use packybara::packrat::PackratDb;
use qt_widgets::{
    cpp_core::MutPtr, qt_core::QString, QComboBox, QFrame, QHBoxLayout, QLabel, QPushButton,
};
//let icon = QIcon::from_q_string(&QString::from_std_str(
//    ":/images/icons8-volume-level-50.png",
//));
//------------------------//
// build the combo boxes  //
//------------------------//
pub fn create<'b>(
    db: &'b mut PackratDb,
    layout: &mut MutPtr<QHBoxLayout>,
) -> (
    MutPtr<QComboBox>,
    MutPtr<QComboBox>,
    MutPtr<QComboBox>,
    MutPtr<QComboBox>,
    MutPtr<QComboBox>,
) {
    unsafe {
        //results
        let level_cb_ptr = setup_levels_cb(db, layout);
        // Roles
        let role_cb_ptr = setup_roles_cb(db, layout);
        // Platform
        let platform_cb_ptr = setup_platforms_cb(db, layout);
        // Site
        let site_cb_ptr = setup_sites_cb(db, layout);
        // Direction
        let dir_cb_ptr = setup_directions_cb(layout);

        // return tuple
        (
            level_cb_ptr,
            role_cb_ptr,
            platform_cb_ptr,
            site_cb_ptr,
            dir_cb_ptr,
        )
    }
}
//------------------------//
// Setup Levels Combobox  //
//------------------------//
unsafe fn setup_levels_cb<'b>(
    db: &'b mut PackratDb,
    layout: &mut MutPtr<QHBoxLayout>,
) -> MutPtr<QComboBox> {
    //results
    let mut level_combobox = QComboBox::new_0a();
    level_combobox.set_object_name(&qs("TopCB"));
    let level_cb_ptr = level_combobox.as_mut_ptr();
    // LEVELS
    let results = db
        .find_all_levels()
        .query()
        .expect("unable to find_all_levels");
    level_combobox.add_item_q_string(&QString::from_std_str("facility"));
    results
        .iter()
        .filter(|s| s.level.as_str() != "facility")
        .for_each(|s| level_combobox.add_item_q_string(&QString::from_std_str(s.level.as_str())));
    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("FirstComboWidget"));
    //let pixmap = icon.pixmap_q_size(&QSize::new_2a(50, 50));
    let mut pxlabel = QPushButton::from_q_string(&QString::from_std_str(""));
    pxlabel.set_object_name(&qs("LevelIcon"));
    pxlabel.set_minimum_width(24); //70
    pxlabel.set_maximum_width(24); //70
    pxlabel.set_minimum_height(24);
    let label = QLabel::from_q_string(&qs("Level"));
    let mut hlayout = create_hlayout();
    hlayout.add_widget(pxlabel.into_ptr());
    hlayout.add_widget(label.into_ptr());
    // assign owner of level
    hlayout.add_widget(level_combobox.into_ptr());
    //layout.add_item(hlayout.into_ptr());
    grpbox.set_layout(hlayout.into_ptr());
    layout.add_widget(grpbox.into_ptr());

    //grpbox.set_title(&QString::from_std_str("Show"));
    level_cb_ptr
}
//----------------------------//
// set up the roles combobox  //
//----------------------------//
unsafe fn setup_roles_cb<'b>(
    db: &'b mut PackratDb,
    layout: &mut MutPtr<QHBoxLayout>,
) -> MutPtr<QComboBox> {
    let mut role_combobox = QComboBox::new_0a();
    role_combobox.set_object_name(&qs("TopCB"));
    let role_cb_ptr = role_combobox.as_mut_ptr();
    let results = db
        .find_all_roles()
        .query()
        .expect("unable to find all roles");
    role_combobox.add_item_q_string(&QString::from_std_str("any"));
    results
        .iter()
        .filter(|s| s.role.as_str() != "any")
        .for_each(|s| role_combobox.add_item_q_string(&QString::from_std_str(s.role.as_str())));
    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("ComboWidget"));
    let mut pxlabel = QPushButton::from_q_string(&QString::from_std_str(""));
    pxlabel.set_object_name(&qs("RoleIcon"));
    pxlabel.set_minimum_width(24); //70
    pxlabel.set_maximum_width(24); //70
    pxlabel.set_minimum_height(24);
    let label = QLabel::from_q_string(&qs("Role"));
    let mut hlayout = create_hlayout();
    hlayout.add_widget(pxlabel.into_ptr());
    hlayout.add_widget(label.into_ptr());
    hlayout.add_widget(role_combobox.into_ptr());
    grpbox.set_layout(hlayout.into_ptr());
    //grpbox.set_title(&QString::from_std_str("Role"));
    layout.add_widget(grpbox.into_ptr());
    role_cb_ptr
}
//------------------//
// setup Platforms  //
//------------------//
unsafe fn setup_platforms_cb<'b>(
    db: &'b mut PackratDb,
    layout: &mut MutPtr<QHBoxLayout>,
) -> MutPtr<QComboBox> {
    let mut platform_combobox = QComboBox::new_0a();
    platform_combobox.set_object_name(&qs("TopCB"));
    let platform_cb_ptr = platform_combobox.as_mut_ptr();
    let results = db
        .find_all_platforms()
        .query()
        .expect("unable to find_all_platforms");
    for r in results {
        let platform_str = r.name.as_str();
        platform_combobox.add_item_q_string(&QString::from_std_str(platform_str));
    }
    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("ComboWidget"));
    let mut pxlabel = QPushButton::from_q_string(&QString::from_std_str(""));
    pxlabel.set_object_name(&qs("PlatformIcon"));
    pxlabel.set_minimum_width(24);
    pxlabel.set_maximum_width(24);
    pxlabel.set_minimum_height(24);
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
unsafe fn setup_sites_cb<'b>(
    db: &'b mut PackratDb,
    layout: &mut MutPtr<QHBoxLayout>,
) -> MutPtr<QComboBox> {
    let mut site_combobox = QComboBox::new_0a();
    site_combobox.set_object_name(&qs("TopCB"));
    let site_cb_ptr = site_combobox.as_mut_ptr();
    let results = db
        .find_all_sites()
        .query()
        .expect("unable to find all sites");
    site_combobox.add_item_q_string(&QString::from_std_str("any"));
    for r in results {
        let site_str = r.name.as_str();
        site_combobox.add_item_q_string(&QString::from_std_str(site_str));
    }
    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("ComboWidget"));
    let mut pxlabel = QPushButton::from_q_string(&QString::from_std_str(""));
    pxlabel.set_object_name(&qs("SiteIcon"));
    pxlabel.set_minimum_width(24);
    pxlabel.set_maximum_width(24);
    pxlabel.set_minimum_height(24);
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
unsafe fn setup_directions_cb<'b>(layout: &mut MutPtr<QHBoxLayout>) -> MutPtr<QComboBox> {
    let mut dir_combobox = QComboBox::new_0a();
    dir_combobox.set_object_name(&qs("TopCB"));
    let dir_cb_ptr = dir_combobox.as_mut_ptr();
    for r in &["ancestor", "exact", "descendant"] {
        dir_combobox.add_item_q_string(&QString::from_std_str(r));
    }
    let mut grpbox = QFrame::new_0a();
    grpbox.set_object_name(&qs("ComboWidget"));
    let label = QLabel::from_q_string(&qs("Direction"));
    let mut hlayout = create_hlayout();
    hlayout.add_widget(label.into_ptr());
    hlayout.add_widget(dir_combobox.into_ptr());
    grpbox.set_layout(hlayout.into_ptr());
    layout.add_widget(grpbox.into_ptr());
    dir_cb_ptr
}
