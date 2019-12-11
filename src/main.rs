#![windows_subsystem = "windows"]
use packybara::packrat::{Client, NoTls, PackratDb};
use packybara::LtreeSearchMode;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    q_header_view::ResizeMode,
    qt_core::QString,
    qt_core::Slot,
    QApplication, QComboBox, QGroupBox, QHBoxLayout, QLineEdit, QPushButton, QTableWidget,
    QTableWidgetItem, QVBoxLayout, QWidget,
};
use std::str::FromStr;

struct Form<'a> {
    _db: &'a mut PackratDb,
    _widget: CppBox<QWidget>,
    _button: MutPtr<QPushButton>,
    _line_edit: MutPtr<QLineEdit>,
    _table_widget: MutPtr<QTableWidget>,
    _grpbox: MutPtr<QGroupBox>,
    _twh: Vec<MutPtr<QTableWidgetItem>>,
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
    ) {
        //results
        let mut lcb = QComboBox::new_0a();
        let results = db.find_all_levels().query().unwrap();
        for r in results {
            let level_str = r.level.as_str();
            lcb.add_item_q_string(&QString::from_std_str(level_str));
        }
        layout.add_widget(&mut lcb);

        let mut rcb = QComboBox::new_0a();
        let results = db.find_all_roles().query().unwrap();
        rcb.add_item_q_string(&QString::from_std_str("any"));
        for r in results {
            let role_str = r.role.as_str();
            rcb.add_item_q_string(&QString::from_std_str(role_str));
        }
        layout.add_widget(&mut rcb);
        let mut pcb = QComboBox::new_0a();

        let results = db.find_all_platforms().query().unwrap();
        for r in results {
            let platform_str = r.name.as_str();
            pcb.add_item_q_string(&QString::from_std_str(platform_str));
        }
        layout.add_widget(&mut pcb);
        let mut scb = QComboBox::new_0a();
        let results = db.find_all_sites().query().unwrap();
        scb.add_item_q_string(&QString::from_std_str("any"));
        for r in results {
            let site_str = r.name.as_str();
            scb.add_item_q_string(&QString::from_std_str(site_str));
        }
        layout.add_widget(&mut scb);

        let mut direction = QComboBox::new_0a();
        //let results = db.find_all_sites().query().unwrap();
        scb.add_item_q_string(&QString::from_std_str("any"));
        for r in &["ancestor", "exact", "descendant"] {
            direction.add_item_q_string(&QString::from_std_str(r));
        }
        layout.add_widget(&mut direction);
        (lcb, rcb, pcb, scb, direction)
    }

    unsafe fn setup_headers(
        mut tw: MutPtr<QTableWidget>,
        headers: Vec<&'static str>,
    ) -> (MutPtr<QTableWidget>, Vec<MutPtr<QTableWidgetItem>>) {
        let mut qtwi_vec = Vec::new();
        for (cnt, val) in headers.into_iter().enumerate() {
            let mut qtwi = QTableWidgetItem::from_q_string(&QString::from_std_str(val));
            tw.set_horizontal_header_item(cnt as i32, &mut qtwi);
            qtwi_vec.push(qtwi.into_ptr());
        }
        (tw, qtwi_vec)
    }

    fn new(mut db: &'a mut PackratDb) -> Form<'a> {
        unsafe {
            // parent widget
            let mut widget = QWidget::new_0a();
            // top vertical layout
            let mut layout = QVBoxLayout::new_1a(&mut widget).into_ptr();
            // groupbox for comboboxes
            let mut grpbox = QGroupBox::new();
            layout.add_widget(&mut grpbox);
            let mut grpbox = grpbox.into_ptr();
            // layout for combo boxes
            let mut hlayout = QHBoxLayout::new_0a().into_ptr();
            let (level, role, platform, site, direction) =
                Self::combo_boxes(&mut db, hlayout.clone());
            let level_ptr = level.into_ptr();
            let role_ptr = role.into_ptr();
            let platform_ptr = platform.into_ptr();
            let site_ptr = site.into_ptr();
            let dir_ptr = direction.into_ptr();

            grpbox.set_layout(hlayout);
            let mut line_edit = QLineEdit::new();

            layout.add_widget(&mut line_edit);
            let line_edit = line_edit.into_ptr();
            let mut tw = QTableWidget::new_2a(5, 5);
            tw.vertical_header().hide();
            tw.horizontal_header().set_stretch_last_section(true);
            tw.horizontal_header()
                .set_section_resize_mode_1a(ResizeMode::Stretch);
            tw.set_show_grid(false);
            tw.set_alternating_row_colors(true);
            tw.set_style_sheet(&QString::from_std_str(
                "alternate-background-color: rgb(70,70,70);color: rgb(200,200,200);background-color: rgb(60,60,60);",
            ));
            tw.horizontal_header()
                .set_style_sheet(&QString::from_std_str(
                    "background-color: rgb(80,80,80); color:white; border:none;outline:none;border-left: 0px; border-right: 0px;",
                ));

            let (tw, qtwi_vec) = Self::setup_headers(
                tw.into_ptr(),
                vec!["Distribution", "Level", "Role", "Platform", "Site"],
            );
            let mut button = QPushButton::from_q_string(&QString::from_std_str("Query"));

            hlayout.add_widget(&mut button);
            let button = button.into_ptr();

            layout.add_widget(tw);
            let mut tw_ptr = tw.clone();
            // let mut button = QPushButton::from_q_string(&QString::from_std_str("Query"));

            // layout.add_widget(&mut button);
            // let button = button.into_ptr();

            widget.show();

            let form = Form {
                button_clicked: Slot::new(move || {
                    let dirtxt = dir_ptr.current_text();

                    let showtxt = level_ptr.current_text();
                    let mut qtwi = QTableWidgetItem::new();
                    qtwi.set_text(&showtxt);
                    tw_ptr.set_item(0, 1, qtwi.into_ptr());

                    let roletxt = role_ptr.current_text();
                    let mut qtwi = QTableWidgetItem::new();
                    qtwi.set_text(&roletxt);
                    tw_ptr.set_item(0, 2, qtwi.into_ptr());

                    let platformtxt = platform_ptr.current_text();
                    let mut qtwi = QTableWidgetItem::new();
                    qtwi.set_text(&platformtxt);
                    tw_ptr.set_item(0, 3, qtwi.into_ptr());

                    let sitetxt = site_ptr.current_text();
                    let mut qtwi = QTableWidgetItem::new();
                    qtwi.set_text(&sitetxt);
                    tw_ptr.set_item(0, 4, qtwi.into_ptr());
                    // for now
                    let mut client = Client::connect(
                        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
                        NoTls,
                    )
                    .unwrap();
                    let mut pb = PackratDb::new(client);
                    let results = pb
                        .find_all_versionpins()
                        .level(showtxt.to_std_string().as_str())
                        .role(roletxt.to_std_string().as_str())
                        .platform(platformtxt.to_std_string().as_str())
                        .site(sitetxt.to_std_string().as_str())
                        .search_mode(
                            LtreeSearchMode::from_str(dirtxt.to_std_string().as_str()).unwrap(),
                        )
                        .query()
                        .unwrap();
                    let mut cnt = 0;
                    tw_ptr.set_row_count(0);
                    tw_ptr.set_row_count(results.len() as i32);
                    for result in results {
                        let mut qtwi = QTableWidgetItem::new();
                        qtwi.set_text(&QString::from_std_str(
                            result.distribution.to_string().as_str(),
                        ));
                        tw_ptr.set_item(cnt, 0, qtwi.into_ptr());

                        let mut qtwi = QTableWidgetItem::new();
                        qtwi.set_text(&QString::from_std_str(
                            result.coords.level.to_string().as_str(),
                        ));
                        tw_ptr.set_item(cnt, 1, qtwi.into_ptr());

                        let mut qtwi = QTableWidgetItem::new();
                        qtwi.set_text(&QString::from_std_str(
                            result.coords.role.to_string().as_str(),
                        ));
                        tw_ptr.set_item(cnt, 2, qtwi.into_ptr());

                        let mut qtwi = QTableWidgetItem::new();
                        qtwi.set_text(&QString::from_std_str(
                            result.coords.platform.to_string().as_str(),
                        ));
                        tw_ptr.set_item(cnt, 3, qtwi.into_ptr());

                        let mut qtwi = QTableWidgetItem::new();
                        qtwi.set_text(&QString::from_std_str(
                            result.coords.site.to_string().as_str(),
                        ));
                        tw_ptr.set_item(cnt, 4, qtwi.into_ptr());

                        cnt += 1;
                    }
                }),
                _db: db,
                _widget: widget,
                _table_widget: tw,
                _twh: qtwi_vec,
                _button: button,
                _line_edit: line_edit,
                _grpbox: grpbox,
                //_boxes: boxes,
            };
            button.clicked().connect(&form.button_clicked);
            //line_edit.text_edited().connect(&form.line_edit_edited);
            form
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;
    let mut pb = PackratDb::new(client);
    QApplication::init(|_| unsafe {
        let _form = Form::new(&mut pb);
        QApplication::exec()
    });
    Ok(())
}
