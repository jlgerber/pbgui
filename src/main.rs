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
    button_clicked: Slot<'a>,
}

impl<'a> Form<'a> {
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
        // fn setup<T>(results: T, mut combobox: CppBox<QComboBox>, item: &str, lqbel:&str) -> CppBox<QComboBox> {

        // }
        //results
        let mut level_combobox = QComboBox::new_0a();
        let level_cb_ptr = level_combobox.as_mut_ptr();
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

        // Roles
        let mut role_combobox = QComboBox::new_0a();
        let role_cb_ptr = role_combobox.as_mut_ptr();
        let results = db.find_all_roles().query().unwrap();
        role_combobox.add_item_q_string(&QString::from_std_str("any"));
        for r in results {
            let role_str = r.role.as_str();
            if role_str != "any" {
                role_combobox.add_item_q_string(&QString::from_std_str(role_str));
            }
        }
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
        // Platforms
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

        // Site
        let mut site_combobox = QComboBox::new_0a();
        let mut site_cb_ptr = site_combobox.as_mut_ptr();
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
        // Direction
        let mut dir_combobox = QComboBox::new_0a();
        let dir_cb_ptr = dir_combobox.as_mut_ptr();
        site_cb_ptr.add_item_q_string(&QString::from_std_str("any"));
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

        let qspacer = QSpacerItem::new_3a(30, 10, Policy::Expanding);
        layout.add_item(qspacer.into_ptr());
        (
            level_cb_ptr,
            role_cb_ptr,
            platform_cb_ptr,
            site_cb_ptr,
            dir_cb_ptr,
        )
    }

    unsafe fn setup_headers(
        mut vpin_tablewidget: MutPtr<QTableWidget>,
        headers: Vec<&'static str>,
    ) {
        for (cnt, val) in headers.into_iter().enumerate() {
            let vpin_table_widget_item =
                QTableWidgetItem::from_q_string(&QString::from_std_str(val));
            vpin_tablewidget
                .set_horizontal_header_item(cnt as i32, vpin_table_widget_item.into_ptr());
        }
    }

    fn new(mut db: &'a mut PackratDb) -> Form<'a> {
        unsafe {
            // parent root_widget
            let mut root_widget = QWidget::new_0a();
            // top vertical layout
            let mut root_layout = QVBoxLayout::new_0a();
            let mut root_layout_ptr = root_layout.as_mut_ptr();
            root_widget.set_layout(root_layout.into_ptr());
            let mut hlayout = QHBoxLayout::new_0a();
            let mut hlayout_ptr = hlayout.as_mut_ptr();
            root_layout_ptr.add_layout_1a(hlayout.into_ptr());
            let (level_ptr, role_ptr, platform_ptr, site_ptr, dir_ptr) =
                Self::combo_boxes(&mut db, &mut hlayout_ptr);
            // LINE EDIT
            let mut line_edit = QLineEdit::new();
            let line_edit_ptr = line_edit.as_mut_ptr();
            root_layout_ptr.add_widget(line_edit.into_ptr());

            let mut vpin_tablewidget = QTableWidget::new_2a(0, COLUMNS);
            let vpin_tw_ptr = vpin_tablewidget.as_mut_ptr();
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
            let mut vpin_tablewidget_ptr = vpin_tw_ptr.clone();
            Self::setup_headers(
                vpin_tw_ptr,
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
            let button_ptr = button.as_mut_ptr();
            button.set_minimum_width(70);
            button.set_maximum_width(70);
            hlayout_ptr.add_widget(button.into_ptr());
            root_layout_ptr.add_widget(vpin_tablewidget.into_ptr());
            root_widget.show();

            let form = Form {
                button_clicked: Slot::new(move || {
                    let dirtxt = dir_ptr.current_text();
                    let line_edit_txt = line_edit_ptr.text().to_std_string();

                    let showtxt = level_ptr.current_text();
                    // let mut vpin_table_widget_item = QTableWidgetItem::new();
                    // vpin_table_widget_item.set_text(&showtxt);
                    let showtxt = showtxt.to_std_string();
                    // vpin_tablewidget_ptr.set_item(0, 1, vpin_table_widget_item.into_ptr());

                    let roletxt = role_ptr.current_text();
                    // let mut vpin_table_widget_item = QTableWidgetItem::new();
                    // vpin_table_widget_item.set_text(&roletxt);
                    let roletxt = roletxt.to_std_string();
                    // vpin_tablewidget_ptr.set_item(0, 2, vpin_table_widget_item.into_ptr());

                    let platformtxt = platform_ptr.current_text();
                    // let mut vpin_table_widget_item = QTableWidgetItem::new();
                    // vpin_table_widget_item.set_text(&platformtxt);
                    let platformtxt = platformtxt.to_std_string();
                    // vpin_tablewidget_ptr.set_item(0, 3, vpin_table_widget_item.into_ptr());

                    let sitetxt = site_ptr.current_text();
                    // let mut vpin_table_widget_item = QTableWidgetItem::new();
                    // vpin_table_widget_item.set_text(&sitetxt);
                    let sitetxt = sitetxt.to_std_string();
                    // vpin_tablewidget_ptr.set_item(0, 4, vpin_table_widget_item.into_ptr());
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
                _widget: root_widget,
                _vpin_table: vpin_tablewidget_ptr,
                _query_button: button_ptr,
                _pkg_line_edit: line_edit_ptr,
            };
            button_ptr.clicked().connect(&form.button_clicked);
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
