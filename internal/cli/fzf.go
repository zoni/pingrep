package cli

import (
	"fmt"
	"os"
	"os/exec"
	"strings"
	"text/template"
)

const fzfFieldSeparator = "\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t"

type fzfCmd struct{}

func fzfTemplate() *template.Template {
	return template.Must(
		template.New("fzf").Funcs(templateFuncs).Parse(
			strings.TrimSpace(`
{{ range .Collection.Bookmarks -}}
{{ .Title | oneline -}}
{{ $.Separator -}}
{{ .URL | oneline -}}
{{ $.Separator -}}
{{ .Description | oneline -}}
{{ $.Separator -}}
{{ .Tags | formatTags true -}}
{{ $.Separator -}}
https://pinboard.in/u:{{ $.Collection.User }}/before:{{ .SavedAt.Unix }}
{{ end }}
`)))
}

func (cmd *fzfCmd) Run(_ *globals) error {
	collection, err := loadCollection()
	if err != nil {
		return err
	}

	// fzf doesn't allow searching through hidden fields, so a hack is needed
	// with creating really long lines and setting ellipsis to an empty string.
	//
	// References:
	//
	// - https://github.com/junegunn/fzf/commit/ef67a45702c01ff93e0ea99a51594c8160f66cc1
	// - https://github.com/junegunn/fzf/issues/2432
	fzf := exec.Command("fzf", //nolint:gosec
		"--delimiter", fzfFieldSeparator,
		"--ellipsis", "",
		"--no-hscroll",
		"--preview", fmt.Sprintf("%s show {2}", os.Args[0]),
		"--bind", "ctrl-y:execute-silent(echo {2} | cbcopy)",
		"--bind", fmt.Sprintf("enter:become(%s browse {2})", os.Args[0]),
		"--bind", fmt.Sprintf("ctrl-o:execute-silent(%s browse {2})", os.Args[0]),
		"--bind", fmt.Sprintf("ctrl-e:execute-silent(%s browse {5})", os.Args[0]),
	)
	fzf.Stdout = os.Stdout
	fzf.Stderr = os.Stderr
	stdin, err := fzf.StdinPipe()
	if err != nil {
		return fmt.Errorf("connect stdin for fzf: %w", err)
	}
	go func() {
		defer stdin.Close()
		err := fzfTemplate().Execute(
			stdin,
			map[string]interface{}{
				"Collection": collection,
				"Separator":  fzfFieldSeparator,
			})
		if err != nil {
			fmt.Fprintf(os.Stderr, "execute template: %v\n", err)
		}
	}()
	return fzf.Run()
}
