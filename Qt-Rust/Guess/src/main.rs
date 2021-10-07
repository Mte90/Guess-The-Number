use rand::{self, Rng};
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{slot, QBox, QObject, SlotNoArgs, QPtr, QString};
use qt_ui_tools::ui_form;
use qt_widgets::{
    QApplication, QLineEdit, QPushButton, QWidget, QLabel
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
        let the_number = rng.gen_range(0..=10);
        unsafe {
            let user_number = self.form.number.text().to_int_0a();
            let label;
            if user_number == the_number {
                self.form.output.set_style_sheet(&QString::from_std_str("QLabel { color : green; }"));
                label = format!("Your number is {} and is right!", user_number);
            }
            else {
                self.form.output.set_style_sheet(&QString::from_std_str("QLabel { color : red; }"));
                label = format!("Your number is {} but the right was {}!", user_number, the_number);
            }
            self.form.output.set_text(&QString::from_std_str(label));
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