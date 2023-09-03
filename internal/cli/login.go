package cli

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/99designs/keyring"
)

type loginCmd struct{}

func promptPassword() string {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter API key from https://pinboard.in/settings/password: ")
	password, _ := reader.ReadString('\n')
	return strings.TrimSpace(password)
}

func (cmd *loginCmd) Run(_ *globals, kr keyring.Keyring) error {
	password := promptPassword()
	err := kr.Set(keyring.Item{
		Key:  keyringAPITokenKeyName,
		Data: []byte(password),
	})
	return err
}
