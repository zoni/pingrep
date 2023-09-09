# Pingrep

`pingrep` is a command-line tool designed to quickly and easily search through your [Pinboard] bookmarks locally.

## Installation

I don't provide binaries so you must build pingrep from source.
[Ensure you have Go installed on your system][install-go], then install pingrep with:

```sh
go install github.com/zoni/pingrep@latest
```

To upgrade an older version, simply run the same command again.

> [!IMPORTANT]
> Executables are installed in the directory named by the `GOBIN` environment variable, which defaults to `$GOPATH/bin` or `$HOME/go/bin` if the `GOPATH` environment variable is not set.
>
> Make sure your shell has this directory listed in the `$PATH` environment variable.

## Usage

pingrep provides several commands to help you manage and search your Pinboard bookmarks.
Here's a brief overview of the most important commands:

### `pingrep login`

Before using pingrep, you need to configure it with your Pinboard API token using:

```sh
pingrep login
```

Pingrep will securely store this token in the system keyring.

### `pingrep fzf`

This command opens an interactive search interface using [`fzf`][fzf], allowing you to search and filter your Pinboard bookmarks easily.

#### Keybindings

- `enter`: Open the selected bookmark in a browser, then exit.
- `ctrl+o`: Open the selected bookmark in a browser, don't exit.
- `ctrl+e`: Open the selected bookmark on [pinboard.in][pinboard].
- `ctrl-y`: Copy the URL of the selected bookmark to the clipboard (requires [cbcopy]).

### `pingrep search <query> ...`

Search for bookmarks using one or more search queries.
Replace `<query>` with your search terms.
You can use multiple queries to narrow down your search.

### `pingrep show <url>`

View detailed information about a specific bookmark by providing its URL as an argument.

### `pingrep update`

Update the local cache of your Pinboard bookmarks to ensure you have the latest data.

Linux users may want to refer to these examples to automate this using a systemd timer:

- [pingrep-update.service]
- [pingrep-update.timer]

### Additional Information

For more information and detailed usage instructions, refer to the built-in help documentation:

```sh
pingrep --help
```

## License

Pingrep is open-source software released under the [BSD-2-Clause Plus Patent License].
This license is designed to provide: a) a simple permissive license; b) that is compatible with the GNU General Public License (GPL), version 2; and c) which also has an express patent grant included.

Please review the [LICENSE] file for the full text of the license.

[BSD-2-Clause Plus Patent License]: https://spdx.org/licenses/BSD-2-Clause-Patent.html
[LICENSE]: LICENSE
[cbcopy]: https://github.com/zoni/dotfiles/blob/f724b16f85649786d393119500033455f21b42ab/src/.local/bin/cbcopy
[install-go]: https://go.dev/doc/install
[fzf]: https://github.com/junegunn/fzf
[pinboard]: https://pinboard.in
[pingrep-update.service]: https://github.com/zoni/dotfiles/blob/main/src/.config/systemd/user/pingrep-update.service
[pingrep-update.timer]: https://github.com/zoni/dotfiles/blob/main/src/.config/systemd/user/pingrep-update.timer
