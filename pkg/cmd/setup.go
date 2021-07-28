package cmd

import (
	"fmt"

	"github.com/gleich/neptune/pkg/cmd/setup"
	"github.com/gleich/neptune/pkg/out"
	"github.com/spf13/cobra"
)

// neptune init command outline
var InitCMD = &cobra.Command{
	Use:   "setup",
	Short: "Start a new neptune book",
	Run: func(cmd *cobra.Command, args []string) {
		err := setup.CheckBin()
		if err != nil {
			out.Error(err, "Required binary check failed")
		}

		responses, err := setup.Ask()
		if err != nil {
			out.Error(err, "Couldn't ask the user setup questions")
		}
		fmt.Println(responses)
	},
}

func init() {
	RootCMD.AddCommand(InitCMD)
}
