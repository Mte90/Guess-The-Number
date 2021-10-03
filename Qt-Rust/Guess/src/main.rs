use std::rc::Rc;
use qt_core::{QBox, QPtr, QString};
use qt_ui_tools::ui_form;
use qt_widgets::{QApplication, QPushButton, QWidget};

#[ui_form("../ui/mainwindow.ui")]
#[derive(Debug)]
struct Form {
    widget: QBox<QWidget>,
    guess: QPtr<QPushButton>,
}

#[derive(Debug)]
struct MainWindow {
    form: Form,
}

impl MainWindow {
    fn new() -> Rc<Self> {
        unsafe {
            let this = Rc::new(MainWindow {
                form: Form::load(),
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.form.guess.clicked().connect(&self.guess());
    }

    fn guess(&self) {
        println!("Test");
    }

    fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }
}
fn main() {
    QApplication::init(|_| {
        let window = MainWindow::new();
        window.show();
        unsafe { QApplication::exec() }
    })
}
