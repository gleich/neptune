package new

import (
	"errors"
	"fmt"
	"path/filepath"

	"github.com/AlecAivazis/survey/v2"
	"github.com/fatih/color"
	"github.com/gleich/neptune/pkg/util"
)

// Create a new markdown file
func File() error {
	var fType string
	err := survey.AskOne(&survey.Select{
		Message: "What is the file type?",
		Options: []string{"galaxy", "nebula"},
		Help:    "galaxy → Place for structured thoughts with real folders\nnebulae → random thoughts that are organized by time",
	}, &fType)
	if err != nil {
		return err
	}

	fmt.Println()
	switch fType {
	case "galaxy":
		createIn := "galaxies"
		for {
			galaxies, err := util.ListFoldersIn(createIn)
			if err != nil {
				return err
			}
			if len(galaxies) == 0 {
				break
			}

			var selectedGalaxy string
			err = survey.AskOne(&survey.Select{
				Message: fmt.Sprintf(
					"Which galaxy do you want to create it in from %v?",
					color.BlueString(createIn+"/"),
				),
				Options: galaxies,
			}, &selectedGalaxy)
			if err != nil {
				return err
			}
			createIn = filepath.Join(createIn, selectedGalaxy)
		}
		fmt.Println(createIn)
	case "nebula":
		return errors.New("Sorry nebulas this is currently no supported")
	}

	return nil
}
