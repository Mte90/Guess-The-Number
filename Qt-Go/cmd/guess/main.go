package main

import (
	"os"
	"fmt"
	"strconv"
	"math/rand"

	"github.com/therecipe/qt/core"
	"github.com/therecipe/qt/widgets"
	"github.com/Mte90/Guess-The-Number/Qt-Go/uigen"
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
	window.Ok = widgets.NewQPushButtonFromPointer(window.Widget.FindChild("ok", core.Qt__FindChildrenRecursively).Pointer())
	window.Number = widgets.NewQLineEditFromPointer(window.Widget.FindChild("number", core.Qt__FindChildrenRecursively).Pointer())
	window.Output  = widgets.NewQLabelFromPointer(window.Widget.FindChild("output", core.Qt__FindChildrenRecursively).Pointer())
	guess := func() {
		user_number, err := strconv.Atoi(window.Number.Text())
		var label string
		if err != nil {
			min := 0
    		max := 10
    		the_number := rand.Intn(max - min) + min
			if the_number == user_number {
				window.Output.SetStyleSheet("QLabel { color : green; }")
				label = fmt.Sprintf("Your number is %d and is right!", user_number)
			} else {
				window.Output.SetStyleSheet("QLabel { color : green; }")
				label = fmt.Sprintf("Your number is %d but the right was %d!", user_number, the_number)
			}
		}
		window.Output.SetText(label)
	}
	window.Ok.ConnectClicked(func(_ bool){
		guess()
	})
	window.Number.ConnectReturnPressed(func(){
		guess()
	})
	window.Widget.Show()
	app.Exec()
}