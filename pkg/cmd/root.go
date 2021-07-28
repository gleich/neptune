package cmd

import "github.com/spf13/cobra"

const description = "📜 Notes as a book"

// Root command for the application
var RootCMD = &cobra.Command{
	Use:   "neptune",
	Short: description,
	Long: `
███╗   ██╗███████╗██████╗ ████████╗██╗   ██╗███╗   ██╗███████╗
████╗  ██║██╔════╝██╔══██╗╚══██╔══╝██║   ██║████╗  ██║██╔════╝
██╔██╗ ██║█████╗  ██████╔╝   ██║   ██║   ██║██╔██╗ ██║█████╗
██║╚██╗██║██╔══╝  ██╔═══╝    ██║   ██║   ██║██║╚██╗██║██╔══╝
██║ ╚████║███████╗██║        ██║   ╚██████╔╝██║ ╚████║███████╗
╚═╝  ╚═══╝╚══════╝╚═╝        ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝

` + description,
}
