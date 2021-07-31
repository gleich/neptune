package cmds

import (
	"github.com/gleich/neptune/pkg/cmds/new"
	"github.com/gleich/neptune/pkg/conf"
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
		config, err := conf.Read()
		if err != nil {
			out.Problem(err, "Failed to read from configuration file")
		}

		switch args[0] {
		case "file":
			location, err := new.File()
			if err != nil {
				out.Problem(err, "Failed to create file")
			}

			err = new.OpenFile(config, location)
			if err != nil {
				out.Problem(err, "Failed to open file")
			}
		}
	},
}

func init() {
	RootCMD.AddCommand(newCMD)
}
