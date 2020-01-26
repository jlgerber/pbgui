use crate::inner_vpin_dialog::InnerVpinDialog;
pub use crate::inner_vpin_dialog::LevelMap;
use crate::inner_vpin_dialog::DEFAULT_SITE;
use qt_core::{QString, Signal, SlotOfQString};
use qt_widgets::{
    cpp_core::{CastInto, MutPtr, Ptr, Ref},
    QComboBox, QDialog, QWidget,
};
use std::cell::RefCell;
use std::os::raw::c_int;
use std::rc::Rc;

/// VpinDialog combines an InnerVpinDialog instance, which it exposes through
/// an immutable interace using the Rusty Interior Mutability Pattern, along with
/// a Slot to handle internal state changes which do not need to be exposed to the
/// outside.
///
/// This nested construction serves two purposes
/// * limits the periods of mutability
/// * provides a convenient interface for Slots.
///
/// On the second point, we would not be able to call any methods on the InnerVpinDialog
/// if we didn't split up the impl between the core qt hierarchy and slots. This approach
/// provides a greater degree of abstraction, at the cost of an additional method call.
///
/// Usage is rather simple.
///
/// # Example
/// ```rust,ignore
///
/// fn main() {
///    
///    QApplication::init(|_app| unsafe {
/// ...
///       // Initaliize the dialog
///
///       let dialog = Rc::new(create_dialog("DEV01", "modelpublish-1.2.0", main_ptr));
///
///      // Create an accepted slot
///      let accepted_slot = Slot::new(enclose! { (dialog) move || {
///         if let Some(roles) = dialog.selected_roles() {
///             println!("roles: {:?}", roles);
///         } else {
///             println!("roles: any");
///         }
///         if let Some(selected_level) = dialog.selected_level() {
///             println!("level: {:?}", selected_level);
///         } else {
///             println!("level: {}", dialog.show_name());
///         }
///         match dialog.selected_site(){
///             Some(site) => println!(
///                 "site:  {}", site
///             ),
///             None => println!("site:  Any"),
///         }
///         dialog.accept();
///      }});
///
///      // Connect the accepted signal to the accepted slot
///      dialog.accepted().connect(&accepted_slot);
///
///      let exec_dialog_slot = Slot::new(enclose! { (dialog) move || {
///         let result = dialog.dialog_mut().exec(); //
///         println!("exec_dialog_slot triggered by button result -> {}", result);
///      }});
///    }
/// }
/// ```
pub struct VpinDialog<'a> {
    dialog: Rc<RefCell<InnerVpinDialog<'a>>>,
    seq_changed: SlotOfQString<'a>,
}

impl<'a> VpinDialog<'a> {
    /// Create the dialog, given a show name, a distribution, and a parent widget
    ///
    /// # Arguments
    /// * `show` - The name of the show
    /// * `distribution` - The distribtuion we are setting a pin for
    /// * `parent` - The parent widget we will attach to
    ///
    /// #  Returns
    /// * VpinDialog instance
    pub unsafe fn create<I: Into<String>>(
        show: I,
        distribution: &str,
        parent: impl CastInto<MutPtr<QWidget>>,
    ) -> VpinDialog {
        let inner_vpin_dialog = Rc::new(RefCell::new(InnerVpinDialog::create(
            show,
            distribution,
            parent,
        )));
        let ivd = inner_vpin_dialog.clone();
        let seq_changed = SlotOfQString::new(move |idx: Ref<QString>| {
            let sequence = idx.to_std_string();
            ivd.borrow().set_shots_for_seq(sequence.as_str());
        });
        let dialog = VpinDialog {
            dialog: inner_vpin_dialog,
            seq_changed,
        };
        dialog
            .seqs_cb()
            .current_index_changed2()
            .connect(&dialog.seq_changed);
        dialog
    }

    /// Return the accepted signal from the button. This is provided as a convenience
    /// for hooking up a slot from this struct.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Signal that sends `()`
    pub unsafe fn accepted(&self) -> Signal<()> {
        self.dialog.borrow().accepted()
    }

    /// Dismiss the dialog using accept. This is a convenience for consumrs
    /// of this struct, to avoid having to drill down.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * None
    pub unsafe fn accept(&self) {
        self.dialog.borrow_mut().accept()
    }

    /// Return the `finished` Signal so that connections to slots may be made.
    ///
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Signal that sends a `c_int`
    pub unsafe fn finished(&self) -> Signal<(c_int,)> {
        self.dialog.borrow().finished()
    }

    /// Get a pointer to the internal dialog
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * A Ptr to the inner QDialog instance
    pub fn dialog(&self) -> Ptr<QDialog> {
        self.dialog.borrow().dialog()
    }

    /// Get a mutable pointer to the inner dialog widget
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to QDialoge
    pub fn dialog_mut(&self) -> MutPtr<QDialog> {
        self.dialog.borrow_mut().dialog_mut()
    }

    /// Return a `rejected` signal instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Signal instance of type `()`
    pub unsafe fn rejected(&self) -> Signal<()> {
        self.dialog.borrow().rejected()
    }

    /// Return a Some wrapped vector of specific role names, if any are selected. Otherwise,
    /// returns None
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Some Vec of String if roles are selected
    /// * None otherwise
    pub unsafe fn selected_roles(&self) -> Option<Vec<String>> {
        self.dialog.borrow().selected_roles()
    }

    /// Retrieve an Option wrapped current site, if specified
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Some wrapped site name, if specified
    /// * None, if `any` site specified
    pub unsafe fn selected_site(&self) -> Option<String> {
        let sel_site = self.dialog.borrow().selected_site();
        if sel_site == DEFAULT_SITE {
            return None;
        }
        Some(sel_site)
    }
    /// Return the show's name. Unfortunately, we have to disambiguate between
    /// the `show` widget name, and the model
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * The show name as a String
    pub fn show_name(&self) -> String {
        self.dialog.borrow().show_name()
    }

    /// Set show name
    pub fn set_show_name<I>(&self, new_name: I)
    where
        I: Into<String>,
    {
        self.dialog.borrow().set_show_name(new_name.into());
    }
    /// Return the a Some wrapped Sequence/shot if the user has activated
    /// the checkbox and selected a sequence or shot. Otherwise, it returns
    /// None
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Option<String>
    pub unsafe fn selected_level(&self) -> Option<String> {
        self.dialog.borrow().selected_level()
    }

    /// Set the distribution name
    ///
    /// # Arguments
    /// * distribution - The distribution name
    ///
    /// # Returns
    /// * None
    pub unsafe fn set_distribution(&self, distribution: &str) {
        self.dialog.borrow().set_distribution(distribution);
    }

    /// Load the default stylesheet
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * None
    pub unsafe fn set_default_stylesheet(&self) {
        self.dialog.borrow_mut().set_default_stylesheet();
    }

    /// Set the sites. This should be done before calling exec on
    /// the inner dialog.
    ///
    /// # Arguments
    /// * The site names as a vector of &str
    ///
    /// # Returns
    /// * None
    pub fn set_sites(&self, sites: Vec<&str>) {
        self.dialog.borrow().set_sites(sites);
    }

    /// set the list of roles. This should before calling exec
    /// on the inner dialog.
    ///
    /// # Arguments
    /// * The roles as a Vector of &str
    ///
    /// # Returns
    /// * None
    pub fn set_roles(&self, roles: Vec<&str>) {
        self.dialog.borrow().set_roles(roles);
    }

    /// Retrieve a mutable pointer to the sequences QComboBox
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr wrapping the QComboBox for the sequences
    pub fn seqs_cb(&self) -> MutPtr<QComboBox> {
        self.dialog.borrow().seqs_cb()
    }

    // /// Given a vector of Strings, set levels
    // pub fn set_levels_old(&self, levels: Vec<String>) {
    //     //let levels = self.dialog.borrow().seqs();
    //     self.dialog.borrow().set_levels(levels);
    // }

    /// Initialize the sequences and shots givne the provided
    /// LevelMap
    ///
    /// # Arguments
    /// * `levels` - a LevelMap instance
    ///
    /// # Returns
    /// * None
    pub fn set_levels(&self, levels: LevelMap) {
        self.set_levels_map(levels);
        self.set_levels_from_map();
    }

    // Helper function that, given a new LevelMap, replaces the existing one.
    //
    // # Arguments
    // * `levels` - A LevelMap whose keys are Sequences and whose shots are
    //              vectors of shots
    fn set_levels_map(&self, levels: LevelMap) {
        self.dialog.borrow_mut().set_levels_map(levels);
    }

    // Helper method that initializes the qt widgets once the level map has been
    // set
    //
    // # Arguments
    // * None
    //
    // # Returns
    // * None
    fn set_levels_from_map(&self) {
        self.dialog.borrow().set_levels_from_map();
    }
}
