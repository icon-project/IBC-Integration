package e2e_test

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"testing"

	"github.com/icon-project/ibc-integration/test/api/handler"
	"golang.org/x/sync/errgroup"
)

const (
	serverUrl = "http://localhost:8080"
)

func TestConformance(t *testing.T) {
	fmt.Println("test start")
	ctx, cancel := context.WithCancel(context.Background())
	go func() {
		c := make(chan os.Signal, syscall.SIGTERM)
		signal.Notify(c, os.Interrupt, syscall.SIGTERM)
		<-c
		cancel()
	}()
	wg, gCtx := errgroup.WithContext(ctx)
	h := handler.New(t, gCtx, wg)

	// Create the request body
	body := map[string]string{
		"image": "relay",
		"tag":   "latest",
		"gid":   "1000:1000",
	}
	jsonBody, err := json.Marshal(body)
	if err != nil {
		t.Fatalf("failed to marshal request body: %v", err)
	}

	// Create the HTTP request with the JSON body
	req, err := http.NewRequest(http.MethodPost, fmt.Sprintf("%s/setup-relay", serverUrl), bytes.NewBuffer(jsonBody))
	if err != nil {
		t.Fatalf("failed to create HTTP request: %v", err)
	}

	// Send the HTTP request
	if _, err := http.DefaultClient.Do(req); err != nil {
		t.Fatalf("failed to send HTTP request: %v", err)
	}

	if err := wg.Wait(); err != nil {
		h.StopRelayer()
	}
}
