package main

import (
	"os"

	"github.com/therecipe/qt/core"
	"github.com/therecipe/qt/uitools"
	"github.com/therecipe/qt/widgets"
)

func main() {
	widgets.NewQApplication(len(os.Args), os.Args)

	MainWindow().Show()

	widgets.QApplication_Exec()
}

func MainWindow() *widgets.QWidget {

	file := core.NewQFile2(":/../ui/mainwindow.ui")
	file.Open(core.QIODevice__ReadOnly)
	formWidget := uitools.NewQUiLoader(nil).Load(file, nil)
	file.Close()

	formWidget.SetWindowTitle("Guess the number")

	var (
		ui_ok = widgets.NewQPushButtonFromPointer(formWidget.FindChild("ok", core.Qt__FindChildrenRecursively).Pointer())
		ui_number = widgets.NewQLineEditFromPointer(formWidget.FindChild("number", core.Qt__FindChildrenRecursively).Pointer())
		ui_output  = widgets.NewQLabelFromPointer(formWidget.FindChild("output", core.Qt__FindChildrenRecursively).Pointer())
	)

	ui_ok.ConnectClicked(func(bool){

	})

	ui_number.ConnectReturnPressed(func(){
		ui_output.SetText("Test")
	})

	return formWidget
}