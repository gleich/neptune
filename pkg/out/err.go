package out

import (
	"os"
	"strings"

	"github.com/gleich/lumber"
)

// Output an error and include a stack trace if needed
func Error(err error, msg ...interface{}) {
	// Don't output a stack trace if running outside of go run
	if !strings.HasSuffix(os.Args[0], "main") {
		lumber.ShowStack = false
	}
	lumber.Fatal(err, msg...)
}
