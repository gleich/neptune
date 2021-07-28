package out

import "github.com/gleich/lumber"

// Output a success message
func Success(msg ...interface{}) {
	lumber.Success(msg...)
}
