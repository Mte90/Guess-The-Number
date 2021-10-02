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
        for &(text, is_done) in &[
            ("Learn Qt", true),
            ("Learn Rust", true),
            ("Conquer the world", false),
        ] {
            let item = QStandardItem::new().into_ptr();
            item.set_text(&qs(text));
            item.set_checkable(true);
            item.set_check_state(if is_done {
                CheckState::Checked
            } else {
                CheckState::Unchecked
            });
            self.source_model.append_row_q_standard_item(item);
        }

        self.proxy_model.set_source_model(&self.source_model);
        self.proxy_model
            .set_filter_role(ItemDataRole::CheckStateRole.into());
        self.form.list.set_model(&self.proxy_model);
        self.form.add.clicked().connect(&self.slot_on_add_clicked());
        self.form
            .remove_selected
            .clicked()
            .connect(&self.slot_on_remove_selected_clicked());
        self.form
            .remove_completed
            .clicked()
            .connect(&self.slot_on_remove_completed_clicked());
        self.form
            .list
            .selection_model()
            .selection_changed()
            .connect(&self.slot_on_list_selection_changed());

        for button in &[
            &self.form.show_completed,
            &self.form.show_active,
            &self.form.show_all,
        ] {
            button.toggled().connect(&self.slot_on_filter_changed());
        }

        self.on_list_selection_changed();
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
        unsafe QApplication::exec()
    })
}
