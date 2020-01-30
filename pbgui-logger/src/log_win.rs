use crate::inner_log_win::LogData;
use crate::InnerLogWin;
use qt_core::Slot;
use qt_gui::QStandardItemModel;
use qt_widgets::{
    cpp_core::{CastInto, MutPtr},
    QTableView, QWidget,
};
use rustqt_utils::enclose;
use std::rc::Rc;

pub struct LogWin<'a> {
    inner_log_win: Rc<InnerLogWin>,
    clear_log: Slot<'a>,
}

impl<'a> LogWin<'a> {
    /// New up a LogWin instance
    pub unsafe fn new(parent: impl CastInto<MutPtr<QWidget>>) -> Self {
        let inner = Rc::new(InnerLogWin::new(parent));
        inner.set_default_stylesheet();
        let log_win = Self {
            inner_log_win: inner.clone(),
            clear_log: Slot::new(enclose! { (inner) move || {
               inner.clear_log();
            }}),
        };
        log_win
            .inner()
            .clear_button()
            .clicked()
            .connect(&log_win.clear_log);
        //configure
        log_win.inner().set_ctrls_visible(false);
        log_win
    }
    /// Retrieve the reference counted pointer to the InnerLogWin
    pub unsafe fn inner(&self) -> Rc<InnerLogWin> {
        self.inner_log_win.clone()
    }

    pub fn clear_log(&self) {
        self.inner_log_win.clear_log();
    }
    /// Retrieve a mutable pointer to the list_view's model
    pub unsafe fn model(&self) -> MutPtr<QStandardItemModel> {
        self.inner_log_win.model()
    }

    /// Retrieve a mutable pointer to the main list_view
    pub unsafe fn table_view(&self) -> MutPtr<QTableView> {
        self.inner_log_win.table_view()
    }

    pub unsafe fn log(&self, log_data: Option<LogData>, msg: &str) {
        self.inner_log_win.log(log_data, msg);
    }
}
