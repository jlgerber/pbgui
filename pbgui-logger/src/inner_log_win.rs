use chrono::{DateTime, Local};
use log::Level;
use qt_core::{GlobalColor, QString};
use qt_gui::{QBrush, QStandardItem, QStandardItemModel};
use qt_widgets::{
    cpp_core::{CastInto, CppBox, MutPtr, Ref as QRef},
    q_abstract_item_view::SelectionBehavior,
    q_header_view::ResizeMode,
    QCheckBox, QFrame, QGridLayout, QGroupBox, QHBoxLayout, QPushButton, QTableView, QWidget,
};
use rustqt_utils::{create_hlayout, create_vlayout, qs, set_stylesheet_from_str};
use std::cell::Cell;

const STYLE_STR: &'static str = include_str!("../resources/pbgui_logger.qss");
const COL_0_WIDTH: i32 = 60;
const COL_1_WIDTH: i32 = 180;
const COL_2_WIDTH: i32 = 270;
const COL_3_WIDTH: i32 = 120;
const COL_4_WIDTH: i32 = 60;

pub struct LogData<'a> {
    pub target: &'a str,
    pub file: Option<&'a str>,
    pub line: Option<u32>,
}

impl<'a> LogData<'a> {
    pub fn target(&self) -> &'a str {
        self.target
    }

    pub fn file(&self) -> &'a str {
        match self.file {
            Some(ref file) => file,
            None => "",
        }
    }

    pub fn line(&self) -> u32 {
        match self.line {
            Some(v) => v,
            None => 0,
        }
    }
}

/// Configuration for the log level in the log ui's controls, dictating
/// the state of the checkboxes for log level
#[derive(Debug, Clone)]
pub struct LogLevelCtrlsConfig {
    pub trace: bool,
    pub debug: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

impl Default for LogLevelCtrlsConfig {
    fn default() -> Self {
        Self {
            trace: false,
            debug: false,
            info: true,
            warn: true,
            error: true,
        }
    }
}

/// Configuration for the log metadata in the log ui's controls, dictating
/// the state of the checkboxes for log metadata
pub struct LogMetadataCtrlsConfig {
    pub level: bool,
    pub datetime: bool,
    pub target: bool,
    pub file: bool,
    pub line: bool,
}

impl Default for LogMetadataCtrlsConfig {
    fn default() -> Self {
        Self {
            level: true,
            datetime: true,
            target: true,
            file: false,
            line: false,
        }
    }
}

struct LogLevelState {
    pub trace: Cell<bool>,
    pub debug: Cell<bool>,
    pub info: Cell<bool>,
    pub warn: Cell<bool>,
    pub error: Cell<bool>,
}

impl LogLevelState {
    fn new(input: &LogLevelCtrlsConfig) -> Self {
        Self {
            trace: Cell::new(input.trace),
            debug: Cell::new(input.debug),
            info: Cell::new(input.info),
            warn: Cell::new(input.warn),
            error: Cell::new(input.error),
        }
    }

    fn is_visible(&self, level: &Level) -> bool {
        match level {
            Level::Trace => self.trace.get(),
            Level::Debug => self.debug.get(),
            Level::Info => self.info.get(),
            Level::Warn => self.warn.get(),
            Level::Error => self.error.get(),
        }
    }
}

pub struct InnerLogWin {
    main: MutPtr<QFrame>,
    table_view: MutPtr<QTableView>,
    view_ctrls_qframe: MutPtr<QFrame>,
    clear_button: MutPtr<QPushButton>,
    trace_cb: MutPtr<QCheckBox>,
    debug_cb: MutPtr<QCheckBox>,
    info_cb: MutPtr<QCheckBox>,
    warn_cb: MutPtr<QCheckBox>,
    error_cb: MutPtr<QCheckBox>,
    level_md_cb: MutPtr<QCheckBox>,
    datetime_md_cb: MutPtr<QCheckBox>,
    target_md_cb: MutPtr<QCheckBox>,
    file_md_cb: MutPtr<QCheckBox>,
    line_md_cb: MutPtr<QCheckBox>,
    model: MutPtr<QStandardItemModel>,
    visible_levels: LogLevelState,
}

impl InnerLogWin {
    pub unsafe fn new(
        parent: impl CastInto<MutPtr<QWidget>>,
        levelconfig: &LogLevelCtrlsConfig,
        metadataconfig: &LogMetadataCtrlsConfig,
    ) -> Self {
        let mut main_frame = QFrame::new_0a();
        let main_frame_ptr = main_frame.as_mut_ptr();
        main_frame.set_object_name(&qs("LoggerMainFrame"));

        // create main layout
        let mut main_layout = create_hlayout();

        // create the view
        let mut view = QTableView::new_0a();
        view.set_object_name(&qs("LoggerTable"));

        //view.set_word_wrap(true);
        view.set_show_grid(false);
        let mut hheader = view.horizontal_header();
        hheader.set_stretch_last_section(true);
        hheader.set_section_resize_mode_1a(ResizeMode::Fixed);
        let view_ptr = view.as_mut_ptr();

        let mut model = QStandardItemModel::new_0a();
        model.set_column_count(6);
        let model_ptr = model.as_mut_ptr();
        view.set_model(model.into_ptr());

        let mut header = view_ptr.vertical_header();
        header.set_section_resize_mode_1a(ResizeMode::Fixed);
        header.set_default_section_size(1);
        view.horizontal_header().hide();

        view.set_selection_behavior(SelectionBehavior::SelectRows);

        view.vertical_header().hide();
        view.set_column_width(0, COL_0_WIDTH);
        view.set_column_width(1, COL_1_WIDTH);
        view.set_column_width(2, COL_2_WIDTH);
        view.set_column_width(3, COL_3_WIDTH);
        view.set_column_width(4, COL_4_WIDTH);

        // add the view to the main layout
        main_layout.add_widget(view.into_ptr());
        let AddCtrlsReturn {
            view_ctrls_qframe,
            clear_button,
            trace_cb,
            debug_cb,
            info_cb,
            warn_cb,
            error_cb,
            level_md_cb,
            datetime_md_cb,
            target_md_cb,
            file_md_cb,
            line_md_cb,
        } = Self::add_ctrls(main_layout.as_mut_ptr(), levelconfig, metadataconfig);
        // set the main layout
        main_frame.set_layout(main_layout.into_ptr());

        // add the main frame to the parent's layout
        let parent = parent.cast_into();
        let mut parent_layout = parent.layout();
        parent_layout.add_widget(main_frame.into_ptr());

        Self {
            main: main_frame_ptr,
            table_view: view_ptr,
            view_ctrls_qframe,
            clear_button,
            trace_cb,
            debug_cb,
            info_cb,
            warn_cb,
            error_cb,
            level_md_cb,
            datetime_md_cb,
            target_md_cb,
            file_md_cb,
            line_md_cb,
            model: model_ptr,
            visible_levels: LogLevelState::new(levelconfig),
        }
    }

    fn add_ctrls(
        mut parent_layout: MutPtr<QHBoxLayout>,
        levelconfig: &LogLevelCtrlsConfig,
        metadataconfig: &LogMetadataCtrlsConfig,
    ) -> AddCtrlsReturn {
        unsafe {
            // create view controls - main fram
            let mut view_ctrls_qframe = QFrame::new_0a();
            view_ctrls_qframe.set_object_name(&qs("LogCtrlsFrame"));
            let view_ctrls_qframe_ptr = view_ctrls_qframe.as_mut_ptr();
            let mut ctrls_top_l = create_vlayout();
            let mut ctrls_top_layout = ctrls_top_l.as_mut_ptr();
            view_ctrls_qframe.set_layout(ctrls_top_l.into_ptr());

            let mut ctrls_l = QGridLayout::new_0a(); //create_vlayout();
            let mut ctrls_layout = ctrls_l.as_mut_ptr();
            ctrls_layout.set_spacing(10);
            //view_ctrls_qframe.set_layout(ctrls_l.into_ptr());
            ctrls_top_layout.add_layout_1a(ctrls_l.into_ptr());
            ctrls_top_layout.add_stretch_0a();
            // add clear button
            let mut clear_button = QPushButton::from_q_string(&QString::from_std_str("Clear"));
            let clear_button_ptr = clear_button.as_mut_ptr();
            ctrls_layout.add_widget_5a(clear_button.into_ptr(), 0, 0, 1, 2);

            // spacer
            let mut spacer = QFrame::new_0a();
            spacer.set_object_name(&qs("Spacer"));
            spacer.set_minimum_width(20);
            spacer.set_minimum_height(20);
            ctrls_layout.add_widget_3a(spacer.into_ptr(), 1, 0);

            let mut loglevel_grp_box = QGroupBox::new();
            let mut loglevel_grp_box_ptr = loglevel_grp_box.as_mut_ptr();
            ctrls_layout.add_widget_3a(loglevel_grp_box.into_ptr(), 2, 0);

            let name = qs("Active Log Levels");
            loglevel_grp_box_ptr.set_title(&name);
            loglevel_grp_box_ptr.set_object_name(&name);
            let mut loglevel_grp_box_layout = create_vlayout();

            let mut trace_cb = QCheckBox::from_q_string(&qs("Trace"));
            let mut trace_cb_ptr = trace_cb.as_mut_ptr();
            trace_cb_ptr.set_checked(levelconfig.trace);
            loglevel_grp_box_layout.add_widget(trace_cb.into_ptr());

            let mut debug_cb = QCheckBox::from_q_string(&qs("Debug"));
            let mut debug_cb_ptr = debug_cb.as_mut_ptr();
            debug_cb_ptr.set_checked(levelconfig.debug);

            loglevel_grp_box_layout.add_widget(debug_cb.into_ptr());

            let mut info_cb = QCheckBox::from_q_string(&qs("Info"));
            let mut info_cb_ptr = info_cb.as_mut_ptr();
            info_cb_ptr.set_checked(levelconfig.info);

            loglevel_grp_box_layout.add_widget(info_cb.into_ptr());

            let mut warn_cb = QCheckBox::from_q_string(&qs("Warn"));
            let mut warn_cb_ptr = warn_cb.as_mut_ptr();
            warn_cb_ptr.set_checked(levelconfig.warn);

            loglevel_grp_box_layout.add_widget(warn_cb.into_ptr());

            let mut error_cb = QCheckBox::from_q_string(&qs("Error"));
            let mut error_cb_ptr = error_cb.as_mut_ptr();
            error_cb_ptr.set_checked(levelconfig.error);

            loglevel_grp_box_layout.add_widget(error_cb.into_ptr());

            loglevel_grp_box_ptr.set_layout(loglevel_grp_box_layout.into_ptr());

            // spacer
            // let mut spacer = QFrame::new_0a();
            // spacer.set_object_name(&qs("Spacer"));
            // spacer.set_minimum_width(20);
            // spacer.set_minimum_height(20);
            // ctrls_layout.add_widget(spacer.into_ptr());

            // second group

            let mut metadata_grp_box = QGroupBox::new();
            let mut metadata_grp_box_ptr = metadata_grp_box.as_mut_ptr();
            ctrls_layout.add_widget_3a(metadata_grp_box.into_ptr(), 2, 1);
            let name = qs("Log Metadata");

            metadata_grp_box_ptr.set_title(&name);
            metadata_grp_box_ptr.set_object_name(&name);
            let mut metadata_grp_box_layout = create_vlayout();

            let mut level_cb = QCheckBox::from_q_string(&qs("Level"));
            let mut level_cb_ptr = level_cb.as_mut_ptr();
            level_cb_ptr.set_checked(metadataconfig.level);

            metadata_grp_box_layout.add_widget(level_cb.into_ptr());

            let mut datetime_cb = QCheckBox::from_q_string(&qs("DateTime"));
            let mut datetime_cb_ptr = datetime_cb.as_mut_ptr();
            datetime_cb_ptr.set_checked(metadataconfig.datetime);

            metadata_grp_box_layout.add_widget(datetime_cb.into_ptr());

            let mut target_cb = QCheckBox::from_q_string(&qs("Target"));
            let mut target_cb_ptr = target_cb.as_mut_ptr();
            target_cb_ptr.set_checked(metadataconfig.target);

            metadata_grp_box_layout.add_widget(target_cb.into_ptr());

            let mut file_cb = QCheckBox::from_q_string(&qs("File"));
            let mut file_cb_ptr = file_cb.as_mut_ptr();
            file_cb_ptr.set_checked(metadataconfig.file);

            metadata_grp_box_layout.add_widget(file_cb.into_ptr());

            let mut line_cb = QCheckBox::from_q_string(&qs("Line"));
            let mut line_cb_ptr = line_cb.as_mut_ptr();
            line_cb_ptr.set_checked(metadataconfig.line);

            metadata_grp_box_layout.add_widget(line_cb.into_ptr());

            metadata_grp_box_ptr.set_layout(metadata_grp_box_layout.into_ptr());

            //ctrls_layout.add_stretch_0a();

            // spacer
            let mut spacer = QFrame::new_0a();
            spacer.set_object_name(&qs("Spacer"));
            spacer.set_minimum_width(20);
            spacer.set_minimum_height(20);
            ctrls_layout.add_widget_5a(spacer.into_ptr(), 4, 0, 1, 2);

            //view_ctrls_qframe.set_layout(ctrls_layout.into_ptr());
            parent_layout.add_widget(view_ctrls_qframe.into_ptr());
            AddCtrlsReturn {
                view_ctrls_qframe: view_ctrls_qframe_ptr,
                clear_button: clear_button_ptr,
                trace_cb: trace_cb_ptr,
                debug_cb: debug_cb_ptr,
                info_cb: info_cb_ptr,
                warn_cb: warn_cb_ptr,
                error_cb: error_cb_ptr,
                level_md_cb: level_cb_ptr,
                datetime_md_cb: datetime_cb_ptr,
                target_md_cb: target_cb_ptr,
                file_md_cb: file_cb_ptr,
                line_md_cb: line_cb_ptr,
            }
        }
    }

    /// Retrieve a mutable pointer to the root widget of the InnerLogWin
    pub fn main(&self) -> MutPtr<QFrame> {
        self.main
    }

    /// Retrieve a mutable pointer to the main view (qlistview)
    pub fn table_view(&self) -> MutPtr<QTableView> {
        self.table_view
    }

    /// Return a mutable pointer to the view control frame
    pub fn ctrls(&self) -> MutPtr<QFrame> {
        self.view_ctrls_qframe
    }

    /// Return a mutable pointer to the clear button
    pub fn clear_button(&self) -> MutPtr<QPushButton> {
        self.clear_button
    }

    /// Retrieve a mutable pointer to the model
    pub fn model(&self) -> MutPtr<QStandardItemModel> {
        self.model
    }

    /// Return the trace level checkbox from the log controlx
    pub fn trace_cb(&self) -> MutPtr<QCheckBox> {
        self.trace_cb
    }

    /// hide or unhide the file metadata checkbox
    pub fn hide_trace_cb(&self, hide: bool) {
        unsafe {
            // let mut view = self.table_view();
            //view.set_column_hidden(0, hide);
            if self.visible_levels.trace.get() == hide {
                let trace = QString::from_std_str("TRACE");
                self.visible_levels.trace.set(!hide);
                let mut view = self.table_view();
                for row in 0..self.model.row_count_0a() {
                    if self.model.item_2a(row, 0).text().compare_q_string(&trace) == 0 {
                        view.set_row_hidden(row, hide);
                    }
                }
            }
        }
    }
    /// Return the debug level checkbox from the log controlx
    pub fn debug_cb(&self) -> MutPtr<QCheckBox> {
        self.debug_cb
    }

    /// hide or unhide the file metadata checkbox
    pub fn hide_debug_cb(&self, hide: bool) {
        unsafe {
            // this looks backwards becasue we set the
            // value to !hide
            if self.visible_levels.debug.get() == hide {
                let debug = QString::from_std_str("DEBUG");
                self.visible_levels.debug.set(!hide);
                let mut view = self.table_view();
                for row in 0..self.model.row_count_0a() {
                    if self.model.item_2a(row, 0).text().compare_q_string(&debug) == 0 {
                        view.set_row_hidden(row, hide);
                    }
                }
            }
        }
    }
    /// Return the info level checkbox from the log controlx
    pub fn info_cb(&self) -> MutPtr<QCheckBox> {
        self.info_cb
    }

    /// hide or unhide the file metadata checkbox
    pub fn hide_info_cb(&self, hide: bool) {
        unsafe {
            if self.visible_levels.info.get() == hide {
                let info = QString::from_std_str("INFO");
                self.visible_levels.info.set(!hide);
                let mut view = self.table_view();
                for row in 0..self.model.row_count_0a() {
                    if self.model.item_2a(row, 0).text().compare_q_string(&info) == 0 {
                        view.set_row_hidden(row, hide);
                    }
                }
            }
        }
    }

    /// Return the warn level checkbox from the log controlx
    pub fn warn_cb(&self) -> MutPtr<QCheckBox> {
        self.warn_cb
    }

    /// hide or unhide the file metadata checkbox
    pub fn hide_warn_cb(&self, hide: bool) {
        unsafe {
            if self.visible_levels.warn.get() == hide {
                let warn = QString::from_std_str("WARN");
                self.visible_levels.warn.set(!hide);
                let mut view = self.table_view();
                for row in 0..self.model.row_count_0a() {
                    if self.model.item_2a(row, 0).text().compare_q_string(&warn) == 0 {
                        view.set_row_hidden(row, hide);
                    }
                }
            }
        }
    }

    /// Return the error level checkbox from the log controlx
    pub fn error_cb(&self) -> MutPtr<QCheckBox> {
        self.error_cb
    }

    /// hide or unhide the file metadata checkbox
    pub fn hide_error_cb(&self, hide: bool) {
        unsafe {
            if self.visible_levels.trace.get() == hide {
                let error = QString::from_std_str("ERROR");
                self.visible_levels.error.set(!hide);
                let mut view = self.table_view();
                for row in 0..self.model.row_count_0a() {
                    if self.model.item_2a(row, 0).text().compare_q_string(&error) == 0 {
                        view.set_row_hidden(row, hide);
                    }
                }
            }
        }
    }

    /// Return the level metadata checkbox from the log controlx
    pub fn level_md_cb(&self) -> MutPtr<QCheckBox> {
        self.level_md_cb
    }

    /// hide or unhide the level metadata checkbox
    pub fn hide_level_md_cb(&self, hide: bool) {
        unsafe {
            let mut view = self.table_view();
            view.set_column_hidden(0, hide);
        }
    }

    /// Return the level metadata checkbox from the log controlx
    pub fn datetime_md_cb(&self) -> MutPtr<QCheckBox> {
        self.datetime_md_cb
    }

    /// hide or unhide the level metadata checkbox
    pub fn hide_datetime_md_cb(&self, hide: bool) {
        unsafe {
            let mut view = self.table_view();
            view.set_column_hidden(1, hide);
        }
    }

    /// Return the target metadata checkbox from the log controlx
    pub fn target_md_cb(&self) -> MutPtr<QCheckBox> {
        self.target_md_cb
    }

    /// hide or unhide the target metadata checkbox
    pub fn hide_target_md_cb(&self, hide: bool) {
        unsafe {
            let mut view = self.table_view();
            view.set_column_hidden(2, hide);
        }
    }

    /// Return the file metadata checkbox from the log controlx
    pub fn file_md_cb(&self) -> MutPtr<QCheckBox> {
        self.file_md_cb
    }

    /// hide or unhide the file metadata checkbox
    pub fn hide_file_md_cb(&self, hide: bool) {
        unsafe {
            let mut view = self.table_view();
            view.set_column_hidden(3, hide);
        }
    }

    /// Return the line metadata checkbox from the log controlx
    pub fn line_md_cb(&self) -> MutPtr<QCheckBox> {
        self.line_md_cb
    }

    /// hide or unhide the line metadata checkbox
    pub fn hide_line_md_cb(&self, hide: bool) {
        unsafe {
            let mut view = self.table_view();
            view.set_column_hidden(4, hide);
        }
    }

    /// Set the visibility of metadata columns based on the configuration struct
    ///
    /// # Arguments
    /// * `config` - A reference to an instance of the LogMetadataCtrlsConfig struct
    ///
    /// # Returns
    /// * None
    pub fn configure_view_columns(&self, config: &LogMetadataCtrlsConfig) {
        self.hide_level_md_cb(!config.level);
        self.hide_datetime_md_cb(!config.datetime);
        self.hide_target_md_cb(!config.target);
        self.hide_file_md_cb(!config.file);
        self.hide_line_md_cb(!config.line);
    }
    /// perminently clear the contents of the log
    pub fn clear_log(&self) {
        unsafe {
            let cnt = self.model.row_count_0a();
            let mut model = self.model;
            model.remove_rows_2a(0, cnt);
        }
    }

    /// Turn the controls on and off
    pub fn set_ctrls_visible(&self, visible: bool) {
        unsafe {
            let mut ctrls = self.view_ctrls_qframe;
            ctrls.set_visible(visible)
        };
    }

    /// set the default stylesheet for the child components
    pub fn set_default_stylesheet(&self) {
        set_stylesheet_from_str(STYLE_STR, self.main);
    }

    fn level_to_str(level: &Level) -> &str {
        match level {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }

    /// Log data
    pub fn log(&self, level: Level, log_data: Option<LogData>, msg: &str) {
        unsafe {
            let mut item = QStandardItem::new();
            item.set_editable(false);

            let dt: DateTime<Local> = Local::now();
            let dt_str = dt.format("%a %b %e %T %Y");
            let mut datetime = QStandardItem::new();
            datetime.set_editable(false);
            datetime.set_text(&qs(dt_str.to_string().as_str()));

            let mut loglevel = QStandardItem::new();
            loglevel.set_editable(false);

            let mut target_item = QStandardItem::new();
            target_item.set_editable(false);

            let mut mp_item = QStandardItem::new();
            mp_item.set_editable(false);

            let mut file_item = QStandardItem::new();
            file_item.set_editable(false);

            let mut line_item = QStandardItem::new();
            line_item.set_editable(false);

            let mut model = self.model();
            let rc = model.row_count_0a();
            model.set_row_count(rc + 1);

            match &log_data {
                &Some(LogData {
                    target, file, line, ..
                }) => {
                    item.set_text(&qs(msg));
                    loglevel.set_text(&qs(Self::level_to_str(&level)));
                    target_item.set_text(&qs(target));
                    file_item.set_text(&qs(file.unwrap_or("").split("/").last().unwrap_or("")));
                    line_item.set_text(&qs(line.unwrap_or(0).to_string().as_str()));

                    let brush = QBrush::from_global_color(GlobalColor::Cyan);
                    loglevel.set_foreground(brush.as_ref());
                    datetime.set_foreground(brush.as_ref());
                    target_item.set_foreground(brush.as_ref());
                    file_item.set_foreground(brush.as_ref());
                    line_item.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, datetime.into_ptr());
                    model.set_item_3a(rc, 2, target_item.into_ptr());
                    model.set_item_3a(rc, 3, file_item.into_ptr());
                    model.set_item_3a(rc, 4, line_item.into_ptr());
                    model.set_item_3a(rc, 5, item.into_ptr());
                    if !self.visible_levels.is_visible(&level) {
                        self.table_view().set_row_hidden(rc, true);
                    }
                }
                &None => {
                    loglevel.set_text(&qs(""));
                    datetime.set_text(&qs(""));
                    target_item.set_text(&qs(""));
                    file_item.set_text(&qs(""));
                    line_item.set_text(&qs(""));

                    item.set_text(&qs(msg));
                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, datetime.into_ptr());
                    model.set_item_3a(rc, 2, target_item.into_ptr());
                    model.set_item_3a(rc, 3, file_item.into_ptr());
                    model.set_item_3a(rc, 4, line_item.into_ptr());
                    model.set_item_3a(rc, 5, item.into_ptr());
                    if !self.visible_levels.is_visible(&level) {
                        self.table_view().set_row_hidden(rc, true);
                    }
                }
            }
            self.table_view().scroll_to_bottom();
        }
    }

    pub unsafe fn log_qstrings(
        &self,
        msg: QRef<QString>,
        level: QRef<QString>,
        color: GlobalColor,
    ) {
        let mut item = QStandardItem::new();
        let mut loglevel = QStandardItem::new();
        let mut model = self.model();
        let rc = model.row_count_0a();
        model.set_row_count(rc + 1);
        loglevel.set_text(level);
        item.set_text(msg);
        let brush = QBrush::from_global_color(color);
        loglevel.set_foreground(brush.as_ref());

        model.set_item_3a(rc, 0, loglevel.into_ptr());
        model.set_item_3a(rc, 1, item.into_ptr());
    }

    pub unsafe fn log_items(&self, item: CppBox<QStandardItem>, loglevel: CppBox<QStandardItem>) {
        let mut model = self.model();
        let rc = model.row_count_0a();
        model.set_row_count(rc + 1);
        model.set_item_3a(rc, 0, loglevel.into_ptr());
        model.set_item_3a(rc, 1, item.into_ptr());
    }
}

/// Structure returned by the add_ctrls associated
/// function
pub struct AddCtrlsReturn {
    view_ctrls_qframe: MutPtr<QFrame>,
    clear_button: MutPtr<QPushButton>,
    trace_cb: MutPtr<QCheckBox>,
    debug_cb: MutPtr<QCheckBox>,
    info_cb: MutPtr<QCheckBox>,
    warn_cb: MutPtr<QCheckBox>,
    error_cb: MutPtr<QCheckBox>,
    level_md_cb: MutPtr<QCheckBox>,
    datetime_md_cb: MutPtr<QCheckBox>,
    target_md_cb: MutPtr<QCheckBox>,
    file_md_cb: MutPtr<QCheckBox>,
    line_md_cb: MutPtr<QCheckBox>,
}
