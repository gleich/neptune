package setup

import (
	"os"
	"os/exec"

	"github.com/gleich/neptune/pkg/conf"
	"github.com/gleich/neptune/pkg/out"
	"github.com/pelletier/go-toml/v2"
)

// Make the actual neptune project
func Make(responses InitQuestionResponses) error {
	cwd, err := os.Getwd()
	if err != nil {
		return err
	}
	if responses.Location != cwd {
		if _, err := os.Stat(responses.Location); !os.IsNotExist(err) {
			err = os.RemoveAll(responses.Location)
			if err != nil {
				return err
			}
		}

		err = os.MkdirAll(responses.Location, 0777)
		if err != nil {
			return err
		}
		out.Success("Created", responses.Location)

		err = os.Chdir(responses.Location)
		if err != nil {
			return err
		}
	}

	// Writing to the config file
	b, err := toml.Marshal(responses.Config)
	if err != nil {
		return err
	}
	err = os.WriteFile(conf.Name, b, 0655)
	if err != nil {
		return err
	}
	out.Success("Created", conf.Name)

	// Running git init if needed
	if responses.GitInit {
		err = exec.Command("git", "init").Run()
		if err != nil {
			return err
		}
		out.Success("git init done")
	}

	return nil
}
