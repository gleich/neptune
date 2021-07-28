package setup

import (
	"errors"
	"os/exec"
)

// Check to make sure that all needed binaries to operate exist
func CheckBin() error {
	requiredBins := []string{
		"pdflatex",
		"pandoc",
		"git",
	}

	for _, bin := range requiredBins {
		path, err := exec.LookPath(bin)
		if err != nil {
			return err
		}
		if path == "" {
			return errors.New(
				"Missing the " + bin + " program. Please install it or make sure it is in your system's path.",
			)
		}
	}

	return nil
}
