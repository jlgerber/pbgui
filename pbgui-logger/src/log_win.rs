use crate::inner_log_win::LogData;
use crate::InnerLogWin;
use qt_gui::QStandardItemModel;
use qt_widgets::{
    cpp_core::{CastInto, MutPtr},
    QTableView, QWidget,
};
use std::rc::Rc;

pub struct LogWin {
    inner_log_win: Rc<InnerLogWin>,
}

impl LogWin {
    /// New up a LogWin instance
    pub unsafe fn new(parent: impl CastInto<MutPtr<QWidget>>) -> Self {
        let inner = Rc::new(InnerLogWin::new(parent));
        inner.set_default_stylesheet();
        Self {
            inner_log_win: inner,
        }
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
