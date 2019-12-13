#![windows_subsystem = "windows"]
use packybara::packrat::{Client, NoTls, PackratDb};
use packybara::LtreeSearchMode;
use qt_core::{AlignmentFlag, QFlags};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    q_abstract_item_view::{EditTrigger, SelectionBehavior, SelectionMode},
    q_header_view::ResizeMode,
    q_size_policy::Policy,
    qt_core::QString,
    qt_core::QStringList,
    qt_core::Slot,
    qt_core::SlotOfIntInt,
    QApplication, QComboBox, QGroupBox, QHBoxLayout, QInputDialog, QLineEdit, QPushButton,
    QSpacerItem, QTableWidget, QTableWidgetItem, QVBoxLayout, QWidget,
};
use std::str::FromStr;

macro_rules! dark_grey_stripe {
    () => {
        "rgb(40,40,40)"
    };
}
macro_rules! light_grey_stripe {
    () => {
        "rgb(50,50,50)"
    };
}
macro_rules! table_text_color {
    () => {
        "rgb(200,200,200)"
    };
}
macro_rules! table_header_bg_color {
    () => {
        "rgb(80,80,80)"
    };
}
macro_rules! table_header_text_color {
    () => {
        "white"
    };
}
const COLUMNS: i32 = 7;

struct Form<'a> {
    _db: &'a mut PackratDb,
    _widget: CppBox<QWidget>,
    _query_button: MutPtr<QPushButton>,
    _pkg_line_edit: MutPtr<QLineEdit>,
    _vpin_table: MutPtr<QTableWidget>,
    button_clicked: Slot<'a>,
    row_double_clicked: SlotOfIntInt<'a>,
}

impl<'a> Form<'a> {
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
    // set up the roles combobox
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
    // Platforms
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
    // Site
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
    // Set up the directions combobox
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
    // build the combo boxes
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

        let qspacer = QSpacerItem::new_3a(30, 10, Policy::Expanding);
        layout.add_item(qspacer.into_ptr());
        // return tuple
        (
            level_cb_ptr,
            role_cb_ptr,
            platform_cb_ptr,
            site_cb_ptr,
            dir_cb_ptr,
        )
    }
    // setup the headers matching the provided header vector
    unsafe fn setup_table_headers(vpin_tablewidget: &mut MutPtr<QTableWidget>) {
        let headers = vec![
            "Id",
            "Distribution",
            "Level",
            "Role",
            "Platform",
            "Site",
            "Withs",
        ];
        for (cnt, val) in headers.into_iter().enumerate() {
            let vpin_table_widget_item =
                QTableWidgetItem::from_q_string(&QString::from_std_str(val));
            vpin_tablewidget
                .set_horizontal_header_item(cnt as i32, vpin_table_widget_item.into_ptr());
        }
    }
    // Setup the TableWidget
    unsafe fn setup_table(root_layout_ptr: &mut MutPtr<QVBoxLayout>) -> MutPtr<QTableWidget> {
        // create the tablewidget
        let mut vpin_tablewidget = QTableWidget::new_2a(0, COLUMNS);
        let mut vpin_tw_ptr = vpin_tablewidget.as_mut_ptr();
        root_layout_ptr.add_widget(vpin_tablewidget.into_ptr());
        // assign table to the root layout
        vpin_tw_ptr.vertical_header().hide();
        vpin_tw_ptr.set_selection_behavior(SelectionBehavior::SelectRows);
        vpin_tw_ptr.set_edit_triggers(QFlags::from(EditTrigger::NoEditTriggers));
        vpin_tw_ptr.set_selection_mode(SelectionMode::SingleSelection);
        vpin_tw_ptr
            .horizontal_header()
            .set_stretch_last_section(true);
        vpin_tw_ptr
            .horizontal_header()
            .set_section_resize_mode_1a(ResizeMode::Stretch);
        vpin_tw_ptr.set_show_grid(false);
        vpin_tw_ptr.set_alternating_row_colors(true);
        vpin_tw_ptr.set_style_sheet(&QString::from_std_str(concat!(
            "alternate-background-color:",
            light_grey_stripe!(),
            ";color:",
            table_text_color!(),
            ";background-color:",
            dark_grey_stripe!(),
            ";"
        )));
        vpin_tw_ptr
            .horizontal_header()
            .set_style_sheet(&QString::from_std_str(concat!(
                "background-color:",
                table_header_bg_color!(),
                ";color:",
                table_header_text_color!(),
                ";border: none; outline:none; border-left: 0px; border-right: 0px;"
            )));
        Self::setup_table_headers(&mut vpin_tw_ptr);

        vpin_tw_ptr
    }

    unsafe fn create_query_button(hlayout_ptr: &mut MutPtr<QHBoxLayout>) -> MutPtr<QPushButton> {
        let mut button = QPushButton::from_q_string(&QString::from_std_str("Query"));
        let button_ptr = button.as_mut_ptr();
        button.set_minimum_width(70);
        button.set_maximum_width(70);
        hlayout_ptr.add_widget(button.into_ptr());
        button_ptr
    }

    // New up a Form
    fn new(mut db: &'a mut PackratDb) -> Form<'a> {
        unsafe {
            // parent root_widget
            let mut root_widget = QWidget::new_0a();
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
            let line_edit_ptr = line_edit.as_mut_ptr();
            root_layout_ptr.add_widget(line_edit.into_ptr());
            // create query button
            let button_ptr = Self::create_query_button(&mut hlayout_ptr);
            // setup table widget
            let mut vpin_tablewidget_ptr = Self::setup_table(&mut root_layout_ptr);
            root_widget.show();

            let form = Form {
                row_double_clicked: SlotOfIntInt::new(move |r: i32, c: i32| {
                    println!("Cell double clicked: {} {}", r, c);
                    let dist_item = vpin_tablewidget_ptr.item(r, 1);
                    let text = dist_item.text().to_std_string();
                    let pieces = text.split("-").collect::<Vec<_>>();
                    assert_eq!(pieces.len(), 2);
                    println!("package: {} version: {}", pieces[0], pieces[1]);
                    let client = Client::connect(
                        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
                        NoTls,
                    )
                    .unwrap();
                    let mut packratdb = PackratDb::new(client);
                    let results = packratdb
                        .find_all_distributions()
                        .package(pieces[0])
                        .query()
                        .unwrap();
                    let mut qsl = QStringList::new();
                    let mut idx = 0;
                    let mut cnt = 0;
                    for r in results {
                        println!("version: {}", r.version);
                        if r.version == pieces[1] {
                            idx = cnt;
                        }
                        cnt += 1;
                        qsl.append_q_string(&QString::from_std_str(r.version));
                    }
                    let mut tf = false;
                    let tf_ptr = MutPtr::from_raw(&mut tf);
                    let new_version = QInputDialog::get_item_7a(
                        root_widget_ptr,
                        &QString::from_std_str("Pick Version"),
                        &QString::from_std_str(pieces[0]),
                        &qsl,
                        idx,
                        false,
                        tf_ptr,
                    );
                    if *tf_ptr == false {
                        println!("cancelled");
                    } else {
                        let value = new_version.to_std_string();
                        println!("value: {}", value);
                    }
                }),
                button_clicked: Slot::new(move || {
                    let dirtxt = dir_ptr.current_text().to_std_string();
                    let line_edit_txt = line_edit_ptr.text().to_std_string();
                    let showtxt = level_ptr.current_text().to_std_string();
                    let roletxt = role_ptr.current_text().to_std_string();
                    let platformtxt = platform_ptr.current_text().to_std_string();
                    let sitetxt = site_ptr.current_text().to_std_string();
                    // for now
                    let client = Client::connect(
                        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
                        NoTls,
                    )
                    .unwrap();
                    let mut packratdb = PackratDb::new(client);
                    let mut vpin_finder = packratdb.find_all_versionpins();

                    vpin_finder
                        .level(showtxt.as_str())
                        .role(roletxt.as_str())
                        .platform(platformtxt.as_str())
                        .site(sitetxt.as_str())
                        .search_mode(LtreeSearchMode::from_str(dirtxt.as_str()).unwrap());
                    let filter_package = if line_edit_txt != "" { true } else { false };
                    let results = vpin_finder.query().unwrap();
                    let mut cnt = 0;
                    vpin_tablewidget_ptr.set_sorting_enabled(false);
                    vpin_tablewidget_ptr.set_row_count(0);
                    vpin_tablewidget_ptr.set_row_count(results.len() as i32);
                    for result in results {
                        if filter_package && line_edit_txt != "" {
                            if !result
                                .distribution
                                .package()
                                .contains(line_edit_txt.as_str())
                            {
                                continue;
                            }
                        }
                        // ID TODO: get it sorting correctly... (data)
                        let mut vpin_table_widget_item = QTableWidgetItem::new();
                        vpin_table_widget_item.set_text(&QString::from_std_str(
                            result.versionpin_id.to_string().as_str(),
                        ));
                        vpin_tablewidget_ptr.set_item(cnt, 0, vpin_table_widget_item.into_ptr());
                        // DISTRIBUTION
                        let mut vpin_table_widget_item = QTableWidgetItem::new();
                        vpin_table_widget_item.set_text(&QString::from_std_str(
                            result.distribution.to_string().as_str(),
                        ));
                        vpin_tablewidget_ptr.set_item(cnt, 1, vpin_table_widget_item.into_ptr());
                        // LEVEL
                        let mut vpin_table_widget_item = QTableWidgetItem::new();
                        vpin_table_widget_item.set_text(&QString::from_std_str(
                            result.coords.level.to_string().as_str(),
                        ));
                        vpin_tablewidget_ptr.set_item(cnt, 2, vpin_table_widget_item.into_ptr());
                        // ROLE
                        let mut vpin_table_widget_item = QTableWidgetItem::new();
                        vpin_table_widget_item.set_text(&QString::from_std_str(
                            result.coords.role.to_string().as_str(),
                        ));
                        vpin_tablewidget_ptr.set_item(cnt, 3, vpin_table_widget_item.into_ptr());
                        // PLATFORM
                        let mut vpin_table_widget_item = QTableWidgetItem::new();
                        vpin_table_widget_item.set_text(&QString::from_std_str(
                            result.coords.platform.to_string().as_str(),
                        ));
                        vpin_tablewidget_ptr.set_item(cnt, 4, vpin_table_widget_item.into_ptr());
                        // SITE
                        let mut vpin_table_widget_item = QTableWidgetItem::new();
                        vpin_table_widget_item.set_text(&QString::from_std_str(
                            result.coords.site.to_string().as_str(),
                        ));
                        vpin_tablewidget_ptr.set_item(cnt, 5, vpin_table_widget_item.into_ptr());
                        // WITHS
                        let mut vpin_table_widget_item = QTableWidgetItem::new();
                        vpin_table_widget_item.set_text(&QString::from_std_str(
                            result.withs.map_or(0, |x| x.len()).to_string().as_str(),
                        ));
                        vpin_tablewidget_ptr.set_item(cnt, 6, vpin_table_widget_item.into_ptr());
                        cnt += 1;
                    }
                    vpin_tablewidget_ptr.set_sorting_enabled(true);
                }),
                _db: db,
                _widget: root_widget,
                _vpin_table: vpin_tablewidget_ptr,
                _query_button: button_ptr,
                _pkg_line_edit: line_edit_ptr,
            };
            button_ptr.clicked().connect(&form.button_clicked);
            //line_edit.text_edited().connect(&form.line_edit_edited);
            vpin_tablewidget_ptr
                .cell_double_clicked()
                .connect(&form.row_double_clicked);
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
        let _form = Form::new(&mut vpin_finder);
        QApplication::exec()
    });
}
