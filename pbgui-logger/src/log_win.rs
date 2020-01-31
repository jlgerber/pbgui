use crate::inner_log_win::{LogData, LogLevelCtrlsConfig, LogMetadataCtrlsConfig};
use crate::InnerLogWin;
use qt_core::{Slot, SlotOfInt};
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
    toggle_level_cb: SlotOfInt<'a>,
    toggle_datetime_cb: SlotOfInt<'a>,
    toggle_target_cb: SlotOfInt<'a>,
    toggle_file_cb: SlotOfInt<'a>,
    toggle_line_cb: SlotOfInt<'a>,
}

impl<'a> LogWin<'a> {
    /// New up a LogWin instance
    pub unsafe fn new(parent: impl CastInto<MutPtr<QWidget>>) -> Self {
        let log_level_ctrls_config = LogLevelCtrlsConfig::default();
        let log_metadata_ctrls_config = LogMetadataCtrlsConfig::default();

        let inner = Rc::new(InnerLogWin::new(
            parent,
            &log_level_ctrls_config,
            &log_metadata_ctrls_config,
        ));
        inner.set_default_stylesheet();
        let log_win = Self {
            inner_log_win: inner.clone(),
            clear_log: Slot::new(enclose! { (inner) move || {
               inner.clear_log();
            }}),
            toggle_level_cb: SlotOfInt::new(enclose! {(inner) move |checked: i32| {
                inner.hide_level_md_cb(checked<1)
            }}),
            toggle_datetime_cb: SlotOfInt::new(enclose! {(inner) move |checked: i32| {
                inner.hide_datetime_md_cb(checked<1)
            }}),
            toggle_target_cb: SlotOfInt::new(enclose! {(inner) move |checked: i32| {
                inner.hide_target_md_cb(checked<1)
            }}),
            toggle_file_cb: SlotOfInt::new(enclose! {(inner) move |checked: i32| {
                inner.hide_file_md_cb(checked<1)
            }}),
            toggle_line_cb: SlotOfInt::new(enclose! {(inner) move |checked: i32| {
                inner.hide_line_md_cb(checked<1)
            }}),
        };

        log_win
            .inner()
            .clear_button()
            .clicked()
            .connect(&log_win.clear_log);
        //configure
        let inner = log_win.inner();
        inner.set_ctrls_visible(false);
        inner.configure_view_columns(&log_metadata_ctrls_config);
        inner
            .level_md_cb()
            .state_changed()
            .connect(&log_win.toggle_level_cb);
        inner
            .datetime_md_cb()
            .state_changed()
            .connect(&log_win.toggle_datetime_cb);
        inner
            .target_md_cb()
            .state_changed()
            .connect(&log_win.toggle_target_cb);
        inner
            .file_md_cb()
            .state_changed()
            .connect(&log_win.toggle_file_cb);
        inner
            .line_md_cb()
            .state_changed()
            .connect(&log_win.toggle_line_cb);
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
