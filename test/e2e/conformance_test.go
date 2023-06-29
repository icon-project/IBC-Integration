package e2e_test

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"testing"

	"github.com/icon-project/ibc-integration/test/api"
	"github.com/icon-project/ibc-integration/test/api/handler"
	"github.com/icon-project/ibc-integration/test/api/handler/utils"
	"github.com/stretchr/testify/require"
	"golang.org/x/sync/errgroup"
)

func TestConformance(t *testing.T) {
	fmt.Println("test start")
	cfg, err := api.GetConfig()
	require.NoError(t, err)
	ctx, cancel := context.WithCancel(context.Background())
	go func() {
		c := make(chan os.Signal, syscall.SIGTERM)
		signal.Notify(c, os.Interrupt, syscall.SIGTERM)
		<-c
		cancel()
	}()
	wg, ctx := errgroup.WithContext(ctx)
	h := handler.New(t, cfg, ctx, wg)

	// Create the request body
	body := map[string]string{
		"image": "relayer",
		"tag":   "latest",
		"gid":   "1000:1000",
	}

	// Send the request
	resp, err := utils.Request(http.MethodPost, handler.RELAY_SETUP_PATH, body)
	if err != nil {
		t.Errorf("The HTTP request failed with error %s\n", err)
	}
	t.Logf("Relay setup succeed with following response: %v", resp)

	if err := wg.Wait(); err != nil {
		t.Errorf("failed to wait for server to stop: %v", err)
	}
	h.StopRelayer()
}
