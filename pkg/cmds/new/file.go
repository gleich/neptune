package new

import (
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/AlecAivazis/survey/v2"
	"github.com/fatih/color"
	"github.com/gleich/neptune/pkg/out"
	"github.com/gleich/neptune/pkg/util"
)

// Outline for a markdown file
type markdownFile struct {
	folder string
	name   string
	binary []byte
}

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
	var (
		location string
	)
	switch fType {
	case "galaxy":
		folder := "galaxies"
		for {
			galaxies, err := util.ListFoldersIn(folder)
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
					color.BlueString(folder+"/"),
				),
				Options: galaxies,
			}, &selectedGalaxy)
			if err != nil {
				return "", err
			}
			folder = filepath.Join(folder, selectedGalaxy)
		}

		location, err = createMarkdownFile(markdownFile{
			folder: folder,
		})
		if err != nil {
			return "", err
		}
	case "nebula":
		now := time.Now()
		location, err = createMarkdownFile(
			markdownFile{
				folder: filepath.Join("nebulas", fmt.Sprint(now.Year()), now.Month().String()),
				name:   now.Format(time.Kitchen) + ".md",
			},
		)
		if err != nil {
			return "", err
		}
	}

	return location, nil
}

// Ask the user what they want to call the file and then create it
func createMarkdownFile(data markdownFile) (string, error) {
	// Asking for the name of the file if needed
	if data.name == "" {
		err := survey.AskOne(&survey.Input{
			Message: "Name of the file (without file extension)",
		}, &data.name)
		if err != nil {
			return "", err
		}
		data.name = data.name + ".md"
	}

	// Creating the folder if it doesn't exist already
	if _, err := os.Stat(data.folder); os.IsNotExist(err) {
		err = os.MkdirAll(data.folder, 0777)
		if err != nil {
			return "", nil
		}
	}

	// Making the actual markdown file
	location := filepath.Join(data.folder, data.name)
	err := os.WriteFile(location, data.binary, 0655)
	if err != nil {
		return "", err
	}
	out.Ok("Created", location)
	return location, nil
}
