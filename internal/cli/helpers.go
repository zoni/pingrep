package cli

import (
	"errors"
	"fmt"
	"os"
	"strings"
	"text/template"

	"github.com/99designs/keyring"
	"github.com/adrg/xdg"
	"github.com/zoni/pingrep/internal/pinboard"
)

// templateFuncs contains functions available to templates.
var templateFuncs = template.FuncMap{
	"formatTags": formatTags,
	"oneline":    oneline,
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
