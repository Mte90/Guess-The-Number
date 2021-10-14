// WARNING! All changes made in this file will be lost!
package uigen

import (
	"github.com/therecipe/qt/core"
	"github.com/therecipe/qt/widgets"
)

type UIMainwindowMainWindow struct {
	CentralWidget *widgets.QWidget
	Number *widgets.QLineEdit
	Ok *widgets.QPushButton
	Output *widgets.QLabel
}

func (this *UIMainwindowMainWindow) SetupUI(MainWindow *widgets.QMainWindow) {
	MainWindow.SetObjectName("MainWindow")
	MainWindow.SetGeometry(core.NewQRect4(0, 0, 279, 63))
	this.CentralWidget = widgets.NewQWidget(MainWindow, core.Qt__Widget)
	this.CentralWidget.SetObjectName("CentralWidget")
	this.Number = widgets.NewQLineEdit(this.CentralWidget)
	this.Number.SetObjectName("Number")
	this.Number.SetGeometry(core.NewQRect4(10, 10, 161, 31))
	this.Ok = widgets.NewQPushButton(this.CentralWidget)
	this.Ok.SetObjectName("Ok")
	this.Ok.SetGeometry(core.NewQRect4(190, 10, 84, 32))
	this.Ok.SetAutoDefault(true)
	this.Ok.SetDefault(true)
	this.Output = widgets.NewQLabel(this.CentralWidget, core.Qt__Widget)
	this.Output.SetObjectName("Output")
	this.Output.SetGeometry(core.NewQRect4(10, 40, 261, 21))
	MainWindow.SetCentralWidget(this.CentralWidget)


    this.RetranslateUi(MainWindow)

}

func (this *UIMainwindowMainWindow) RetranslateUi(MainWindow *widgets.QMainWindow) {
    _translate := core.QCoreApplication_Translate
	MainWindow.SetWindowTitle(_translate("MainWindow", "Guess the number", "", -1))
	this.Ok.SetText(_translate("MainWindow", "Guess!", "", -1))
	this.Output.SetText(_translate("MainWindow", "Insert a number between 0 and 10", "", -1))
}
