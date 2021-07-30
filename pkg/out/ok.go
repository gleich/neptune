package out

import (
	"github.com/gleich/statuser/v2"
)

// Out a success message to the console
func Ok(msg ...interface{}) {
	statuser.Success(msg...)
}
