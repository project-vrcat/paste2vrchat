package app

import (
	"errors"
	"fmt"
	"github.com/jchv/go-webview2"
	"github.com/project-vrcat/paste2vrchat/internal/w32"
	"log"
	"net/url"
	"os"
	"os/exec"
	"unsafe"
)

func GUI() {
	w := webview2.New(false)
	defer w.Destroy()
	defer func() {
		if server == nil {
			return
		}
		_ = server.Close()
	}()

	bind(w)

	updateIcon(w.Window())
	w.SetTitle(fmt.Sprintf("Paste2VRChat %v", buildInfo.Version))
	w.SetSize(800, 600, webview2.HintNone)
	w.Dispatch(func() {
		u := fmt.Sprintf("http://%s/", <-addrCh)
		w.Navigate(u)
	})
	w.Run()
}

func updateIcon(hWnd unsafe.Pointer) {
	w32.SendMessage(hWnd, 0x0080, 1, w32.ExtractIcon(os.Args[0], 0))
}

func BindPasteText(text string) error {
	log.Println("PasteText:", text)
	return nil
}

func BindBuildInfo() (info interface{}, err error) {
	info = buildInfo
	return
}

func BindOpenURL(urlStr string) error {
	if u, err := url.Parse(urlStr); err != nil || (u.Scheme != "http" && u.Scheme != "https") {
		return err
	} else if u.Scheme != "http" && u.Scheme != "https" {
		return errors.New("scheme does not support")
	}
	return exec.Command("rundll32", "url.dll,FileProtocolHandler", urlStr).Start()
}

func bind(w webview2.WebView) {
	_ = w.Bind("pasteText", BindPasteText)
	_ = w.Bind("buildInfo", BindBuildInfo)
	_ = w.Bind("openURL", BindOpenURL)
}
