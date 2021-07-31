package conf

import "time"

// Name of the configuration file
const FileName = "neptune.toml"

// Outline for a configuration file
type RootConfiguration struct {
	Name        string    `toml:"name"`
	Author      string    `toml:"author"`
	Format      string    `toml:"format"`
	OpenCommand []string  `toml:"open_command"`
	Start       time.Time `toml:"start"`
}
