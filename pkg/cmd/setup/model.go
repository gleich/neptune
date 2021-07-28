package setup

import "github.com/gleich/neptune/pkg/conf"

// Outline for the init question responses
type InitQuestionResponses struct {
	Location string
	GitInit  bool
	Config   conf.Configuration
}
