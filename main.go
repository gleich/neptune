package main

import (
	"github.com/gleich/neptune/pkg/cmd"
	"github.com/gleich/neptune/pkg/out"
)

func main() {
	err := cmd.RootCMD.Execute()
	if err != nil {
		out.Error(err, "Failed to execute root command")
	}
}
