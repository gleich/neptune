package conf

import "time"

// Name of the config file
const Name = "neptune.toml"

// Outline for a config
type Configuration struct {
	Name    string    `toml:"name"`
	Author  string    `toml:"author"`
	Started time.Time `toml:"start"`
	Format  string    `toml:"format"`
}
