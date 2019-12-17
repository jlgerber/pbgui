#![windows_subsystem = "windows"]
use packybara::db::update::versionpins::VersionPinChange;
use packybara::packrat::{Client, NoTls, PackratDb};
use pbgui::constants::*;
use pbgui::utility::qs;
use pbgui::versionpin_changes_table::create_pinchanges_widget;
use pbgui::versionpin_table::{setup_table, update_vpin_table};
use qt_core::{
    q_io_device::OpenModeFlag, AlignmentFlag, Orientation, QFile, QFlags, QListOfInt, QPoint,
    QResource, QTextStream, QVariant, WidgetAttribute,
};
use qt_gui::{QBrush, QColor, QIcon};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, /*Ptr,*/ Ref},
    q_line_edit::ActionPosition,
    qt_core::ContextMenuPolicy,
    qt_core::QString,
    qt_core::QStringList,
    qt_core::Slot,
    QAction, QApplication, QComboBox, QGroupBox, QHBoxLayout, QInputDialog, QLineEdit, QMenu,
    QMessageBox, QPushButton, QSplitter, QTableWidget, QTableWidgetItem, QVBoxLayout, QWidget,
    SlotOfQPoint,
};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use whoami;

macro_rules! qcolor_blue {
    () => {
        QColor::from_rgb_3a(100, 150, 255)
    };
}

struct Form<'a> {
    _db: &'a mut PackratDb,
    _widget: CppBox<QWidget>,
    _query_button: MutPtr<QPushButton>,
    _pkg_line_edit: MutPtr<QLineEdit>,
    _vpin_table: MutPtr<QTableWidget>,
    _pinchanges_list: MutPtr<QTableWidget>,
    _save_button: MutPtr<QPushButton>,
    dist_popup_menu: CppBox<QMenu>,
    package_popup_menu: CppBox<QMenu>,
    // needed so that qt wont segfault
    dist_popup_action: MutPtr<QAction>,
    query_button_clicked: Slot<'a>,
    save_clicked: Slot<'a>,
    choose_distribution_triggered: Slot<'a>,
    show_dist_menu: SlotOfQPoint<'a>,
    clear_package: Slot<'a>,
    show_line_edit_menu: SlotOfQPoint<'a>,
}

impl<'a> Form<'a> {
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
        let results = db.find_all_levels().query().unwrap();
        level_combobox.add_item_q_string(&QString::from_std_str("facility"));
        results
            .iter()
            .filter(|s| s.level.as_str() != "facility")
            .for_each(|s| {
                level_combobox.add_item_q_string(&QString::from_std_str(s.level.as_str()))
            });
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
        let results = db.find_all_roles().query().unwrap();
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
        let results = db.find_all_platforms().query().unwrap();
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
    //----------------//
    // Site Combobox  //
    //----------------//
    unsafe fn setup_sites_cb<'b>(
        db: &'b mut PackratDb,
        layout: &mut MutPtr<QHBoxLayout>,
    ) -> MutPtr<QComboBox> {
        let mut site_combobox = QComboBox::new_0a();
        let site_cb_ptr = site_combobox.as_mut_ptr();
        let results = db.find_all_sites().query().unwrap();
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
    //---------------------------------//
    // Set up the directions combobox  //
    //---------------------------------//
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
    //------------------------//
    // build the combo boxes  //
    //------------------------//
    unsafe fn combo_boxes<'b>(
        db: &'b mut PackratDb,
        layout: &mut MutPtr<QHBoxLayout>,
    ) -> (
        MutPtr<QComboBox>,
        MutPtr<QComboBox>,
        MutPtr<QComboBox>,
        MutPtr<QComboBox>,
        MutPtr<QComboBox>,
    ) {
        //results
        let level_cb_ptr = Self::setup_levels_cb(db, layout);
        // Roles
        let role_cb_ptr = Self::setup_roles_cb(db, layout);
        // Platform
        let platform_cb_ptr = Self::setup_platforms_cb(db, layout);
        // Site
        let site_cb_ptr = Self::setup_sites_cb(db, layout);
        // Direction
        let dir_cb_ptr = Self::setup_directions_cb(layout);

        //let qspacer = QSpacerItem::new_3a(30, 10, Policy::Expanding);
        //layout.add_item(qspacer.into_ptr());
        // return tuple
        (
            level_cb_ptr,
            role_cb_ptr,
            platform_cb_ptr,
            site_cb_ptr,
            dir_cb_ptr,
        )
    }
    //----------------------//
    // Create Query Button  //
    //----------------------//
    unsafe fn create_query_button(hlayout_ptr: &mut MutPtr<QHBoxLayout>) -> MutPtr<QPushButton> {
        let mut button = QPushButton::from_q_string(&QString::from_std_str("")); //Query
        button.set_object_name(&QString::from_std_str("QueryButton"));
        let button_ptr = button.as_mut_ptr();
        button.set_minimum_width(60); //70
        button.set_maximum_width(60); //70
        button.set_minimum_height(60); //60
        hlayout_ptr.add_widget(button.into_ptr());
        button_ptr
    }
    //---------------

    unsafe fn get_coords_from_row(
        row_widget: &mut MutPtr<QTableWidget>,
        row: i32,
    ) -> (
        CppBox<QString>,
        CppBox<QString>,
        CppBox<QString>,
        CppBox<QString>,
        i32,
        i32,
        i32,
    ) {
        //level
        let level = row_widget.item(row, COL_LEVEL).text();
        let role = row_widget.item(row, COL_ROLE).text();
        let platform = row_widget.item(row, COL_PLATFORM).text();
        let site = row_widget.item(row, COL_SITE).text();
        let vpin_id = row_widget.item(row, COL_ID).data(2);
        let dist_id = row_widget.item(row, COL_DISTRIBUTION_ID).data(2);
        let pkgcoord_id = row_widget.item(row, COL_PKGCOORD_ID).data(2);

        (
            level,
            role,
            platform,
            site,
            vpin_id.to_int_0a(),
            dist_id.to_int_0a(),
            pkgcoord_id.to_int_0a(),
        )
    }
    //------------------------------------//
    // choose_alternative_distribution    //
    //------------------------------------//
    // button double click Slot delegates //
    // the work to this function          //
    //------------------------------------//
    unsafe fn choose_alternative_distribution(
        r: i32,
        mut vpin_tablewidget_ptr: MutPtr<QTableWidget>,
        usage_ptr: Rc<RefCell<HashMap<i32, i32>>>,
        root_widget_ptr: MutPtr<QWidget>,
        mut pinchanges_ptr: MutPtr<QTableWidget>,
        update_cnt_ptr: Rc<Cell<i32>>,
    ) {
        let mut dist_item = vpin_tablewidget_ptr.item(r, COL_DISTRIBUTION);
        let mut orig_qstr = dist_item.text();
        let orig_text = orig_qstr.to_std_string();
        // split up the distribution into the package name
        // and the version
        let (package, version) =
            if let &[package, version] = &*orig_text.split("-").collect::<Vec<_>>() {
                (package, version)
            } else {
                panic!("unable to extract packge and version from row");
            };
        let client = Client::connect(
            "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
            NoTls,
        )
        .unwrap();
        let mut packratdb = PackratDb::new(client);
        let results = packratdb
            .find_all_distributions()
            .package(package)
            .query()
            .unwrap();
        let mut qsl = QStringList::new();
        let mut idx = 0;
        let mut cnt = 0;
        let mut dist_versions = HashMap::new();
        for r in results {
            if r.version == version {
                idx = cnt;
            }
            cnt += 1;
            dist_versions.insert(r.version.clone(), r.id);
            qsl.append_q_string(&QString::from_std_str(r.version));
        }
        let mut ok_or_cancel = false;
        let ok_or_cancel_ptr = MutPtr::from_raw(&mut ok_or_cancel);
        // Get New version by popping up a Dialog
        let new_version = QInputDialog::get_item_7a(
            root_widget_ptr,
            &QString::from_std_str("Pick Version"),
            &QString::from_std_str(package),
            &qsl,
            idx,
            false,
            ok_or_cancel_ptr,
        );
        if *ok_or_cancel_ptr == false {
            println!("cancelled");
        } else {
            let value = new_version.to_std_string();
            let new_dist_id = dist_versions.get(value.as_str()).unwrap();
            let new_value = format!("{}-{}", package, value);
            if orig_text == new_value {
                println!("new value and old value match. Skipping");
                return;
            }
            let (level, role, platform, site, vpin_id, dist_id, pkgcoord_id) =
                Self::get_coords_from_row(&mut vpin_tablewidget_ptr, r);
            let new_value_qstr = QString::from_std_str(new_value);
            // build up new string
            dist_item.set_text(&new_value_qstr);
            orig_qstr.append_q_string(&QString::from_std_str("   ->   "));
            orig_qstr.append_q_string(&new_value_qstr);
            orig_qstr.append_q_string(&QString::from_std_str(format!(
                "     ({}, {}, {}, {})     distribution id: {}     pkgcoord id: {}",
                level.to_std_string(),
                role.to_std_string(),
                platform.to_std_string(),
                site.to_std_string(),
                dist_id,
                pkgcoord_id
            )));

            if usage_ptr.borrow().contains_key(&dist_id) {
                let row = usage_ptr.borrow();
                let row = row.get(&dist_id).unwrap();
                let mut item = pinchanges_ptr.item(*row, COL_PC_DISPLAY);
                item.set_text(&orig_qstr);
            } else {
                let row_cnt = pinchanges_ptr.row_count() + 1;
                pinchanges_ptr.set_row_count(row_cnt);
                // VPIN ID
                let mut pinchanges_item = QTableWidgetItem::new();
                let variant = QVariant::from_int(vpin_id);
                pinchanges_item.set_data(
                    2, // EditRole
                    variant.as_ref(),
                );
                pinchanges_ptr.set_item(row_cnt - 1, COL_PC_VPINID, pinchanges_item.into_ptr());
                // DIST ID
                let mut pinchanges_item = QTableWidgetItem::new();
                let variant = QVariant::from_int(*new_dist_id);
                pinchanges_item.set_data(
                    2, // EditRole
                    variant.as_ref(),
                );
                pinchanges_ptr.set_item(row_cnt - 1, COL_PC_DISTID, pinchanges_item.into_ptr());
                // PKGCOORD ID
                let mut pinchanges_item = QTableWidgetItem::new();
                let variant = QVariant::from_int(pkgcoord_id);
                pinchanges_item.set_data(
                    2, // EditRole
                    variant.as_ref(),
                );
                pinchanges_ptr.set_item(row_cnt - 1, COL_PC_PKGCOORDID, pinchanges_item.into_ptr());
                // DISPLAY
                let pinchanges_item = QTableWidgetItem::from_q_string(&orig_qstr);
                pinchanges_ptr.set_item(row_cnt - 1, COL_PC_DISPLAY, pinchanges_item.into_ptr());

                let update_color = qcolor_blue!();
                dist_item.set_foreground(&QBrush::from_q_color(update_color.as_ref()));
                dist_item.table_widget().clear_selection();
                let idx = update_cnt_ptr.get();
                usage_ptr.borrow_mut().insert(dist_id, idx);
                update_cnt_ptr.set(idx + 1);
            }
        }
    }
    fn load_stylesheet(mut parent_widget: MutPtr<QWidget>) {
        unsafe {
            // Does not work
            //QResource::add_search_path(&QString::from_std_str("/Users/jgerber/bin/"));
            let _result = QResource::register_resource_q_string(&QString::from_std_str(
                "/Users/jgerber/bin/pbgui.rcc",
            ));
            //println!("Loading resource successful?: {}", result);
            let mut file =
                QFile::from_q_string(&QString::from_std_str("/Users/jgerber/bin/pbgui.qss"));
            if file.open_1a(QFlags::from(OpenModeFlag::ReadOnly)) {
                let mut text_stream = QTextStream::new();
                text_stream.set_device(file.as_mut_ptr());
                let stylesheet = text_stream.read_all();
                parent_widget.set_style_sheet(stylesheet.as_ref());
            } else {
                println!("stylesheet not found");
            }
        }
    }
    //--------------------//
    // Create Main Widget //
    //--------------------//
    fn new(mut db: &'a mut PackratDb) -> Form<'a> {
        unsafe {
            // parent root_widget
            let mut root_widget = QWidget::new_0a();
            root_widget.set_base_size_2a(1200, 800);
            let root_widget_ptr = root_widget.as_mut_ptr();
            // top vertical layout
            let mut root_layout = QVBoxLayout::new_0a();
            let mut root_layout_ptr = root_layout.as_mut_ptr();
            root_widget.set_layout(root_layout.into_ptr());
            // header layout
            let mut hlayout = QHBoxLayout::new_0a();
            let mut hlayout_ptr = hlayout.as_mut_ptr();
            root_layout_ptr.add_layout_1a(hlayout.into_ptr());
            // setup comboboxes in header
            let (level_ptr, role_ptr, platform_ptr, site_ptr, dir_ptr) =
                Self::combo_boxes(&mut db, &mut hlayout_ptr);
            // LINE EDIT
            let mut line_edit = QLineEdit::new();
            line_edit.set_attribute_2a(WidgetAttribute::WAMacShowFocusRect, false);
            line_edit.set_object_name(&QString::from_std_str("packageLineEdit"));
            let clear_icon = QIcon::from_q_string(&QString::from_std_str(":/images/clear.png"));
            println!("icon is null {}", clear_icon.is_null());
            let clear_action = line_edit.add_action_q_icon_action_position(
                clear_icon.as_ref(),
                ActionPosition::TrailingPosition,
            );
            line_edit.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
            let mut line_edit_popup_menu = QMenu::new();
            let mut line_edit_popup_menu_ptr = line_edit_popup_menu.as_mut_ptr();
            let choose_line_edit_clear_action =
                line_edit_popup_menu.add_action_q_string(&QString::from_std_str("Clear"));

            let mut line_edit_ptr = line_edit.as_mut_ptr();
            //root_layout_ptr.add_widget(line_edit.into_ptr());

            hlayout_ptr.add_widget(line_edit.into_ptr());
            // create query button
            let button_ptr = Self::create_query_button(&mut hlayout_ptr);
            // Create Splitter between query results and action logger
            let mut vsplit = QSplitter::new();
            let mut vsplit_ptr = vsplit.as_mut_ptr();
            vsplit.set_orientation(Orientation::Vertical);
            // set splitter sizing
            // setup the main table widget
            let vpin_tablewidget_ptr = setup_table(&mut vsplit_ptr);
            let (pinchanges_ptr, save_button) = create_pinchanges_widget(&mut vsplit_ptr);
            // setup popup menu for versionpin table
            let mut dist_popup_menu = QMenu::new();
            let choose_dist_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Change Version"));
            let _choose_withs_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Withs"));
            let mut dist_popup_menu_ptr = dist_popup_menu.as_mut_ptr();
            // set the style sheet
            Self::load_stylesheet(root_widget_ptr);
            root_widget.show();
            //
            let usage = Rc::new(RefCell::new(HashMap::<i32, i32>::new()));
            let usage_ptr = Rc::clone(&usage);
            let update_cnt = Rc::new(Cell::new(0));
            let update_cnt_ptr = Rc::clone(&update_cnt);
            let mut pinchanges_ptr = pinchanges_ptr.clone();
            let dist_usage_ptr = usage_ptr.clone();
            let dist_update_cnt_ptr = update_cnt_ptr.clone();
            let mut splitter_sizes = QListOfInt::new();
            splitter_sizes.append_int(Ref::from_raw_ref(&(500 as i32)));
            splitter_sizes.append_int(Ref::from_raw_ref(&(300 as i32)));
            vsplit.set_sizes(&splitter_sizes);
            root_layout_ptr.add_widget(vsplit.into_ptr());
            let form = Form {
                clear_package: Slot::new(move || {
                    line_edit_ptr.clear();
                }),
                show_line_edit_menu: SlotOfQPoint::new(move |pos: Ref<QPoint>| {
                    let _action = line_edit_popup_menu_ptr
                        .exec_1a_mut(line_edit_ptr.map_to_global(pos).as_ref());
                }),
                show_dist_menu: SlotOfQPoint::new(move |pos: Ref<QPoint>| {
                    let _action = dist_popup_menu_ptr
                        .exec_1a_mut(vpin_tablewidget_ptr.map_to_global(pos).as_ref());
                }),
                // show_clear_menu: SlotOfQPoint::new(move |pos: Ref<QPoint>| {
                //     let _action = dist_popup_menu_ptr
                //         .exec_1a_mut(vpin_tablewidget_ptr.map_to_global(pos).as_ref());
                // }),
                //---------------------//
                // save clicked        //
                //---------------------//
                save_clicked: Slot::new(move || {
                    // grab all the data from the pin changes
                    let client = Client::connect(
                        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
                        NoTls,
                    )
                    .unwrap();
                    let comments = QInputDialog::get_multi_line_text_3a(
                        root_widget_ptr,
                        &qs("Save Changes"),
                        &qs("Comment"),
                    )
                    .to_std_string();
                    let mut pb = PackratDb::new(client);
                    let user = whoami::username();
                    let mut update = pb.update_versionpins(user.as_str(), comments.as_str());
                    let mut changes = Vec::new();
                    for row_idx in 0..pinchanges_ptr.row_count() {
                        let vpin_id = pinchanges_ptr.item(row_idx, COL_PC_VPINID).data(2);
                        let dist_id = pinchanges_ptr.item(row_idx, COL_PC_DISTID).data(2);
                        changes.push(VersionPinChange::new(
                            vpin_id.to_int_0a(),
                            Some(dist_id.to_int_0a()),
                            None,
                        ));
                    }
                    let results = update.changes(&mut changes).update();
                    if results.is_ok() {
                        pinchanges_ptr.clear();
                        pinchanges_ptr.set_row_count(0);
                        let mut mb = QMessageBox::new();
                        mb.set_text(&qs("Success"));
                        mb.exec();
                    //todo - reset color of query
                    } else {
                        let mut mb = QMessageBox::new();
                        mb.set_text(&qs("Error Occured"));
                        mb.set_detailed_text(&qs(format!("{:#?}", results)));
                        //println!("{:#?}", results);
                        mb.exec();
                    }
                }),
                //--------------------------------//
                // Add query_button_clicked Slot  //
                //--------------------------------//
                query_button_clicked: Slot::new(move || {
                    update_vpin_table(
                        dir_ptr,
                        line_edit_ptr,
                        level_ptr,
                        role_ptr,
                        platform_ptr,
                        site_ptr,
                        vpin_tablewidget_ptr,
                    );
                }),
                //
                // choose_distribution_triggered slot.
                //
                choose_distribution_triggered: Slot::new(move || {
                    let current_row = vpin_tablewidget_ptr.current_row();

                    Self::choose_alternative_distribution(
                        current_row,
                        vpin_tablewidget_ptr,
                        dist_usage_ptr.clone(),
                        root_widget_ptr,
                        pinchanges_ptr,
                        dist_update_cnt_ptr.clone(),
                    );
                }),
                _db: db,
                _widget: root_widget,
                _vpin_table: vpin_tablewidget_ptr,
                _query_button: button_ptr,
                _save_button: save_button,
                _pkg_line_edit: line_edit_ptr,
                _pinchanges_list: pinchanges_ptr,
                dist_popup_menu: dist_popup_menu,
                dist_popup_action: choose_dist_action,
                package_popup_menu: line_edit_popup_menu,
            };
            //
            // connect signals to slots
            //
            button_ptr.clicked().connect(&form.query_button_clicked);
            save_button.clicked().connect(&form.save_clicked);
            vpin_tablewidget_ptr
                .custom_context_menu_requested()
                .connect(&form.show_dist_menu);
            choose_dist_action
                .triggered()
                .connect(&form.choose_distribution_triggered);
            clear_action.triggered().connect(&form.clear_package);
            // line_edit_ptr
            //     .custom_context_menu_requested()
            //     .connect(&form.clear_package);
            line_edit_ptr
                .custom_context_menu_requested()
                .connect(&form.show_line_edit_menu);
            choose_line_edit_clear_action
                .triggered()
                .connect(&form.clear_package);
            form
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;

    let mut vpin_finder = PackratDb::new(client);
    QApplication::init(|_| unsafe {
        let mut _form = Form::new(&mut vpin_finder);
        QApplication::exec()
    });
}
