use qt_core::{Orientation, QListOfInt};
use qt_widgets::{
    cpp_core::{MutPtr, Ref},
    QSplitter, QVBoxLayout,
};

pub fn create(layout: &mut MutPtr<QVBoxLayout>) -> MutPtr<QSplitter> {
    unsafe {
        let mut with_splitter = QSplitter::new();
        let with_splitter_ptr = with_splitter.as_mut_ptr();
        with_splitter.set_orientation(Orientation::Horizontal);
        layout.add_widget(with_splitter.into_ptr());
        with_splitter_ptr
    }
}

pub fn set_sizes(splitter: &mut MutPtr<QSplitter>) {
    unsafe {
        let mut splitter_sizes = QListOfInt::new();
        splitter_sizes.append_int(Ref::from_raw_ref(&(200 as i32)));
        splitter_sizes.append_int(Ref::from_raw_ref(&(1000 as i32)));
        splitter_sizes.append_int(Ref::from_raw_ref(&(200 as i32)));
        splitter.set_sizes(&splitter_sizes);
    }
}
