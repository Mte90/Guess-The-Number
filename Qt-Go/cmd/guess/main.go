package main

import (
	"os"
	"fmt"
	"github.com/therecipe/qt/core"
	"github.com/therecipe/qt/widgets"
	"github.com/ilyasmg/Guess-The-Number/Qt-Go/uigen"
)


type MainWindow struct {
	uigen.UIMainwindowMainWindow
	Widget *widgets.QMainWindow
}


func main() {
	app := widgets.NewQApplication(len(os.Args), os.Args)
	window := &MainWindow{
		Widget: widgets.NewQMainWindow(nil, core.Qt__Window),
	}
	window.SetupUI(window.Widget)
	var (
		ui_ok = widgets.NewQPushButtonFromPointer(window.Widget.FindChild("ok", core.Qt__FindChildrenRecursively).Pointer())
		ui_number = widgets.NewQLineEditFromPointer(window.Widget.FindChild("number", core.Qt__FindChildrenRecursively).Pointer())
		ui_output  = widgets.NewQLabelFromPointer(window.Widget.FindChild("output", core.Qt__FindChildrenRecursively).Pointer())
	)
	ui_ok.ConnectClicked(func(bool){
		fmt.Println("Test1")
		ui_output.SetText("Test")
	})

	ui_number.ConnectReturnPressed(func(){
		fmt.Println("Test2")
		ui_output.SetText("Test")
	})
	window.Widget.Show()
	app.Exec()
}