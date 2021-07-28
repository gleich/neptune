package conf

import "time"

// Outline for a config
type Configuration struct {
	Name    string
	Author  string
	started time.Time
	Format  string
}
