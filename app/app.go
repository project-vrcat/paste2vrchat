package app

import (
	"flag"
)

var (
	showGUI = flag.Bool("gui", true, "Show GUI")
)

func Start() {
	flag.Parse()
	start()
}

func start() {
	if *showGUI {
		go httpServer()
		GUI()
	} else {
		httpServer()
	}
}
