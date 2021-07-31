package cmds

import (
	"github.com/gleich/neptune/pkg/cmds/new"
	"github.com/gleich/neptune/pkg/out"
	"github.com/spf13/cobra"
)

// neptune new command
var newCMD = &cobra.Command{
	Use:       "new",
	Short:     "Create a new file in a nebulae or in a galaxy",
	Args:      cobra.ExactArgs(1),
	ValidArgs: []string{"file", "nebula", "galaxy"},
	Run: func(cmd *cobra.Command, args []string) {
		switch args[0] {
		case "file":
			err := new.File()
			if err != nil {
				out.Problem(err, "Failed to create file")
			}
		}
	},
}

func init() {
	RootCMD.AddCommand(newCMD)
}
