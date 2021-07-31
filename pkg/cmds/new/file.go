package new

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"

	"github.com/AlecAivazis/survey/v2"
	"github.com/fatih/color"
	"github.com/gleich/neptune/pkg/out"
	"github.com/gleich/neptune/pkg/util"
)

// Create a new markdown file
func File() (string, error) {
	var fType string
	err := survey.AskOne(&survey.Select{
		Message: "What is the file type?",
		Options: []string{"galaxy", "nebula"},
		Help:    "galaxy → Place for structured thoughts with real folders\nnebulae → random thoughts that are organized by time",
	}, &fType)
	if err != nil {
		return "", err
	}

	fmt.Println()
	var location string
	switch fType {
	case "galaxy":
		createIn := "galaxies"
		for {
			galaxies, err := util.ListFoldersIn(createIn)
			if err != nil {
				return "", err
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
				return "", err
			}
			createIn = filepath.Join(createIn, selectedGalaxy)
		}
		location, err = createMarkdownFile(createIn)
		if err != nil {
			return "", err
		}
	case "nebula":
		return "", errors.New("Sorry nebulas this is currently no supported")
	}

	return location, nil
}

// Ask the user what they want to call the file and then create it
func createMarkdownFile(location string) (string, error) {
	// Asking for the name of the file
	var name string
	err := survey.AskOne(&survey.Input{
		Message: "Name of the file (without file extension)",
	}, &name)
	if err != nil {
		return "", err
	}

	// Making the actual markdown file
	location = filepath.Join(location, name+".md")
	err = os.WriteFile(location, []byte{}, 0655)
	if err != nil {
		return "", err
	}
	out.Ok("Created", location)
	return location, nil
}
