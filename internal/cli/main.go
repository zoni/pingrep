package cli

import (
	"github.com/99designs/keyring"
	"github.com/alecthomas/kong"
)

const (
	programName            = "pingrep"
	keyringAPITokenKeyName = "pinboard-api-token" //nolint:gosec
)

type globals struct{}

type cli struct {
	globals

	Browse browseCmd `cmd:"" help:"Open a bookmark in a browser."`
	Fzf    fzfCmd    `cmd:"" help:"Search for bookmarks using fzf."`
	Login  loginCmd  `cmd:"" help:"Set the pinboard API token."`
	Search searchCmd `cmd:"" default:"withargs" help:"Search for bookmarks."`
	Show   showCmd   `cmd:"" help:"Show a bookmark."`
	Update updateCmd `cmd:"" help:"Update the local cache of bookmarks."`
}

func Main() {
	kr, err := keyring.Open(keyring.Config{
		ServiceName:             programName,
		LibSecretCollectionName: "login",
	})
	exitIfError(err)

	var cli cli
	ctx := kong.Parse(&cli,
		kong.Name(programName),
		kong.Description(`pingrep is a command-line tool for searching Pinboard bookmarks.`),
		kong.BindTo(kr, (*keyring.Keyring)(nil)),
	)

	err = ctx.Run(&cli.globals)
	exitIfError(err)
}
