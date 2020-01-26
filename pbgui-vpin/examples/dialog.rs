use pbgui_vpin::vpin_dialog;
use pbgui_vpin::vpin_dialog::LevelMap;
use qt_core::{Slot, SlotOfInt};
use qt_widgets::cpp_core::MutPtr;
use qt_widgets::QApplication;
use qt_widgets::{QMainWindow, QPushButton};
use rustqt_utils::enclose;
use std::rc::Rc;

fn main() {
    QApplication::init(|_app| unsafe {
        let mut main = QMainWindow::new_0a();
        let mut main_ptr = main.as_mut_ptr();
        let mut button = QPushButton::new();
        let button_ptr = button.as_mut_ptr();
        main.set_central_widget(button.into_ptr());

        let dialog = Rc::new(create_dialog("DEV01", "modelpublish-1.2.0", main_ptr));
        // we can create and hook up a finished slot. However, the finished slot will be activated
        // whether the user selects Ok or Cancel.
        dialog.set_show_name("DEV02");
        dialog.set_distribution("modelpublish-1.3.0");
        let finished_slot = SlotOfInt::new(move |result: std::os::raw::c_int| {
            println!("finished_slot -> {}", result);
        });

        dialog.finished().connect(&finished_slot);

        // we create a slot that is triggered when OK is pressed to act only in the event
        // that the user has requested action.
        let accepted_slot = Slot::new(enclose! { (dialog) move || {
            if let Some(roles) = dialog.selected_roles() {
                println!("roles: {:?}", roles);
            } else {
                println!("roles: any");
            }
            if let Some(selected_level) = dialog.selected_level() {
                println!("level: {:?}", selected_level);
            } else {
                println!("level: {}", dialog.show_name());
            }
            match dialog.selected_site(){
                Some(site) => println!(
                    "site:  {}", site
                ),
                None => println!("site:  Any"),
            }
            dialog.accept();
        }});
        // Connect the accepted signal to the accepted slot
        dialog.accepted().connect(&accepted_slot);

        let exec_dialog_slot = Slot::new(enclose! { (dialog) move || {
            let result = dialog.dialog_mut().exec(); //
            println!("exec_dialog_slot triggered by button result -> {}", result);
        }});

        button_ptr.pressed().connect(&exec_dialog_slot);
        main_ptr.show();
        QApplication::exec()
    });
}

unsafe fn create_dialog<'a, I: Into<String>>(
    name: I,
    distribution: &'a str,
    main_ptr: MutPtr<QMainWindow>,
) -> vpin_dialog::VpinDialog<'a> {
    let dialog = vpin_dialog::VpinDialog::create(name, distribution, main_ptr);
    dialog.set_default_stylesheet();
    dialog.set_roles(vec![
        "anim", "integ", "model", "fx", "cfx", "light", "comp", "roto",
    ]);
    let levelmap = initialize_levelmap();
    dialog.set_levels(levelmap);

    dialog.set_sites(vec!["hyderabad", "montreal", "playa", "vancouver"]);
    dialog
}

fn initialize_levelmap() -> LevelMap {
    let mut lm = LevelMap::new();
    lm.insert(
        "RD".to_string(),
        vec![
            "0001".to_string(),
            "0002".to_string(),
            "0003".to_string(),
            "9999".to_string(),
        ],
    );
    lm.insert(
        "AA".to_string(),
        vec![
            "0001".to_string(),
            "0002".to_string(),
            "0003".to_string(),
            "0004".to_string(),
        ],
    );
    lm
}
