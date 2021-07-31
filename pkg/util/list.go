package util

import (
	"os"
)

// Get a list of folders in a folder.
// Returns the full path to the folder.
func ListFoldersIn(path string) ([]string, error) {
	fs, err := os.ReadDir(path)
	if err != nil {
		return []string{}, err
	}

	folders := []string{}
	for _, fileOrFolder := range fs {
		if fileOrFolder.IsDir() {
			folders = append(folders, fileOrFolder.Name())
		}
	}
	return folders, nil
}
