use crate::utility::{create_vlayout, qs};
use qt_widgets::q_abstract_item_view::DragDropMode;
use qt_widgets::{
    cpp_core::{MutPtr, MutRef}, q_size_policy::Policy, QAction, QFrame, QListView, QSizePolicy, QSplitter,
    QToolBar, QToolButton, QWidget,
};
use std::rc::Rc;
use std::cell::RefCell;
use listitem::ItemList;
use crate::ClientProxy;
use packybara::packrat::PackratDb;

pub struct WithToolbar<'a> {
    pub itemlist: Rc<RefCell<ItemList<'a>>>,
    pub edit: MutPtr<QAction>,
    pub save: MutPtr<QAction>,
}


impl<'a> WithToolbar<'a> {
    /// New up a WIthToolbar instance
    pub fn new( itemlist: Rc<RefCell<ItemList<'a>>>,edit: MutPtr<QAction>, save: MutPtr<QAction>) -> Self {
        Self { itemlist, edit, save }
    }
}

/// create and return the withpackage list widget, given the parent splitter.
///
/// # Arguments
/// * `splitter` - The  splitter which we will give ownership of the widget to.
///
/// # Returns
/// * A pointer to the Withs List Widget
pub fn create<'c>(splitter: MutPtr<QSplitter>) -> Rc<RefCell<WithToolbar<'c>>> {
    unsafe {
        let  itemlist = create_withwidget(splitter);
        let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
        let mut packratdb = PackratDb::new(client);
        let packages = packratdb.find_all_packages().query().expect("unable to find packages");
        let packages = packages.into_iter().map(|x| x.name).collect::<Vec<_>>();
        // add actions
        let edit_action = itemlist.borrow_mut().mode_toolbar.borrow_mut().toolbar.add_action_1a(&qs("Edit"));
        let save_action = itemlist.borrow_mut().mode_toolbar.borrow_mut().toolbar.add_action_1a(&qs("Save"));
        itemlist.borrow_mut().set_cb_items(packages);
       Rc::new(RefCell::new(WithToolbar::new(itemlist,edit_action, save_action)))
    }
}

unsafe fn create_withwidget<'z>(
    mut splitter: MutPtr<QSplitter>
) ->Rc<RefCell<ItemList<'z>>> {
    // create the top frame
    let mut frame = QFrame::new_0a();
    let mut frame_ptr = frame.as_mut_ptr();
    let mut layout = create_vlayout();

    frame.set_layout(layout.into_ptr());
    splitter.add_widget(frame.into_ptr());

    let mut parent_w: MutPtr<QWidget> = frame_ptr.static_upcast_mut();
    let itemlist = Rc::new(RefCell::new(ItemList::new(&mut parent_w)));

    itemlist
}