package main

import (
	"fmt"

	"github.com/gleich/neptune/pkg/conf"
	"github.com/gleich/neptune/pkg/out"
	"github.com/gleich/statuser/v2"
)

func main() {
	statuser.Emojis = false
	config, err := conf.Read()
	if err != nil {
		out.Problem(err, "Failed to parse config file")
	}
	fmt.Println(config)
}
