package app

import (
	"fmt"
	bundle "github.com/GizmoOAO/go-asset-bundle"
	"mime"
	"net"
	"net/http"
	"os"
)

var (
	server *http.Server
	addrCh = make(chan string, 1)
)

func httpServer() {
	iLoveMicrosoft()

	listen := newLocalListener()
	addr := listen.Addr().(*net.TCPAddr)

	if *showGUI {
		addrCh <- addr.String()
	}

	server = &http.Server{
		Handler: http.FileServer(getFS()),
	}
	fmt.Printf("=> http server started on \u001B[1;32m:%d\u001B[0m\n", addr.Port)

	_ = server.Serve(listen)
}

func newLocalListener() net.Listener {
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return nil
	}
	return l
}

func getFS() http.FileSystem {
	f, err := os.Stat("public")
	if err == nil && f.IsDir() {
		return http.Dir("public")
	}
	ab, _ := bundle.OpenAssetBundle("public.ab")
	return ab
}

// https://github.com/golang/go/issues/32350
func iLoveMicrosoft() {
	_ = mime.AddExtensionType(".js", "application/javascript; charset=utf-8")
}
