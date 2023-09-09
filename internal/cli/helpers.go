package cli

import (
	"errors"
	"fmt"
	"os"
	"strings"
	"text/template"
	"time"

	"github.com/99designs/keyring"
	"github.com/adrg/xdg"
	"github.com/mitchellh/go-wordwrap"
	"github.com/zoni/pingrep/internal/pinboard"
)

// templateFuncs contains functions available to templates.
var templateFuncs = template.FuncMap{
	"formatTags": formatTags,
	"oneline":    oneline,
	"trimspace":  strings.TrimSpace,
	"wordwrap":   func(lim uint, s string) string { return wordwrap.WrapString(s, lim) },
}

// formatTags formats tags for display.
func formatTags(prependHashTag bool, tags []string) string {
	if !prependHashTag {
		return strings.Join(tags, " ")
	}
	if len(tags) == 0 {
		return ""
	}
	return "#" + strings.Join(tags, " #")
}

// oneline replaces all forms of extra whitespace (newlines, tabs, consecutive
// spaces) with a single space to ensure string consumes only one line.
func oneline(s string) string {
	s = strings.ReplaceAll(s, "\t", " ")
	s = strings.ReplaceAll(s, "\r\n", " ")
	s = strings.ReplaceAll(s, "\n", " ")
	s = strings.ReplaceAll(s, "  ", " ")
	return s
}

// exitIfError prints error and exits with a non-zero exit code if err is not nil.
func exitIfError(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %s\n", err)
		os.Exit(1)
	}
}

// getPinboardToken returns the pinboard API token from the keyring.
func getPinboardToken(kr keyring.Keyring) (string, error) {
	token, err := kr.Get(keyringAPITokenKeyName)
	if err != nil {
		return "", fmt.Errorf("read API token from keyring: %w. Try `%s login` first", err, programName)
	}
	return string(token.Data), nil
}

// createPinboardClient returns a new pinboard client.
func createPinboardClient(kr keyring.Keyring) (*pinboard.Client, error) {
	token, err := getPinboardToken(kr)
	if err != nil {
		return nil, err
	}
	return pinboard.NewClient(token)
}

// loadCollection returns the pinboard bookmark collection from the local cache.
func loadCollection() (*pinboard.Collection, error) {
	path, err := xdg.DataFile(fmt.Sprintf("%s/bookmarks.json", programName))
	if err != nil {
		return nil, fmt.Errorf("create data dir: %w", err)
	}

	collection, err := pinboard.FromFile(path)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			return nil, fmt.Errorf("load collection: %w. Try `%s update` first", err, programName)
		}
		return nil, fmt.Errorf("load collection: %w", err)
	}
	return collection, nil
}

// getLastUpdateTimestamp returns the time of the last update of the bookmarks file.
func getLastUpdateTimestamp(path string) (time.Time, error) {
	collection, err := pinboard.FromFile(path)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			// If the file doesn't exist, we'll just return the zero value,
			// a date that is well in the past and so forces an update.
			return time.Time{}, nil
		}
		return time.Time{}, err
	}
	return collection.LastUpdate, nil
}
