use pbgui_withs::{
    utility::{create_vlayout, qs},
    WithsList, WithsListConfig,
};
use qt_core::{QResource, Slot};
use qt_widgets::{QApplication, QPushButton, QWidget};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    QApplication::init(|_app| unsafe {
        let _result = QResource::register_resource_q_string(&qs("/Users/jgerber/bin/withlist.rcc"));
        let mut main = QWidget::new_0a();
        let mut main_ref = main.as_mut_ptr();
        let main_layout = create_vlayout();
        main.set_layout(main_layout.into_ptr());

        let withs_list = Rc::new(RefCell::new(WithsList::new(
            main_ref,
            WithsListConfig::default(),
        )));

        let wl_c3 = withs_list.clone();
        let wl_c4 = withs_list.clone();

        withs_list
            .borrow_mut()
            .set_stylesheet("/Users/jgerber/bin/withlist.qss");

        withs_list.borrow_mut().set_cb_items(vec![
            "amtools",
            "animcomp",
            "animpublish",
            "animrender",
            "assetbrowser",
            "assetmanager",
            "atomic",
            "autorender",
            "dd",
            "ddg",
            "deferredpipeline",
            "gcc",
            "houdini",
            "houdinipipeline",
            "houdinisubmission",
            "jsconfig",
            "jstools",
            "jsutils",
            "layoutpipelne",
            "lightpipeline",
            "make",
            "mari",
            "maya",
            "modelpipeline",
            "modelpublish",
            "mudbox",
            "nuke",
            "nukesubmission",
            "organic",
            "packaboo",
            "packaboo_utils",
            "packrat",
            "pk",
            "pbutils",
            "prez",
            "qt",
            "qtpy",
            "race",
            "racetrack",
            "raceview",
            "redshift",
            "rigtools",
            "samson",
            "shotgun",
            "shotgunapi",
            "submission",
            "texturepublish",
            "texturepipeline",
            "vray",
            "vrayddbase",
            "vray_for_maya",
            "wam",
            "wambase",
            "xerces",
        ]);
        // find_shortcut.activated().connect(&find_slot);
        // add_shortcut.activated().connect(&add_slot);

        withs_list.borrow_mut().set_add_mode();
        withs_list.borrow_mut().set_cb_max_visible_items(50);
        let mut print_button = QPushButton::from_q_string(&qs("pushme"));
        let bp = print_button.as_mut_ref();
        main_ref.layout().add_widget(print_button.into_ptr());

        let print_slot: Slot<'static> = Slot::new(move || {
            for x in wl_c3.borrow().items() {
                println!("{}", x);
            }
        });
        bp.pressed().connect(&print_slot);

        let mut clear_button = QPushButton::from_q_string(&qs("Clear"));
        let cb = clear_button.as_mut_ref();
        main_ref.layout().add_widget(clear_button.into_ptr());

        let clear_slot: Slot<'static> = Slot::new(move || {
            wl_c4.borrow_mut().clear();
        });
        cb.pressed().connect(&clear_slot);
        main_ref.show();

        QApplication::exec()
    });
}
