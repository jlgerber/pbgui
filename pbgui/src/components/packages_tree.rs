use super::dist_tree::tree;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use crate::utility::create_vlayout;
use qt_widgets::{cpp_core::MutPtr, QFrame, QSplitter};
use std::rc::Rc;

pub fn create<'c>(
    mut splitter: MutPtr<QSplitter>,
    to_thread_sender: Sender<OMsg>,
    //) -> Rc<RefCell<tree::DistributionTreeView<'c>>> {
) -> Rc<tree::DistributionTreeView<'c>> {
    unsafe {
        let mut frame = QFrame::new_0a();
        let layout = create_vlayout();
        frame.set_layout(layout.into_ptr());

        let distribution_tree_view =
            tree::DistributionTreeView::create(frame.as_mut_ptr(), to_thread_sender);
        splitter.add_widget(frame.into_ptr());

        distribution_tree_view.set_default_stylesheet();

        //Rc::new(RefCell::new(distribution_tree_view))
        Rc::new(distribution_tree_view)
    }
}
