use qt_core::QString;
use qt_widgets::{cpp_core::MutPtr, QTableWidget, QTableWidgetItem};

/// Utility function to help set up table headers. The user passes in the table pointer,
/// as well as a slice of 3-tuples, whose components are as follows:
/// 1. The index of the header in the ui
/// 2. The label of the header, as a static str
/// 3. A bool indicating whether the header is visible or not.
///
/// # Arguments
/// * `tablewidget_ptr` - The table we are interested in setting up with headers
/// * `headers` - A slice of tuples, as described above
///
/// # Returns
/// * None
pub fn setup(tablewidget_ptr: &mut MutPtr<QTableWidget>, headers: &[(i32, &'static str, bool)]) {
    unsafe {
        for (idx, val, hidden) in headers.into_iter() {
            if !hidden {
                let vpin_table_widget_item =
                    QTableWidgetItem::from_q_string(&QString::from_std_str(val));
                tablewidget_ptr.set_horizontal_header_item(*idx, vpin_table_widget_item.into_ptr());
            } else {
                tablewidget_ptr.set_column_hidden(*idx, true);
            }
        }
    }
}
