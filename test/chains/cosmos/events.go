package cosmos

import (
	"context"
	"errors"
	"fmt"
	rpchttp "github.com/cometbft/cometbft/rpc/client/http"
	"github.com/icon-project/ibc-integration/test/chains"
	"golang.org/x/exp/slices"
	"net/http"
	"strings"
	"sync"
	"time"
)

type WebSocketListener struct {
	contract     string
	chain        *CosmosLocalnet
	Shutdown     chan struct{}
	Events       []map[string][]string
	eventMapLock sync.RWMutex
	wg           sync.WaitGroup
	timeout      time.Duration
}

func NewCosmosEventListener(c *CosmosLocalnet, contract string, timeout time.Duration) *WebSocketListener {
	return &WebSocketListener{
		chain:    c,
		timeout:  timeout,
		contract: c.GetIBCAddress(contract),
		Shutdown: make(chan struct{}),
		Events:   []map[string][]string{},
	}
}

func (w *WebSocketListener) Start() {

	fmt.Println("Starting to listen on...")

	w.wg.Add(1)
	go func() {
		defer w.wg.Done()
		if err := w.handleWebSocket(); err != nil && err != http.ErrServerClosed {
			fmt.Printf("Error starting server: %v\n", err)
		}
	}()
}

func (w *WebSocketListener) Stop() {
	fmt.Println("Stopping...")
	close(w.Shutdown)

	select {
	case <-time.After(5 * time.Second):
		fmt.Println("Shutdown took too long, force closing...")
	}

	fmt.Println("Stopped.")
}

func (w *WebSocketListener) handleWebSocket() error {
	client, err := rpchttp.New(w.chain.GetHostRPCAddress(), "/websocket")
	if err != nil {
		return err
	}

	err = client.Start()
	if err != nil {
		return err
	}
	defer func(client *rpchttp.HTTP) {
		err := client.Stop()
		if err != nil {

		}
	}(client)
	ctx, cancel := context.WithCancel(context.Background())

	query := strings.Join([]string{"tm.event = 'Tx'",
		//fmt.Sprintf("tx.height >= %d ", height),
		fmt.Sprintf("message.module = 'wasm'"),
		fmt.Sprintf("wasm._contract_address = '%s'", w.contract),
		//fmt.Sprintf(index),
	}, " AND ")
	channel, err := client.Subscribe(ctx, "wasm-client", query)
	if err != nil {
		cancel()
		return err
	}

	fmt.Println("WebSocket connected!")

	for {
		select {
		case event := <-channel:

			w.eventMapLock.Lock()
			//e, _ := json.Marshal(event.Events)
			w.Events = append(w.Events, event.Events)
			w.eventMapLock.Unlock()

		case <-w.Shutdown:
			cancel()
			fmt.Printf("Shutting down...\n")
			return nil
		}
	}
}

func (w *WebSocketListener) FindEvent(filters chains.Filter) (chains.Event, error) {
	timer := time.NewTimer(w.timeout)
	defer timer.Stop()

	for {
		select {
		case <-timer.C:
			return nil, errors.New("event not found") // Event doesn't exist within the timeout
		default:
			if exist, event := w.eventExists(filters); exist {
				fmt.Printf("event---%v \n", event)
				return event, nil // Event exists
			}
		}
	}
}

func (w *WebSocketListener) eventExists(filters chains.Filter) (bool, chains.Event) {
	w.eventMapLock.RLock()
	defer w.eventMapLock.RUnlock()

	for _, event := range w.Events {

		if matchesFilters(event, filters) {
			return true, event
		}
	}

	return false, nil
}

func matchesFilters(event chains.Event, filters chains.Filter) bool {
	for key, value := range filters {
		mValue, found := event[key]

		if !found {
			return false
		}

		if !slices.Contains(mValue, value.(string)) {
			return false
		}
	}
	fmt.Println("------------FOUND--------------")
	return true
}
