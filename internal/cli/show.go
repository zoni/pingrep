package cli

import (
	"fmt"
	"os"
	"strings"
	"text/template"
)

func displayTemplate() *template.Template {
	return template.Must(
		template.New("display").Funcs(templateFuncs).Parse(
			strings.TrimSpace(`
{{ .bookmark.URL }}

{{ .bookmark.Title }}
{{ .bookmark.Description }}

---

Tags: {{ .bookmark.Tags | formatTags false }}
Saved at: {{ .bookmark.SavedAt }}
Context: https://pinboard.in/u:{{ .user }}/before:{{ .bookmark.SavedAt.Unix }}
`) + "\n"))
}

type showCmd struct {
	URL string `arg:"" help:"The URL of the bookmark to show."`
}

func (cmd *showCmd) Run(_ *globals) error {
	collection, err := loadCollection()
	if err != nil {
		return err
	}
	for _, bookmark := range collection.Bookmarks {
		if bookmark.URL == cmd.URL {
			return displayTemplate().Execute(
				os.Stdout,
				map[string]interface{}{
					"bookmark": bookmark,
					"user":     collection.User,
				})
		}
	}
	return fmt.Errorf("no bookmark found with URL %q", cmd.URL)
}
