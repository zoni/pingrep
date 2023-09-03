package pinboard

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"time"
)

// Collection is a collection of bookmarks.
type Collection struct {
	User       string     `json:"user"`
	LastUpdate time.Time  `json:"lastUpdate"`
	Bookmarks  []Bookmark `json:"bookmarks"`
}

// Bookmark is a bookmark as stored on pinboard, but with some fields renamed
// for clarity.
type Bookmark struct {
	URL         string
	Title       string
	Description string
	Tags        []string
	SavedAt     time.Time
	Meta        string
	Hash        string
}

// Write writes the collection to the given writer as JSON.
func (c *Collection) Write(w io.Writer) error {
	encoder := json.NewEncoder(bufio.NewWriter(w))
	return encoder.Encode(c)
}

// Save saves the collection to the given path as JSON.
func (c *Collection) Save(path string) error {
	f, err := os.Create(path)
	if err != nil {
		return fmt.Errorf("creating file: %w", err)
	}
	defer f.Close()
	return c.Write(f)
}

// Empty returns an empty collection.
func Empty() *Collection {
	return &Collection{Bookmarks: make([]Bookmark, 0)}
}

// FromReader reads a collection from the given reader.
func FromReader(r io.Reader) (*Collection, error) {
	var c Collection
	decoder := json.NewDecoder(bufio.NewReader(r))
	for {
		err := decoder.Decode(&c)
		if err != nil {
			if err == io.EOF { //nolint:errorlint
				break
			}
			return nil, fmt.Errorf("json decode error: %w", err)
		}
	}
	return &c, nil
}

// FromFile reads a collection from the given file.
func FromFile(path string) (*Collection, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("open file: %w", err)
	}
	defer f.Close()
	return FromReader(f)
}

// FromAPI creates a collection by loading it from pinboard.in using the given
// client.
func FromAPI(client *Client) (*Collection, error) {
	lastupdate, err := client.LastUpdate()
	if err != nil {
		return nil, fmt.Errorf("fetch last update timestamp: %w", err)
	}

	bookmarks, err := client.Bookmarks()
	if err != nil {
		return nil, fmt.Errorf("fetch bookmarks: %w", err)
	}

	return &Collection{
		LastUpdate: lastupdate,
		Bookmarks:  bookmarks,
		User:       client.User,
	}, nil
}
