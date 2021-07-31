package main

import (
	"github.com/gleich/neptune/pkg/cmds"
	"github.com/gleich/neptune/pkg/out"
	"github.com/gleich/statuser/v2"
)

func main() {
	statuser.Emojis = false
	err := cmds.RootCMD.Execute()
	if err != nil {
		out.Problem(err, "Failed to exectue root command")
	}
}
