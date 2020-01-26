use packybara::db::traits::PBFind;
use packybara::packrat::PackratDb;
use packybara::packrat::{Client, NoTls};
use pbgui_toolbar::toolbar;
use qt_core::QResource;
use qt_widgets::{QApplication, QMainWindow, QWidget};
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
        let _result = QResource::register_resource_q_string(&qs("/Users/jgerber/bin/pbgui.rcc"));
        let mut main_window = QMainWindow::new_0a();
        let mut main_widget = QWidget::new_0a();
        let _main_widget_ptr = main_widget.as_mut_ptr();

        // main_layout
        let mut main_layout = create_vlayout();
        let _main_layout_ptr = main_layout.as_mut_ptr();
        main_widget.set_layout(main_layout.into_ptr());

        // set main_widget as the central widget in main_window
        main_window.set_central_widget(main_widget.into_ptr());
        let tb = toolbar::create(main_window.as_mut_ptr());
        tb.set_default_stylesheet();
        let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
        let mut db = PackratDb::new(client);

        let results = db
            .find_all_levels()
            .query()
            .expect("unable to find_all_levels");
        let results = results.iter().map(|s| s.level.as_str()).collect::<Vec<_>>();
        tb.set_level_items(results);

        let results = db
            .find_all_roles()
            .query()
            .expect("unable to find_all_roless");
        let results = results.iter().map(|s| s.role.as_str()).collect::<Vec<_>>();
        tb.set_role_items(results);

        let results = db
            .find_all_platforms()
            .query()
            .expect("unable to find_all_platforms");
        let results = results.iter().map(|s| s.name.as_str()).collect::<Vec<_>>();
        tb.set_platform_items(results);

        let results = db
            .find_all_sites()
            .query()
            .expect("unable to find_all_platforms");
        let results = results.iter().map(|s| s.name.as_str()).collect::<Vec<_>>();
        tb.set_site_items(results);

        main_window.show();
        QApplication::exec()
    });
}
