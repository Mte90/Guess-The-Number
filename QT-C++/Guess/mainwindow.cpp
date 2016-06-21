#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    // QT event on the button
    connect( ui->ok, SIGNAL( clicked() ), this, SLOT( guess() ) );
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::guess()
{
    // number random between 1 and 10
    int theNumber = int(qrand() % ((10) - 1) + 1);
    int userNumber = ui->number->text().toInt();
    QString label;
    if(theNumber == userNumber) {
        ui->output->setStyleSheet("QLabel { color : green; }");
        sprintf(label, "Your number is %u and is right!", userNumber);
    } else {
        ui->output->setStyleSheet("QLabel { color : red; }");
        sprintf(label, "Your number is %u but the right was %u!", userNumber, theNumber);
    }
    ui->output->setText(QString(label));
}
