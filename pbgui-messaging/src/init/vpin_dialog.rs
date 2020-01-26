use crate::outgoing::ovpin_dialog::OVpinDialog;
use crate::OMsg;
use crate::Sender;

/// Given a channel Sender and a default show, request that the VpinDialog
/// be initialized.
///
/// # Arguments
/// * `to_thread_sender` - A channel Sender used to communicate with the secondary, non ui thread
/// * `default_show` - The name of the show to gather roles for
pub fn init<I>(to_thread_sender: Sender<OMsg>, default_show: I)
where
    I: Into<String>,
{
    to_thread_sender
        .send(OMsg::VpinDialog(OVpinDialog::GetRoles))
        .expect("unable to get roles");
    to_thread_sender
        .send(OMsg::VpinDialog(OVpinDialog::GetSites))
        .expect("unable to get sites");
    to_thread_sender
        .send(OMsg::VpinDialog(OVpinDialog::GetLevels(
            default_show.into(),
        )))
        .expect("unable to get levels");
}
