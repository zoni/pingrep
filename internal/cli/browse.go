package cli

import (
	"github.com/pkg/browser"
)

type browseCmd struct {
	URL string `arg:"" help:"The URL of the bookmark to open in a browser."`
}

func (cmd *browseCmd) Run(_ *globals) error {
	return browser.OpenURL(cmd.URL)
}
