use crate::utility::qs;
use qt_widgets::{cpp_core::MutPtr, QSplitter, QVBoxLayout, QWidget};

/// Create and configure the center widget,
pub fn create(with_splitter_ptr: &mut MutPtr<QSplitter>) -> MutPtr<QVBoxLayout> {
    unsafe {
        // The center widget is the most prominent of the of the
        // two widgets contained in the splitter. The other -
        // the withpackages widget -- is intended to sit off to the
        // right and take up relatively little space, when displayed.
        let mut center_widget = QWidget::new_0a();
        center_widget.set_object_name(&qs("CenterWidget"));
        let mut center_layout = QVBoxLayout::new_0a();
        let center_layout_ptr = center_layout.as_mut_ptr();
        center_widget.set_layout(center_layout.into_ptr());
        // add center widget into splitter
        with_splitter_ptr.add_widget(center_widget.into_ptr());
        center_layout_ptr
    }
}
