use qt_core::QString;
use qt_widgets::{cpp_core::MutPtr, QTableWidget, QTableWidgetItem};

//------------------------------//
// setup the headers matching   //
// the provided header vector   //
//------------------------------//
pub fn setup_table_headers(
    vpin_tablewidget: &mut MutPtr<QTableWidget>,
    headers: &[(i32, &'static str, bool)],
) {
    unsafe {
        for (idx, val, hidden) in headers.into_iter() {
            if !hidden {
                let vpin_table_widget_item =
                    QTableWidgetItem::from_q_string(&QString::from_std_str(val));
                vpin_tablewidget
                    .set_horizontal_header_item(*idx, vpin_table_widget_item.into_ptr());
            } else {
                vpin_tablewidget.set_column_hidden(*idx, true);
            }
        }
    }
}
