use crate::utility::create_vlayout;
use pbgui_withs::{WithsList, WithsListConfig};
use qt_widgets::{cpp_core::MutPtr, QFrame, QSplitter, QWidget};
use std::cell::RefCell;
use std::rc::Rc;

/// create and return the WithsList struct, which provides the withs list widget,
/// given the parent splitter.
///
/// # Arguments
/// * `splitter` - The  splitter which we will give ownership of the widget to.
///
/// # Returns
/// * A pointer to the WithsList
pub fn create<'c>(splitter: MutPtr<QSplitter>) -> Rc<RefCell<WithsList<'c>>> {
    unsafe { create_package_withs_list(splitter) }
}

unsafe fn create_package_withs_list<'z>(
    mut splitter: MutPtr<QSplitter>,
) -> Rc<RefCell<WithsList<'z>>> {
    // create the top frame
    let mut frame = QFrame::new_0a();
    let frame_ptr = frame.as_mut_ptr();
    let layout = create_vlayout();

    frame.set_layout(layout.into_ptr());
    splitter.add_widget(frame.into_ptr());

    let parent_w: MutPtr<QWidget> = frame_ptr.static_upcast_mut();

    Rc::new(RefCell::new(WithsList::new(
        parent_w,
        WithsListConfig::default(),
    )))
}
