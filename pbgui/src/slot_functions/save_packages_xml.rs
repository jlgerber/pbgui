use crate::messaging::outgoing::OMainWin;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use crate::utility::qs;
use qt_widgets::{cpp_core::MutPtr, QComboBox, QFileDialog, QMainWindow};

/// Updates the withpackages in response to versionpin selection in the main view
pub fn save_packages_xml(
    main_window: MutPtr<QMainWindow>,
    level_cb: MutPtr<QComboBox>,
    to_thread_sender: Sender<OMsg>,
) {
    unsafe {
        let level = level_cb.current_text().to_std_string();
        log::info!("current show: {}", &level);
        // now I need qdialog
        let output_path = QFileDialog::get_save_file_name_4a(
            main_window,
            &qs("save packages.xml"),
            &qs(""),
            &qs("*.xml"),
        );
        if output_path.is_null() {
            log::debug!("packages.xml save cancelled by user");
            return;
        }
        let output = output_path.to_std_string();
        log::debug!("saving packages.xml to {}", output);
        to_thread_sender
            .send(OMsg::MainWin(OMainWin::SavePackagesXml {
                show: level,
                output,
            }))
            .expect("unable to get vpins");
    }
}
