use log;
use qt_core::{q_io_device::OpenModeFlag, QFile, QFlags, QSize, QString, QTextStream, QVariant};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QDesktopWidget, QHBoxLayout, QMainWindow, QTableWidget, QTableWidgetItem, QVBoxLayout,
};
/// Given an input of &str or String, return a boxed QString
pub fn qs<S: AsRef<str>>(input: S) -> CppBox<QString> {
    QString::from_std_str(input.as_ref())
}
//"/Users/jgerber/bin/pbgui.qss"
pub fn load_stylesheet(sheet: &str, mut widget: MutPtr<QMainWindow>) {
    unsafe {
        // Does not work
        //QResource::add_search_path(&QString::from_std_str("/Users/jgerber/bin/"));
        //
        // this is now called in main.rs
        // let _result = QResource::register_resource_q_string(&QString::from_std_str(
        //    "/Users/jgerber/bin/pbgui.rcc",
        //));

        let mut file = QFile::from_q_string(&QString::from_std_str(sheet));
        if file.open_1a(QFlags::from(OpenModeFlag::ReadOnly)) {
            let mut text_stream = QTextStream::new();
            text_stream.set_device(file.as_mut_ptr());
            let stylesheet = text_stream.read_all();
            widget.set_style_sheet(stylesheet.as_ref());
        } else {
            log::warn!("stylesheet not found");
        }
    }
}

/// Update a row
pub fn update_text_row<T: std::fmt::Display>(
    value: &T,
    table: &mut MutPtr<QTableWidget>,
    cnt: i32,
    column: i32,
) {
    unsafe {
        let mut changes_table_item = QTableWidgetItem::new();
        changes_table_item.set_text(&QString::from_std_str(value.to_string().as_str()));
        table.set_item(cnt, column, changes_table_item.into_ptr());
    }
}

/// Update a row given a RowType
pub fn update_row(value: RowType, table: &mut MutPtr<QTableWidget>, cnt: i32, column: i32) {
    unsafe {
        let mut changes_table_item = QTableWidgetItem::new();
        match value {
            RowType::Str(s) => {
                changes_table_item.set_text(&QString::from_std_str(s));
                table.set_item(cnt, column, changes_table_item.into_ptr());
            }
            RowType::Int(i) => {
                let variant = QVariant::from_int(i);
                changes_table_item.set_data(
                    2, // EditRole
                    variant.as_ref(),
                );
                table.set_item(cnt, column, changes_table_item.into_ptr());
            }
        }
    }
}

/// Type of row
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RowType<'a> {
    Str(&'a str),
    Int(i32),
}

/// Resize the window to some scale of the current screen.
///
/// # Arguments
/// * `main_window`: The main window of the gui application
/// * `scale`: A scale factor applied to the full size of the main screen in
/// order to arrive at the requested size
pub fn resize_window_to_screen(main_window: &mut MutPtr<QMainWindow>, scale: f32) {
    unsafe {
        let desktop = QDesktopWidget::new();
        let screen_size = desktop.available_geometry();
        let new_size = QSize::new_2a(
            (screen_size.width() as f32 * scale) as i32,
            (screen_size.height() as f32 * scale) as i32,
        );
        main_window.set_geometry_4a(
            ((screen_size.width() - new_size.width()) as f32 / 2.0) as i32,
            ((screen_size.height() - new_size.height()) as f32 / 2.0) as i32,
            new_size.width(),
            new_size.height(),
        );
    }
}

pub fn create_vlayout() -> CppBox<QVBoxLayout> {
    unsafe {
        let mut pc_vlayout = QVBoxLayout::new_0a();
        pc_vlayout.set_margin(0);
        pc_vlayout.set_contents_margins_4a(0, 0, 0, 0);
        pc_vlayout.set_spacing(0);
        pc_vlayout
    }
}

pub fn create_hlayout() -> CppBox<QHBoxLayout> {
    unsafe {
        let mut pc_hlayout = QHBoxLayout::new_0a();
        pc_hlayout.set_margin(0);
        pc_hlayout.set_contents_margins_4a(0, 0, 0, 0);
        pc_hlayout.set_spacing(0);
        pc_hlayout
    }
}
