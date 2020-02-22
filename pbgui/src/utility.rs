//! Provides free function utilities used throughout the application
use log;
use qt_core::{
    q_io_device::OpenModeFlag, QFile, QFlags, QModelIndex, QSize, QString, QTextStream, QVariant,
};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref},
    QDesktopWidget, QHBoxLayout, QMainWindow, QTableWidget, QTableWidgetItem, QVBoxLayout,
};

/// Given an input of &str or String, return a boxed QString
///
/// # Arguments
///
/// * `input` - type which implements AsRef<str>
///
/// # Returns
///
/// * Owned QStrin
pub fn qs<S: AsRef<str>>(input: S) -> CppBox<QString> {
    QString::from_std_str(input.as_ref())
}

//"/Users/jgerber/bin/pbgui.qss"
/// load the stylesheet given a reference to the full path to the stylesheet and a MutPtr to
/// the QMainWindow, to which the stylesheet will be applied
///
/// # Arguments
///
/// * `sheet` - The path to the stylesheet
/// * `widget` - A MutPtr to the QMainWindow
///
/// # Returns
///
/// * None
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

/// Updates cell in a QTableWidget givn a row and column
///
/// # Arguments
///
/// * `value` - A type which implements std::fmt::Display, the value will replace the existing row value
/// * `table` - A mutable reference to a MutPtr to the QTableWidget to be updated
/// * `row` - The row to update provided as an i32
/// * `column` - The column to update given as an i32
///
/// # Returns
///
/// * None
pub fn update_text_row<T: std::fmt::Display>(
    value: &T,
    table: &mut MutPtr<QTableWidget>,
    row: i32,
    column: i32,
) {
    unsafe {
        let mut changes_table_item = QTableWidgetItem::new();
        changes_table_item.set_text(&QString::from_std_str(value.to_string().as_str()));
        table.set_item(row, column, changes_table_item.into_ptr());
    }
}

/// Update a QTableWdiget cell with a RowType instance, given a row and column
///
/// # Arguments
///
/// * `value` - an instance of the RowType enum
/// * `table` - mutable reference to a MutPtr to a QTableWidget to update
/// * `row` - The row of the cell to update
/// * `column` The column of the cell to update
///
/// # Returns
///
/// * None
pub fn update_row(value: RowType, table: &mut MutPtr<QTableWidget>, row: i32, column: i32) {
    unsafe {
        let mut changes_table_item = QTableWidgetItem::new();
        match value {
            RowType::Str(s) => {
                changes_table_item.set_text(&QString::from_std_str(s));
                table.set_item(row, column, changes_table_item.into_ptr());
            }
            RowType::Int(i) => {
                let variant = QVariant::from_int(i);
                changes_table_item.set_data(
                    2, // EditRole
                    variant.as_ref(),
                );
                table.set_item(row, column, changes_table_item.into_ptr());
            }
        }
    }
}

/// Enum defining the type of the row, as either a Str(&str) or Int(i32). The RowType is
/// non-owning.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RowType<'a> {
    Str(&'a str),
    Int(i32),
}

/// Resize the window to some scale of the current screen.
///
/// # Arguments
///
/// * `main_window`: The main window of the gui application
/// * `scale`: A scale factor applied to the full size of the main screen in
///            order to arrive at the requested size
///
///
/// # Returns
///
/// * None
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

/// Create a QVBoxLayout that whose contents margins and spacing have been zero'ed out
pub fn create_vlayout() -> CppBox<QVBoxLayout> {
    unsafe {
        let mut pc_vlayout = QVBoxLayout::new_0a();
        pc_vlayout.set_margin(0);
        pc_vlayout.set_contents_margins_4a(0, 0, 0, 0);
        pc_vlayout.set_spacing(0);
        pc_vlayout
    }
}

/// Create a QHBoxLayout that whose contents margins and spacing have been zero'ed out
pub fn create_hlayout() -> CppBox<QHBoxLayout> {
    unsafe {
        let mut pc_hlayout = QHBoxLayout::new_0a();
        pc_hlayout.set_margin(0);
        pc_hlayout.set_contents_margins_4a(0, 0, 0, 0);
        pc_hlayout.set_spacing(0);
        pc_hlayout
    }
}

/// Given a QModelIndex retrieved from the tree, return the distribution name
///
/// # Arguments
///
/// * `idx` - A Ref to a QModelIndex identifiying the distribution in the QTreeView
///
/// # Returns
///
/// * Option wrapped distribution name String
pub unsafe fn distribution_from_idx(idx: Ref<QModelIndex>) -> Option<String> {
    if !idx.is_valid() {
        log::warn!("distribution_from_idx supplied QModelIndex not valid.");
        return None;
    }
    let parent = idx.parent();
    if !parent.is_valid() {
        //we clicked on the distribution. Our parent is the root
        return None;
    }
    let gp = parent.parent();
    if gp.is_valid() {
        // we are too deep. Our grandparent should have been None
        return None;
    }
    // get package name
    let package = if parent.column() == 0 {
        parent.data_0a().to_string().to_std_string()
    } else {
        parent
            .sibling_at_column(0)
            .data_0a()
            .to_string()
            .to_std_string()
    };

    let version = if idx.column() == 0 {
        idx.data_0a().to_string().to_std_string()
    } else {
        idx.sibling_at_column(0)
            .data_0a()
            .to_string()
            .to_std_string()
    };
    let dist = format!("{}-{}", package, version);
    log::debug!("found dist: {}", dist);
    // only the distribution is allowed to have a dash in its name
    Some(dist)
}
