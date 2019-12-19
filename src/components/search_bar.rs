use packybara::packrat::PackratDb;
use qt_core::{AlignmentFlag, QFlags};
use qt_widgets::{
    cpp_core::MutPtr, qt_core::QString, QComboBox, QGroupBox, QHBoxLayout, QPushButton,
};

//------------------------//
// build the combo boxes  //
//------------------------//
pub fn combo_boxes<'b>(
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
    let mut grpbox = QGroupBox::new();
    let mut hlayout = QHBoxLayout::new_0a();
    // assign owner of level
    hlayout.add_widget_3a(
        level_combobox.into_ptr(),
        1,
        QFlags::from(AlignmentFlag::AlignBottom),
    );
    grpbox.set_layout(hlayout.into_ptr());
    grpbox.set_title(&QString::from_std_str("Show"));
    layout.add_widget(grpbox.into_ptr());
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
    let mut grpbox = QGroupBox::new();
    let mut hlayout = QHBoxLayout::new_0a();
    hlayout.add_widget_3a(
        role_combobox.into_ptr(),
        1,
        QFlags::from(AlignmentFlag::AlignBottom),
    );
    grpbox.set_layout(hlayout.into_ptr());
    grpbox.set_title(&QString::from_std_str("Role"));
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
    let platform_cb_ptr = platform_combobox.as_mut_ptr();
    let results = db
        .find_all_platforms()
        .query()
        .expect("unable to find_all_platforms");
    for r in results {
        let platform_str = r.name.as_str();
        platform_combobox.add_item_q_string(&QString::from_std_str(platform_str));
    }
    let mut grpbox = QGroupBox::new();
    let mut hlayout = QHBoxLayout::new_0a();
    hlayout.add_widget_3a(
        platform_combobox.into_ptr(),
        1,
        QFlags::from(AlignmentFlag::AlignBottom),
    );
    grpbox.set_title(&QString::from_std_str("Platform"));
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
    let mut grpbox = QGroupBox::new();
    let mut hlayout = QHBoxLayout::new_0a();
    hlayout.add_widget_3a(
        site_combobox.into_ptr(),
        1,
        QFlags::from(AlignmentFlag::AlignBottom),
    );
    grpbox.set_layout(hlayout.into_ptr());
    grpbox.set_title(&QString::from_std_str("Site"));
    layout.add_widget(grpbox.into_ptr());
    site_cb_ptr
}
//
// Set up the directions combobox
//
unsafe fn setup_directions_cb<'b>(layout: &mut MutPtr<QHBoxLayout>) -> MutPtr<QComboBox> {
    let mut dir_combobox = QComboBox::new_0a();
    let dir_cb_ptr = dir_combobox.as_mut_ptr();
    for r in &["ancestor", "exact", "descendant"] {
        dir_combobox.add_item_q_string(&QString::from_std_str(r));
    }
    let mut grpbox = QGroupBox::new();
    let mut hlayout = QHBoxLayout::new_0a();
    hlayout.add_widget_3a(
        dir_combobox.into_ptr(),
        1,
        QFlags::from(AlignmentFlag::AlignBottom),
    );
    grpbox.set_layout(hlayout.into_ptr());
    grpbox.set_title(&QString::from_std_str("Direction"));
    layout.add_widget(grpbox.into_ptr());
    dir_cb_ptr
}

//
// Create Query Button
//
pub fn create_query_button(hlayout_ptr: &mut MutPtr<QHBoxLayout>) -> MutPtr<QPushButton> {
    unsafe {
        let mut button = QPushButton::from_q_string(&QString::from_std_str("")); //Query
        button.set_object_name(&QString::from_std_str("QueryButton"));
        let button_ptr = button.as_mut_ptr();
        button.set_minimum_width(60); //70
        button.set_maximum_width(60); //70
        button.set_minimum_height(60); //60
        hlayout_ptr.add_widget(button.into_ptr());
        button_ptr
    }
}
