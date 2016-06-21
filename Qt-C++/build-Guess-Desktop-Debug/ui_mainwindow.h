/********************************************************************************
** Form generated from reading UI file 'mainwindow.ui'
**
** Created by: Qt User Interface Compiler version 5.6.1
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_MAINWINDOW_H
#define UI_MAINWINDOW_H

#include <QtCore/QVariant>
#include <QtWidgets/QAction>
#include <QtWidgets/QApplication>
#include <QtWidgets/QButtonGroup>
#include <QtWidgets/QHeaderView>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_MainWindow
{
public:
    QWidget *centralWidget;
    QLineEdit *number;
    QPushButton *ok;
    QLabel *output;

    void setupUi(QMainWindow *MainWindow)
    {
        if (MainWindow->objectName().isEmpty())
            MainWindow->setObjectName(QStringLiteral("MainWindow"));
        MainWindow->resize(279, 63);
        centralWidget = new QWidget(MainWindow);
        centralWidget->setObjectName(QStringLiteral("centralWidget"));
        number = new QLineEdit(centralWidget);
        number->setObjectName(QStringLiteral("number"));
        number->setGeometry(QRect(10, 10, 161, 31));
        ok = new QPushButton(centralWidget);
        ok->setObjectName(QStringLiteral("ok"));
        ok->setGeometry(QRect(190, 10, 84, 32));
        ok->setAutoDefault(true);
        output = new QLabel(centralWidget);
        output->setObjectName(QStringLiteral("output"));
        output->setGeometry(QRect(10, 40, 261, 21));
        MainWindow->setCentralWidget(centralWidget);

        retranslateUi(MainWindow);

        ok->setDefault(true);


        QMetaObject::connectSlotsByName(MainWindow);
    } // setupUi

    void retranslateUi(QMainWindow *MainWindow)
    {
        MainWindow->setWindowTitle(QApplication::translate("MainWindow", "Guess the number", 0));
        ok->setText(QApplication::translate("MainWindow", "Guess!", 0));
        output->setText(QApplication::translate("MainWindow", "Insert a number between 0 and 10", 0));
    } // retranslateUi

};

namespace Ui {
    class MainWindow: public Ui_MainWindow {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_MAINWINDOW_H
