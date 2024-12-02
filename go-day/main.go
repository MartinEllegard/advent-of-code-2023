package main

import (
	"fmt"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"strings"
)

const (
	replaceID           = "AABBCCDD"
	replaceID2          = "AABBCCDDEE"
	bootstrapFolderName = "bootstrap"
	reponame            = "https://github.com/martinellegard/aoc.git"
)

func main() {
	args := os.Args[1:]

	if len(args) < 2 {
		fmt.Println()
		fmt.Println("install requires your aoc year as the first argument and day as the second argument")
		fmt.Println()
		fmt.Println("\tgo run go-day/install.go [aoc_year] [aoc_day]")
		fmt.Println()
		os.Exit(1)
	}
	year := args[0]
	day := args[1]

	projectName := fmt.Sprint("aoc", year, "day", day)

	println(projectName)
	// Open bootstrap
	template := os.DirFS("go-day/bootstrap")

	// Copy bootstrap
	fullPath := fmt.Sprint("./", year, "/", projectName)
	// os.Mkdir(fullPath, os.ModeAppend.)
	// Copy bootstrap content into new day
	err := os.CopyFS(fullPath, template)
	if err != nil {
		log.Fatal("Failed to copy template to designated area")
	}

	err = filepath.Walk(fullPath, func(fullPath string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}

		b, err := os.ReadFile(fullPath)
		if err != nil {
			return err
		}

		contentStr := string(b)
		if strings.Contains(contentStr, replaceID) || strings.Contains(contentStr, replaceID2) {
			replacedContent := strings.ReplaceAll(
				strings.ReplaceAll(contentStr, replaceID2, fullPath),
				replaceID,
				projectName)
			file, err := os.OpenFile(fullPath, os.O_WRONLY|os.O_TRUNC, 0644)
			if err != nil {
				return err
			}
			defer file.Close()
			_, err = file.WriteString(replacedContent)
			if err != nil {
				return err
			}
		}
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}
}
