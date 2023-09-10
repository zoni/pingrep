package pinboard

import (
	"github.com/zoni/pingrep/internal/query"
)

func (c *Collection) Filter(filter []query.FilterOp) *Collection {
	return c
}
