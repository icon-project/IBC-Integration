package interchaintest

import (
	"fmt"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"os"
	"path/filepath"
)

var ibcConfigPath = filepath.Join(os.Getenv(chains.BASE_PATH), "ibc-config")

func CleanBackupConfig() {
	files, err := filepath.Glob(filepath.Join(ibcConfigPath, "*.json"))
	if err != nil {
		fmt.Println("Error deleting file:", err)
		return
	}

	for _, file := range files {
		err := os.Remove(file)
		if err != nil {
			fmt.Println("Error deleting file:", err)
		}
	}

}

// for saving data in particular format
func BackupConfig(chain chains.Chain) error {
	config, err := chain.BackupConfig()
	if err != nil {
		return err
	}
	fileName := fmt.Sprintf("%s/%s.json", ibcConfigPath, chain.(ibc.Chain).Config().ChainID)
	dirPath := filepath.Dir(fileName)

	err = os.MkdirAll(dirPath, os.ModePerm)
	if err != nil {
		return err
	}
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

	_, err = file.Write(config)
	return err
}

func RestoreConfig(chain chains.Chain) error {
	fileName := fmt.Sprintf("%s/%s.json", ibcConfigPath, chain.(ibc.Chain).Config().ChainID)
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
