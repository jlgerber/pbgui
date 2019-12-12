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
    qt_core::Slot,
    QApplication, QComboBox, QGroupBox, QHBoxLayout, QLineEdit, QPushButton, QSpacerItem,
    QTableWidget, QTableWidgetItem, QVBoxLayout, QWidget,
};

use std::str::FromStr;
const COLUMNS: i32 = 7;
struct Form<'a> {
    _db: &'a mut PackratDb,
    _widget: CppBox<QWidget>,
    _query_button: MutPtr<QPushButton>,
    _pkg_line_edit: MutPtr<QLineEdit>,
    _vpin_table: MutPtr<QTableWidget>,
    _grpbox: Vec<CppBox<QGroupBox>>,
    _pkg_header_items: Vec<MutPtr<QTableWidgetItem>>,
    _qspacer: MutPtr<QSpacerItem>,
    button_clicked: Slot<'a>,
}

impl<'a> Form<'a> {
    unsafe fn combo_boxes<'b>(
        db: &'b mut PackratDb,
        mut layout: MutPtr<QHBoxLayout>,
    ) -> (
        CppBox<QComboBox>,
        CppBox<QComboBox>,
        CppBox<QComboBox>,
        CppBox<QComboBox>,
        CppBox<QComboBox>,
        CppBox<QSpacerItem>,
        Vec<CppBox<QGroupBox>>,
    ) {
        // fn setup<T>(results: T, mut combobox: CppBox<QComboBox>, item: &str, lqbel:&str) -> CppBox<QComboBox> {

        // }
        //results
        let mut grpboxes = Vec::new();
        let mut level_combobox = QComboBox::new_0a();
        // LEVELs
        let results = db.find_all_levels().query().unwrap();
        level_combobox.add_item_q_string(&QString::from_std_str("facility"));
        for r in results {
            let level_str = r.level.as_str();
            if level_str != "facility" {
                level_combobox.add_item_q_string(&QString::from_std_str(level_str));
            }
        }
        let mut grpbox = QGroupBox::new();
        //grpbox.set_parent(layout); //does not work
        let mut hlayout = QHBoxLayout::new_1a(&mut grpbox).into_ptr();
        hlayout.add_widget_3a(
            &mut level_combobox,
            1,
            QFlags::from(AlignmentFlag::AlignBottom),
        );
        grpbox.set_title(&QString::from_std_str("Show"));
        layout.add_widget(&mut grpbox);
        grpboxes.push(grpbox);
        // Roles
        let mut role_combobox = QComboBox::new_0a();
        let results = db.find_all_roles().query().unwrap();
        role_combobox.add_item_q_string(&QString::from_std_str("any"));
        for r in results {
            let role_str = r.role.as_str();
            if role_str != "any" {
                role_combobox.add_item_q_string(&QString::from_std_str(role_str));
            }
        }
        let mut grpbox = QGroupBox::new();
        let mut hlayout = QHBoxLayout::new_1a(&mut grpbox).into_ptr();
        hlayout.add_widget_3a(
            &mut role_combobox,
            1,
            QFlags::from(AlignmentFlag::AlignBottom),
        );
        grpbox.set_title(&QString::from_std_str("Role"));
        layout.add_widget(&mut grpbox);
        grpboxes.push(grpbox);
        // Platforms
        let mut platform_combobox = QComboBox::new_0a();
        let results = db.find_all_platforms().query().unwrap();
        for r in results {
            let platform_str = r.name.as_str();
            platform_combobox.add_item_q_string(&QString::from_std_str(platform_str));
        }
        let mut grpbox = QGroupBox::new();
        let mut hlayout = QHBoxLayout::new_1a(&mut grpbox).into_ptr();
        hlayout.add_widget_3a(
            &mut platform_combobox,
            1,
            QFlags::from(AlignmentFlag::AlignBottom),
        );
        grpbox.set_title(&QString::from_std_str("Platform"));
        layout.add_widget(&mut grpbox);
        grpboxes.push(grpbox);
        // Site
        let mut site_combobox = QComboBox::new_0a();
        let results = db.find_all_sites().query().unwrap();
        site_combobox.add_item_q_string(&QString::from_std_str("any"));
        for r in results {
            let site_str = r.name.as_str();
            site_combobox.add_item_q_string(&QString::from_std_str(site_str));
        }
        let mut grpbox = QGroupBox::new();
        let mut hlayout = QHBoxLayout::new_1a(&mut grpbox).into_ptr();
        hlayout.add_widget_3a(
            &mut site_combobox,
            1,
            QFlags::from(AlignmentFlag::AlignBottom),
        );
        grpbox.set_title(&QString::from_std_str("Site"));
        layout.add_widget(&mut grpbox);
        grpboxes.push(grpbox);
        // Direction
        let mut dir_combobox = QComboBox::new_0a();
        site_combobox.add_item_q_string(&QString::from_std_str("any"));
        for r in &["ancestor", "exact", "descendant"] {
            dir_combobox.add_item_q_string(&QString::from_std_str(r));
        }
        let mut grpbox = QGroupBox::new();
        let mut hlayout = QHBoxLayout::new_1a(&mut grpbox).into_ptr();
        hlayout.add_widget_3a(
            &mut dir_combobox,
            1,
            QFlags::from(AlignmentFlag::AlignBottom),
        );
        grpbox.set_title(&QString::from_std_str("Direction"));
        layout.add_widget(&mut grpbox);
        grpboxes.push(grpbox);

        let mut qspacer = QSpacerItem::new_3a(30, 10, Policy::Expanding);
        layout.add_item(&mut qspacer);
        (
            level_combobox,
            role_combobox,
            platform_combobox,
            site_combobox,
            dir_combobox,
            qspacer,
            grpboxes,
        )
    }

    unsafe fn setup_headers(
        mut vpin_tablewidget: MutPtr<QTableWidget>,
        headers: Vec<&'static str>,
    ) -> (MutPtr<QTableWidget>, Vec<MutPtr<QTableWidgetItem>>) {
        let mut pkg_header_items_vec = Vec::new();
        for (cnt, val) in headers.into_iter().enumerate() {
            let mut vpin_table_widget_item =
                QTableWidgetItem::from_q_string(&QString::from_std_str(val));
            vpin_tablewidget.set_horizontal_header_item(cnt as i32, &mut vpin_table_widget_item);
            pkg_header_items_vec.push(vpin_table_widget_item.into_ptr());
        }
        (vpin_tablewidget, pkg_header_items_vec)
    }

    fn new(mut db: &'a mut PackratDb) -> Form<'a> {
        unsafe {
            // parent widget
            let mut widget = QWidget::new_0a();
            // top vertical layout
            let mut layout = QVBoxLayout::new_1a(&mut widget).into_ptr();
            // groupbox for comboboxes
            // layout for combo boxes
            let mut hlayout = QHBoxLayout::new_0a().into_ptr();
            let (level, role, platform, site, dir_combobox, qspacer, grpboxes) =
                Self::combo_boxes(&mut db, hlayout.clone());
            let level_ptr = level.into_ptr();
            let role_ptr = role.into_ptr();
            let platform_ptr = platform.into_ptr();
            let site_ptr = site.into_ptr();
            let dir_ptr = dir_combobox.into_ptr();
            layout.add_layout_1a(hlayout);
            let mut line_edit = QLineEdit::new();

            layout.add_widget(&mut line_edit);
            let line_edit = line_edit.into_ptr();
            let mut vpin_tablewidget = QTableWidget::new_2a(0, COLUMNS);
            vpin_tablewidget.vertical_header().hide();
            vpin_tablewidget.set_selection_behavior(SelectionBehavior::SelectRows);
            vpin_tablewidget.set_edit_triggers(QFlags::from(EditTrigger::NoEditTriggers));
            vpin_tablewidget.set_selection_mode(SelectionMode::SingleSelection);
            vpin_tablewidget
                .horizontal_header()
                .set_stretch_last_section(true);
            vpin_tablewidget
                .horizontal_header()
                .set_section_resize_mode_1a(ResizeMode::Stretch);
            vpin_tablewidget.set_show_grid(false);
            vpin_tablewidget.set_alternating_row_colors(true);
            vpin_tablewidget.set_style_sheet(&QString::from_std_str(
                "alternate-background-color: rgb(50,50,50);color: rgb(200,200,200);background-color: rgb(40,40,40);",
            ));
            vpin_tablewidget.horizontal_header()
                .set_style_sheet(&QString::from_std_str(
                    "background-color: rgb(80,80,80); color:white; border:none;outline:none;border-left: 0px; border-right: 0px;",
                ));

            let (vpin_tablewidget, pkg_header_items_vec) = Self::setup_headers(
                vpin_tablewidget.into_ptr(),
                vec![
                    "Id",
                    "Distribution",
                    "Level",
                    "Role",
                    "Platform",
                    "Site",
                    "Withs",
                ],
            );
            let mut button = QPushButton::from_q_string(&QString::from_std_str("Query"));
            button.set_minimum_width(70);
            button.set_maximum_width(70);
            hlayout.add_widget(&mut button);
            let button = button.into_ptr();

            layout.add_widget(vpin_tablewidget);
            let mut vpin_tablewidget_ptr = vpin_tablewidget.clone();

            widget.show();

            let form = Form {
                button_clicked: Slot::new(move || {
                    let dirtxt = dir_ptr.current_text();
                    let line_edit_txt = line_edit.text().to_std_string();

                    let showtxt = level_ptr.current_text();
                    let mut vpin_table_widget_item = QTableWidgetItem::new();
                    vpin_table_widget_item.set_text(&showtxt);
                    let showtxt = showtxt.to_std_string();
                    vpin_tablewidget_ptr.set_item(0, 1, vpin_table_widget_item.into_ptr());

                    let roletxt = role_ptr.current_text();
                    let mut vpin_table_widget_item = QTableWidgetItem::new();
                    vpin_table_widget_item.set_text(&roletxt);
                    let roletxt = roletxt.to_std_string();
                    vpin_tablewidget_ptr.set_item(0, 2, vpin_table_widget_item.into_ptr());

                    let platformtxt = platform_ptr.current_text();
                    let mut vpin_table_widget_item = QTableWidgetItem::new();
                    vpin_table_widget_item.set_text(&platformtxt);
                    let platformtxt = platformtxt.to_std_string();
                    vpin_tablewidget_ptr.set_item(0, 3, vpin_table_widget_item.into_ptr());

                    let sitetxt = site_ptr.current_text();
                    let mut vpin_table_widget_item = QTableWidgetItem::new();
                    vpin_table_widget_item.set_text(&sitetxt);
                    let sitetxt = sitetxt.to_std_string();
                    vpin_tablewidget_ptr.set_item(0, 4, vpin_table_widget_item.into_ptr());
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
                        .search_mode(
                            LtreeSearchMode::from_str(dirtxt.to_std_string().as_str()).unwrap(),
                        );
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
                _widget: widget,
                _vpin_table: vpin_tablewidget,
                _pkg_header_items: pkg_header_items_vec,
                _query_button: button,
                _pkg_line_edit: line_edit,
                _grpbox: grpboxes,
                _qspacer: qspacer.into_ptr(),
                //_boxes: boxes,
            };
            button.clicked().connect(&form.button_clicked);
            //line_edit.text_edited().connect(&form.line_edit_edited);
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
