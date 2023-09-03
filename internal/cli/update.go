package cli

import (
	"fmt"

	"github.com/99designs/keyring"
	"github.com/adrg/xdg"
	"github.com/zoni/pingrep/internal/pinboard"
)

type updateCmd struct{}

func (cmd *updateCmd) Run(_ *globals, kr keyring.Keyring) error {
	path, err := xdg.DataFile(fmt.Sprintf("%s/bookmarks.json", programName))
	if err != nil {
		return fmt.Errorf("create data dir: %w", err)
	}
	token, err := getPinboardToken(kr)
	if err != nil {
		return err
	}
	client, err := pinboard.NewClient(token)
	if err != nil {
		return fmt.Errorf("construct pinboard client: %w", err)
	}
	collection, err := pinboard.FromAPI(client)
	if err != nil {
		return fmt.Errorf("read pins from api: %w", err)
	}
	return collection.Save(path)
}
