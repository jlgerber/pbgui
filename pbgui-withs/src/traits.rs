use qt_widgets::{cpp_core::MutPtr, QFrame, QHBoxLayout, QVBoxLayout, QWidget};
//
// TRAITS
//

pub unsafe trait NewWidget<P, R> {
    fn create(parent: &MutPtr<P>) -> MutPtr<R>;
}

unsafe impl NewWidget<QWidget, QWidget> for QWidget {
    fn create(parent: &MutPtr<QWidget>) -> MutPtr<QWidget> {
        unsafe {
            let mut main = QWidget::new_0a();
            let main_ptr = main.as_mut_ptr();
            let mut parent_ptr = parent.layout();
            assert!(!parent_ptr.is_null());
            parent_ptr.add_widget(main.into_ptr());
            main_ptr
        }
    }
}

unsafe impl NewWidget<QWidget, QFrame> for QFrame {
    fn create(parent: &MutPtr<QWidget>) -> MutPtr<QFrame> {
        unsafe {
            let mut main = QFrame::new_0a();
            let main_ptr = main.as_mut_ptr();
            let mut parent_ptr = parent.layout();
            assert!(!parent_ptr.is_null());
            parent_ptr.add_widget(main.into_ptr());
            main_ptr
        }
    }
}

/// Choose the type of layout that you want to create
/// in the AddLayout trait implementation
#[allow(dead_code)]
pub enum LayoutType {
    VBoxLayout,
    HBoxLayout,
}

/// Trait provides a function to add a layout to
pub unsafe trait AddLayout<R> {
    type Layout;
    fn add_layout(&mut self, layout: Self::Layout) -> MutPtr<R>;
}

fn add_layout_to_widget(widget: &mut MutPtr<QWidget>, layout: LayoutType) {
    unsafe {
        match layout {
            LayoutType::VBoxLayout => {
                let mut layout = QVBoxLayout::new_0a();
                layout.set_margin(0);
                layout.set_contents_margins_4a(0, 0, 0, 0);
                layout.set_spacing(0);
                widget.set_layout(layout.into_ptr());
            }
            LayoutType::HBoxLayout => {
                let mut layout = QHBoxLayout::new_0a();
                layout.set_margin(0);
                layout.set_contents_margins_4a(0, 0, 0, 0);
                layout.set_spacing(0);
                widget.set_layout(layout.into_ptr());
            }
        }
    }

    unsafe impl AddLayout<QWidget> for MutPtr<QWidget> {
        type Layout = LayoutType;

        fn add_layout(&mut self, layout: LayoutType) -> MutPtr<QWidget> {
            unsafe {
                add_layout_to_widget(self, layout);
                self.as_mut_ref().unwrap().as_mut_ptr()
            }
        }
    }
}

unsafe impl AddLayout<QFrame> for MutPtr<QFrame> {
    type Layout = LayoutType;

    fn add_layout(&mut self, layout: LayoutType) -> MutPtr<QFrame> {
        unsafe {
            let mut qw: MutPtr<QWidget> = self.static_upcast_mut();
            add_layout_to_widget(&mut qw, layout);
            self.as_mut_ref().unwrap().as_mut_ptr()
        }
    }
}
