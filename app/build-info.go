package app

import (
	_ "embed"
	"encoding/json"
	"runtime"
)

//go:embed build.json
var buildInfoFile []byte

var buildInfo struct {
	Version   string `json:"version"`
	GitHash   string `json:"gitHash"`
	BuildTime string `json:"buildTime"`
	GoVersion string `json:"goVersion"`
}

func init() {
	_ = json.Unmarshal(buildInfoFile, &buildInfo)
	buildInfo.GoVersion = runtime.Version()
}
