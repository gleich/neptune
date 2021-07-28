package conf

import "time"

// Outline for a config
type Configuration struct {
	Name    string `toml:"name,omitempty"`
	Author  string
	Started time.Time
	Format  string
}
