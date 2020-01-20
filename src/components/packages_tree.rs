use crate::utility::create_vlayout;
use pbgui_tree::tree;
//use qt_widgets::q_abstract_item_view::DragDropMode;
use crate::ClientProxy;
use packybara::packrat::PackratDb;
use packybara::traits::*;
use qt_widgets::{cpp_core::MutPtr, QFrame, QSplitter};
use std::cell::RefCell;
use std::rc::Rc;

pub fn create<'c>(mut splitter: MutPtr<QSplitter>) -> Rc<RefCell<tree::DistributionTreeView<'c>>> {
    unsafe {
        let mut frame = QFrame::new_0a();
        let layout = create_vlayout();
        frame.set_layout(layout.into_ptr());
        let dtv = tree::DistributionTreeView::create(frame.as_mut_ptr());
        splitter.add_widget(frame.into_ptr());
        println!("here we fgo");
        let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
        let mut db = PackratDb::new(client);
        let results = get_all_packages(&mut db);
        let results = results.iter().map(|s| s.name.as_str()).collect::<Vec<_>>();
        let sites = get_all_sites(&mut db);
        let sites = sites.iter().map(|s| s.name.as_str()).collect::<Vec<_>>();
        dtv.set_packages(results);
        dtv.set_sites(sites, "portland");
        dtv.set_default_stylesheet();

        Rc::new(RefCell::new(dtv))
    }
}

fn get_all_packages(
    db: &mut PackratDb,
) -> std::vec::Vec<packybara::db::find_all::packages::FindAllPackagesRow> {
    db.find_all_packages()
        .query()
        .expect("unable to find_all_packages")
}

fn get_all_sites(
    db: &mut PackratDb,
) -> std::vec::Vec<packybara::db::find_all::sites::FindAllSitesRow> {
    db.find_all_sites()
        .query()
        .expect("unable to find_all_sites")
}
