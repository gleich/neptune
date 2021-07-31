package new

import (
	"os/exec"

	"github.com/gleich/neptune/pkg/conf"
	"github.com/gleich/neptune/pkg/out"
)

// Open a file in your favorite editor
func OpenFile(config conf.RootConfiguration, filePath string) error {
	if len(config.OpenCommand) != 0 {
		err := exec.Command(
			config.OpenCommand[0],
			append(config.OpenCommand[1:], filePath)...).Run()
		if err != nil {
			return err
		}
		out.Ok("Opened", filePath)
	}
	return nil
}
