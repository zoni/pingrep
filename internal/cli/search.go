package cli

import (
	"strings"

	"github.com/kr/pretty"
	"github.com/zoni/pingrep/internal/query"
)

type searchCmd struct {
	Query []string `arg:"" help:"Search query."`
}

func (cmd *searchCmd) Run(_ *globals) error {
	result, err := query.Parse(strings.Join(cmd.Query, " "))
	if err != nil {
		return err
	}
	pretty.Print(result)
	return nil
}
