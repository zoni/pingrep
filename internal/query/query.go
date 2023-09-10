package query

import (
	p "github.com/ajitid/goparsify"
)

const (
	// Only include bookmarks whose title or description contains the given phrase.
	OpIncludePhrase = iota
	// Exclude any bookmarks whose title or description contains the given phrase.
	OpExcludePhrase
	// Only include bookmarks with the given tag.
	OpIncludeTag
	// Exclude any bookmarks with the given tag.
	OpExcludeTag
)

// FilterOp defines a single filter operation.
type FilterOp struct {
	// Op is the operation to perform, as defined by the `Op*` constants.
	Op uint8
	// Value is the value to filter on.
	Value string
}

var (
	_word         = p.Chars("a-zA-Z0-9").Map(func(r *p.Result) { r.Result = r.Token })
	_quotedString = p.StringLit("\"'")
	_tag          = p.Seq("#", p.Chars("a-zA-Z0-9")).Map(func(r *p.Result) {
		r.Result = r.Child[1].Token
	})

	_tagInclude = _tag
	_tagExclude = p.Seq("!", _tag).Map(func(r *p.Result) {
		r.Result = r.Child[1].Result
	})

	_phraseInclude = p.Any(_word, _quotedString)
	_phraseExclude = p.Seq("!", p.Any(_word, _quotedString)).Map(func(r *p.Result) {
		r.Result = r.Child[1].Token
	})

	_query = p.OneOrMore(
		p.Any(
			_tagExclude.Map(func(r *p.Result) {
				r.Result = FilterOp{Op: OpExcludeTag, Value: r.Result.(string)}
			}),
			_tagInclude.Map(func(r *p.Result) {
				r.Result = FilterOp{Op: OpIncludeTag, Value: r.Result.(string)}
			}),
			_phraseExclude.Map(func(r *p.Result) {
				r.Result = FilterOp{Op: OpExcludePhrase, Value: r.Result.(string)}
			}),
			_phraseInclude.Map(func(r *p.Result) {
				r.Result = FilterOp{Op: OpIncludePhrase, Value: r.Token}
			}),
		),
	).Map(func(r *p.Result) {
		ret := []interface{}{}
		for _, child := range r.Child {
			ret = append(ret, child.Result)
		}
		r.Result = ret
	})
)

// Parse parses the given query string into a list of filter operations.
func Parse(query string) ([]FilterOp, error) {
	result, err := p.Run(_query, query)
	if err != nil {
		return nil, err
	}
	typed := make([]FilterOp, len(result.([]interface{})))
	for i, v := range result.([]interface{}) {
		typed[i] = v.(FilterOp)
	}
	return typed, nil
}
