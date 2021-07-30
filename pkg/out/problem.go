package out

import (
	"os"
	"strings"

	"github.com/gleich/lumber"
)

// Output a problem to the console
func Problem(err error, msg ...interface{}) {
	// Don't output stack if in prod
	if !strings.HasSuffix(os.Args[0], "main") {
		lumber.ShowStack = false
	}
	lumber.Fatal(err, msg...)
}
