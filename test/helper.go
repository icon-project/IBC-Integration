package interchaintest

import (
	"fmt"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"os"
	"path/filepath"
)

type Contract struct {
	Name    string `json:"name"`
	Address string `json:"address"`
}

type IBCContracts []Contract
type XCallContracts []Contract

var ibcContracts = filepath.Join(os.Getenv("HOME"), "ibcContracts-%s.json")

// for saving data in particular format
func BackupConfig(chain chains.Chain) error {
	addresses, err := chain.BackupConfig()
	if err != nil {
		return err
	}
	fileName := fmt.Sprintf(ibcContracts, chain.(ibc.Chain).Config().ChainID)
	file, err := os.Create(fileName)
	if err != nil {
		fmt.Println("Error creating file:", err)
		return err
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			fmt.Println("Error closing file:", err)
		}
	}(file)

	_, err = file.Write(addresses)
	return err
}

func RestoreConfig(chain chains.Chain) error {
	fileName := fmt.Sprintf(ibcContracts, chain.(ibc.Chain).Config().ChainID)
	file, err := os.Open(fileName)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return err
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			fmt.Println("Error closing file:", err)
		}
	}(file)

	fileInfo, err := file.Stat()
	if err != nil {
		fmt.Println("Error getting file info:", err)
		return err
	}
	fileSize := fileInfo.Size()

	// Read the file content into a buffer
	buffer := make([]byte, fileSize)
	_, err = file.Read(buffer)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return err
	}
	err = chain.RestoreConfig(buffer)
	return err
}
