use rand::{self, Rng};
use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{qs, slot, ContextMenuPolicy, QBox, QObject, QPoint, SlotNoArgs, QPtr, QString};
use qt_ui_tools::ui_form;
use qt_widgets::{
    QAction, QApplication, QLineEdit, QMenu, QMessageBox, QPushButton, QVBoxLayout, QWidget, SlotOfQPoint, QLabel
};
use std::rc::Rc;


#[ui_form("../ui/mainwindow.ui")]
#[derive(Debug)]
struct Form {
    widget: QBox<QWidget>,
    ok: QPtr<QPushButton>,
    number: QPtr<QLineEdit>,
    output: QPtr<QLabel>,
}

#[derive(Debug)]
struct TodoWidget {
    form: Form
}

impl StaticUpcast<QObject> for TodoWidget {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.widget.as_ptr().static_upcast()
    }
}

impl TodoWidget {
    fn new() -> Rc<Self> {
        unsafe {
            let this = Rc::new(TodoWidget {
                form: Form::load(),  
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.form.ok.clicked().connect(&self.slot_guess());
        self.form.number.return_pressed().connect(&self.slot_guess());
    }

    #[slot(SlotNoArgs)]
    fn guess(self: &Rc<Self>) {
        let mut rng = rand::thread_rng();
        let the_number = rng.gen_range(1..=10);
        unsafe {
            let user_number = self.form.number.text().to_int_0a();
            if user_number == the_number {
                println!("Your number is {} and is right!", user_number)
            }
            else {
                println!("Your number is {} but the right was {}!", user_number, the_number)
            }
            // let label = QString::new().as_ptr();
            // label.arg_i64(the_number);
            //self.form.output.set_text(QString::from_std_str("Test"));
        }
    }
    
    fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let todo_widget = TodoWidget::new();
        todo_widget.show();
        QApplication::exec()
    })
}