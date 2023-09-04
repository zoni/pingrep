package cli

import (
	"fmt"
	"time"

	"github.com/99designs/keyring"
	"github.com/adrg/xdg"
	"github.com/zoni/pingrep/internal/pinboard"
)

type updateCmd struct {
	Verbose bool `short:"v" help:"Show verbose output."`
}

func (cmd *updateCmd) Run(_ *globals, kr keyring.Keyring) error {
	path, err := xdg.DataFile(fmt.Sprintf("%s/bookmarks.json", programName))
	if err != nil {
		return fmt.Errorf("create data dir: %w", err)
	}
	client, err := createPinboardClient(kr)
	if err != nil {
		return fmt.Errorf("construct pinboard client: %w", err)
	}

	localTime, err := getLastUpdateTimestamp(path)
	if err != nil {
		return fmt.Errorf("get last update timestamp: %w", err)
	}
	remoteTime, err := client.LastUpdate()
	if err != nil {
		return fmt.Errorf("get last update timestamp from api: %w", err)
	}

	if !localTime.Before(remoteTime) {
		if cmd.Verbose {
			fmt.Printf("Bookmarks already up to date (last changed %s)\n", remoteTime.Format(time.RFC3339))
		}
		return nil
	}

	collection, err := pinboard.FromAPI(client)
	if err != nil {
		return fmt.Errorf("read pins from api: %w", err)
	}
	if err := collection.Save(path); err != nil {
		return fmt.Errorf("save new bookmarks: %w", err)
	}
	fmt.Printf("Bookmarks updated (last changed %s)\n", remoteTime.Format(time.RFC3339))
	return nil
}
