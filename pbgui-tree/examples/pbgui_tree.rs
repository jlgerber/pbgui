use packybara::packrat::PackratDb;
use packybara::packrat::{Client, NoTls};
use packybara::traits::*;
use pbgui_tree::tree;
use qt_core::QResource;
use qt_widgets::{QApplication, QFrame, QMainWindow};
use rustqt_utils::{create_vlayout, qs};

pub struct ClientProxy {}

impl ClientProxy {
    pub fn connect() -> Result<Client, Box<dyn std::error::Error>> {
        let client = Client::connect(
            "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
            NoTls,
        )?;
        Ok(client)
    }
}

fn main() {
    QApplication::init(|_app| unsafe {
        let _result = QResource::register_resource_q_string(&qs(
            "/Users/jgerber/src/rust/pbgui-tree/resources/pbgui_tree.rcc",
        ));
        let mut main_window = QMainWindow::new_0a();
        let mut main_widget = QFrame::new_0a();
        let main_widget_ptr = main_widget.as_mut_ptr();

        // main_layout
        let main_layout = create_vlayout();
        //let  main_layout_ptr = main_layout.as_mut_ptr();
        main_widget.set_layout(main_layout.into_ptr());
        // set main_widget as the central widget in main_window
        main_window.set_central_widget(main_widget.into_ptr());

        let mytree = tree::DistributionTreeView::create(main_widget_ptr);

        mytree.set_default_stylesheet();
        mytree.set_packages(vec!["foo", "bar", "bla"]);

        mytree.clear_packages();
        let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
        let mut db = PackratDb::new(client);
        let results = get_all_packages(&mut db);
        let results = results.iter().map(|s| s.name.as_str()).collect::<Vec<_>>();
        let sites = get_all_sites(&mut db);
        let sites = sites.iter().map(|s| s.name.as_str()).collect::<Vec<_>>();

        mytree.set_packages(results);
        mytree.set_sites(sites, "portland");
        main_window.show();
        QApplication::exec()
    });
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
