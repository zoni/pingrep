package query_test

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/zoni/pingrep/internal/query"
)

func TestParse(t *testing.T) {
	tests := []struct {
		query   string
		expect  []query.FilterOp
		wantErr bool
	}{
		{
			query: "foo",
			expect: []query.FilterOp{
				{Op: query.OpIncludePhrase, Value: "foo"},
			},
			wantErr: false,
		},
		{
			query: "foo bar",
			expect: []query.FilterOp{
				{Op: query.OpIncludePhrase, Value: "foo"},
				{Op: query.OpIncludePhrase, Value: "bar"},
			},
			wantErr: false,
		},
		{
			query: "'foo bar'",
			expect: []query.FilterOp{
				{Op: query.OpIncludePhrase, Value: "foo bar"},
			},
			wantErr: false,
		},
		{
			query: "foo #bar",
			expect: []query.FilterOp{
				{Op: query.OpIncludePhrase, Value: "foo"},
				{Op: query.OpIncludeTag, Value: "bar"},
			},
			wantErr: false,
		},
		{
			query: "!foo",
			expect: []query.FilterOp{
				{Op: query.OpExcludePhrase, Value: "foo"},
			},
			wantErr: false,
		},
		{
			query: "!#bar",
			expect: []query.FilterOp{
				{Op: query.OpExcludeTag, Value: "bar"},
			},
			wantErr: false,
		},
		{
			query: "!'no foo' !#bar",
			expect: []query.FilterOp{
				{Op: query.OpExcludePhrase, Value: "no foo"},
				{Op: query.OpExcludeTag, Value: "bar"},
			},
			wantErr: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.query, func(t *testing.T) {
			result, err := query.Parse(tt.query)
			if (err != nil) != tt.wantErr {
				t.Fatalf("Parse() error = %v, wantErr %v", err, tt.wantErr)
			}
			if diff := cmp.Diff(tt.expect, result); diff != "" {
				t.Errorf("Parse() mismatch (-want +got):\n%s", diff)
			}
		})
	}
}
