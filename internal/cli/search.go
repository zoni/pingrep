package cli

import (
	"errors"
)

type searchCmd struct {
	Query []string `arg:"" help:"Search query."`
}

func (cmd *searchCmd) Run(_ *globals) error {
	return errors.New("not implemented")
}
