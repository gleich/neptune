package conf

import (
	"os"

	"github.com/pelletier/go-toml/v2"
)

// Read from a configration file
func Read() (RootConfiguration, error) {
	// Read from the file
	b, err := os.ReadFile(FileName)
	if err != nil {
		return RootConfiguration{}, err
	}

	// Parse the toml
	var data RootConfiguration
	err = toml.Unmarshal(b, &data)
	if err != nil {
		return RootConfiguration{}, err
	}

	return data, nil
}
