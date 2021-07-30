package main

import (
	"github.com/gleich/neptune/pkg/out"
	"github.com/gleich/statuser/v2"
)

func main() {
	statuser.Emojis = false
	out.Ok("Done", "wow")
}
