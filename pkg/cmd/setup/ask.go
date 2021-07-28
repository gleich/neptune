package setup

import (
	"errors"
	"os"
	"path/filepath"

	"github.com/AlecAivazis/survey/v2"
)

// Ask the user a bunch of questions as part of the setup process
func Ask() (InitQuestionResponses, error) {
	responses := InitQuestionResponses{}

	// Asking for the location to setup a neptune project
	err := survey.AskOne(&survey.Input{
		Message: "What is the location where you want to setup your neptune project?",
		Default: ".",
	}, &responses.Location)
	if err != nil {
		return InitQuestionResponses{}, err
	}
	responses.Location, err = filepath.Abs(responses.Location)
	if err != nil {
		return InitQuestionResponses{}, err
	}

	// If the folder already exists than make sure the user wants to reset it.
	confirmed := false
	if _, err := os.Stat(responses.Location); os.IsNotExist(err) {
		err = survey.AskOne(&survey.Confirm{
			Message: "Are you sure that you want to create the folder: " + responses.Location,
		}, &confirmed)
		if err != nil {
			return InitQuestionResponses{}, err
		}
	} else {
		err = survey.AskOne(&survey.Confirm{
			Message: "It looks like " + responses.Location + " already exists. Are you sure that you want to reset it?",
		}, &confirmed)
		if err != nil {
			return InitQuestionResponses{}, err
		}
	}
	if !confirmed {
		return InitQuestionResponses{}, errors.New("User failed to confirm start directory")
	}

	// Ask the user if they want to have neptune git init the repo
	err = survey.AskOne(&survey.Confirm{
		Message: "Do you want to have your project be git initialized?",
	}, &responses.GitInit)
	if err != nil {
		return InitQuestionResponses{}, err
	}

	return responses, nil
}
