package pinboard

import (
	"fmt"
	"strings"
	"time"

	pb "github.com/zoni/go-pinboard"
)

// Client is a Pinboard client.
//
// It wraps https://github.com/zoni/go-pinboard internally.
type Client struct {
	client *pb.Pinboard
	User   string
}

// NewClient creates a new Pinboard client with the given auth token.
func NewClient(token string) (*Client, error) {
	var user string
	_, err := fmt.Sscanf(strings.Replace(token, ":", " ", 1), "%s %s", &user, &token)
	if err != nil {
		return nil, fmt.Errorf("invalid token format: %w", err)
	}

	return &Client{
		client: &pb.Pinboard{User: user, Token: token},
		User:   user,
	}, nil
}

// Bookmarks returns all bookmarks.
func (c *Client) Bookmarks() ([]Bookmark, error) {
	posts, err := c.client.PostsAll(pb.PostsAllFilter{})
	if err != nil {
		return nil, err
	}
	bookmarks := make([]Bookmark, 0, len(posts))
	for _, p := range posts {
		bookmarks = append(bookmarks, Bookmark{
			URL:         p.Url,
			Title:       p.Description,
			Description: p.Extended,
			Tags:        p.Tags,
			SavedAt:     p.Date,
			Meta:        p.Meta,
			Hash:        p.Hash,
		})
	}
	return bookmarks, nil
}

// LastUpdate returns the time of the last update.
func (c *Client) LastUpdate() (time.Time, error) {
	return c.client.PostsUpdated()
}
