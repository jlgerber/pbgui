use qt_core::{Orientation, QListOfInt};
use qt_widgets::{
    cpp_core::{MutPtr, Ref},
    QSplitter, QVBoxLayout,
};
/// Create VersionPin Table Splitter
/// This splitter separates the VersionPin table, from the
/// stackwidget below, owning both of them.
///
/// # Arguments
/// * `center_layout_ptr` - the center layout, which will be given ownership
/// of the splitter
///
/// # Returns
/// * A pointer to the splitter
pub fn create(center_layout_ptr: &mut MutPtr<QVBoxLayout>) -> MutPtr<QSplitter> {
    unsafe {
        // Create a horizontally running Splitter (the splitter divides
        // the widget horizontally. Qt refers to this as vertical
        // orientation. I find it confusing.)
        let mut vsplit = QSplitter::new();
        let vsplit_ptr = vsplit.as_mut_ptr();
        vsplit.set_orientation(Orientation::Vertical);
        // set up splitter sizing
        let mut splitter_sizes = QListOfInt::new();
        splitter_sizes.append_int(Ref::from_raw_ref(&(500 as i32)));
        splitter_sizes.append_int(Ref::from_raw_ref(&(300 as i32)));
        vsplit.set_sizes(&splitter_sizes);
        center_layout_ptr.add_widget(vsplit.into_ptr());
        vsplit_ptr
    }
}
