use crate::main_window::InnerMainWindow;
use crate::messaging::outgoing::omain_win::OMainWin;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use std::rc::Rc;
/// update the main versionpin table by gathering the user's requested query parameters from    
/// the comboboxes up top, and sending a message to the secondary thread asking to get
/// version pins.
///
/// # Arguments
/// * `toolbar` - shared pointer to the MainToolbar
/// * `to_thread_sender` - sends OMsgs over a channel to the secondary thread
///
/// # Returns
/// * None
///
/// # Panics
/// If it is unable to send the message to the non-ui thread via the channel sender                        
pub fn update_vpin_table(main_window: Rc<InnerMainWindow>, to_thread_sender: Sender<OMsg>) {
    unsafe {
        let toolbar = main_window.main_toolbar();
        let dirtxt = toolbar.dir().current_text().to_std_string();
        let showtxt = toolbar.level().current_text().to_std_string();
        let roletxt = toolbar.role().current_text().to_std_string();
        let platformtxt = toolbar.platform().current_text().to_std_string();
        let sitetxt = toolbar.site().current_text().to_std_string();
        let packagetxt = toolbar.line_edit().text().to_std_string();
        let packagetxt = if &packagetxt != "" {
            Some(packagetxt)
        } else {
            None
        };
        log::debug!("signaling GetVpins");

        to_thread_sender
            .send(OMsg::MainWin(OMainWin::GetVpins {
                mode: main_window.search_mode(),
                package: packagetxt,
                level: showtxt,
                role: roletxt,
                platform: platformtxt,
                site: sitetxt,
                dir: dirtxt,
            }))
            .expect("unable to get vpins");
    }
}
