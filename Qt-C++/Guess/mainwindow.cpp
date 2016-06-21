#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    // Qt event on the button
    connect( ui->ok, SIGNAL( clicked() ), this, SLOT( guess() ) );
    connect( ui->number, SIGNAL( returnPressed() ), this, SLOT( guess() ) );
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
        label = QString("Your number is %1 and is right!").arg(userNumber);
    } else {
        ui->output->setStyleSheet("QLabel { color : red; }");
        label = QString("Your number is %1 but the right was %2!").arg(userNumber).arg(theNumber);
    }
    ui->output->setText(QString(label));
}
