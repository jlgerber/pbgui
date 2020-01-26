use crate::messaging::outgoing::omain_win::OMainWin;
use crate::messaging::OMsg;
use crate::messaging::Sender;
use pbgui_toolbar::toolbar;
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
pub fn update_vpin_table(toolbar: Rc<toolbar::MainToolbar>, to_thread_sender: Sender<OMsg>) {
    unsafe {
        let dirtxt = toolbar.dir().current_text().to_std_string();
        let showtxt = toolbar.level().current_text().to_std_string();
        let roletxt = toolbar.role().current_text().to_std_string();
        let platformtxt = toolbar.platform().current_text().to_std_string();
        let sitetxt = toolbar.site().current_text().to_std_string();

        to_thread_sender
            .send(OMsg::MainWin(OMainWin::GetVpins {
                level: showtxt,
                role: roletxt,
                platform: platformtxt,
                site: sitetxt,
                dir: dirtxt,
                package: None,
            }))
            .expect("unable to get vpins");
    }
}
