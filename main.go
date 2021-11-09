//go:generate rsrc -manifest app.manifest -ico app.ico -o rsrc.syso
package main

import (
	"github.com/project-vrcat/paste2vrchat/app"
)

func main() { app.Start() }
