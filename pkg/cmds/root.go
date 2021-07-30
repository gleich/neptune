package cmds

import (
	"fmt"

	"github.com/spf13/cobra"
)

const description = "Notes as a thoughtful, simple, and easy to use system to last a lifetime."

// Root command for the application
var RootCMD = &cobra.Command{
	Use:   "neptune",
	Short: description,
	Long: fmt.Sprintf(
		` ________       _______       ________    _________    ___  ___      ________       _______
|\   ___  \    |\  ___ \     |\   __  \  |\___   ___\ |\  \|\  \    |\   ___  \    |\  ___ \
\ \  \\ \  \   \ \   __/|    \ \  \|\  \ \|___ \  \_| \ \  \\\  \   \ \  \\ \  \   \ \   __/|
 \ \  \\ \  \   \ \  \_|/__   \ \   ____\     \ \  \   \ \  \\\  \   \ \  \\ \  \   \ \  \_|/__
  \ \  \\ \  \   \ \  \_|\ \   \ \  \___|      \ \  \   \ \  \\\  \   \ \  \\ \  \   \ \  \_|\ \
   \ \__\\ \__\   \ \_______\   \ \__\          \ \__\   \ \_______\   \ \__\\ \__\   \ \_______\
    \|__| \|__|    \|_______|    \|__|           \|__|    \|_______|    \|__| \|__|    \|_______|

%v

Made with ❤️ by Matt Gleich 2021
`,
		description,
	),
}
