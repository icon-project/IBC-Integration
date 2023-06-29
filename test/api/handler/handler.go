package handler

import (
	"context"
	"fmt"
	"net/http"
	"sync"
	"testing"
	"time"

	"github.com/icon-project/ibc-integration/test/api"
	"github.com/icon-project/ibc-integration/test/api/handler/utils"
	"github.com/icon-project/ibc-integration/test/chains"
	"golang.org/x/sync/errgroup"
)

const (
	RELAY_SETUP_PATH     = "/relayer-setup"
	RELAY_START_PATH     = "/relayer-start"
	RELAY_STOP_PATH      = "/relayer-stop"
	CHAIN_LINK           = "/link-chain"
	WALLET_BUILD         = "/build-wallet"
	IBC_SETUP            = "/setup-ibc"
	CONTRACT_ADDRESS_GET = "/contract-address-get/"
	EXECUTE_CALL         = "/execute-call"
)

var (
	server *handler
	once   sync.Once
)

type handler struct {
	ctx context.Context
	wg  *errgroup.Group
	api *api.Setup
}

func (h *handler) root(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello, World !"))
}

func (h *handler) serve(mux *http.ServeMux) {
	once.Do(func() {
		addr := fmt.Sprintf("%s:%s", chains.GetEnvOrDefault("HOST", "127.0.0.1"), chains.GetEnvOrDefault("PORT", "8080"))
		server := &http.Server{
			Addr:         addr,
			Handler:      mux,
			ReadTimeout:  10 * time.Second,
			WriteTimeout: 10 * time.Second,
		}

		h.wg.Go(func() error {
			fmt.Printf("Starting server on: %s\n", addr)
			if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
				return fmt.Errorf("failed to listen and serve: %w", err)
			}
			return nil
		})

		h.wg.Go(func() error {
			<-h.ctx.Done()
			ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
			defer cancel()

			if err := server.Shutdown(ctx); err != nil {
				return fmt.Errorf("failed to shutdown server gracefully: %s", err)
			}

			return nil
		})
	})
}

type setRelayRequest struct {
	Image string `json:"image"`
	Tag   string `json:"tag"`
	GID   string `json:"gid"`
}

func (h *handler) setupRelayer(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	var req setRelayRequest
	if err := utils.DecodeJSONBody(r, &req); err != nil {
		utils.HandleError(w, err, http.StatusBadRequest)
		return
	}
	fmt.Println("setupRelayer", req)
	image := chains.GetEnvOrDefault("RELAYER_IMAGE", req.Image)
	tag := chains.GetEnvOrDefault("RELAYER_IMAGE_TAG", req.Tag)
	gid := chains.GetEnvOrDefault("RELAYER_IMAGE_GID", req.GID)
	h.api.AddRelayer(image, tag, gid)
}

type LinkOption struct {
	ChainA string `json:"chainA"`
	ChainB string `json:"chainB"`
}

func (h *handler) linkChain(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	var req LinkOption
	if err := utils.DecodeJSONBody(r, &req); err != nil {
		utils.HandleError(w, err, http.StatusBadRequest)
		return
	}
	h.api.LinkChain(req.ChainA, req.ChainB)
}

type reqBuildWallet struct {
	KeyName   string `json:"keyName"`
	ChainName string `json:"chainName"`
}

func (h *handler) buildWallet(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowed", r.Method), http.StatusMethodNotAllowed)
		return
	}
	var req []reqBuildWallet
	if err := utils.DecodeJSONBody(r, &req); err != nil {
		utils.HandleError(w, err, http.StatusBadRequest)
		return
	}
	for _, v := range req {
		h.api.BuildWallet(v.ChainName, v.KeyName)
	}
}

type reqSetupIBC struct {
	ChainName string `json:"chainName"`
	KeyName   string `json:"keyName"`
}

func (h *handler) setupIBC(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	var req reqSetupIBC
	if err := utils.DecodeJSONBody(r, &req); err != nil {
		utils.HandleError(w, err, http.StatusBadRequest)
		return
	}
	h.api.SetupIBC(req.ChainName, req.KeyName)
}

func (h *handler) startRelayer(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	if err := h.api.StartRelayer(); err != nil {
		utils.HandleError(w, err, http.StatusInternalServerError)
	}
}

// returns contract adress
func (h *handler) getAddress(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	chainName := r.URL.Path[len("/get-address/"):]
	addr := h.api.GetContractAddress(chainName)
	utils.HandleSuccess(w, addr, http.StatusOK)
}

type reqExecuteCall struct {
	ChainName string `json:"chainName"`
	DstChain  string `json:"dstChain"`
	Message   []byte `json:"message"`
	KeyName   string `json:"user"`
	Rollback  []byte `json:"rollback"`
}

func (h *handler) executeCall(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	var req reqExecuteCall
	if err := utils.DecodeJSONBody(r, &req); err != nil {
		utils.HandleError(w, err, http.StatusBadRequest)
		return
	}
	h.api.ExecuteCall(req.ChainName, req.DstChain, req.KeyName, req.Message, req.Rollback)
}

func (h *handler) stopRelayer(w http.ResponseWriter, r *http.Request) {
	h.api.StopRelayer()
}

func (h *handler) StopRelayer() {
	h.api.StopRelayer()
}

func New(t *testing.T, cfg *api.OuterConfig, ctx context.Context, wg *errgroup.Group) *handler {
	if server != nil {
		return server
	}
	server = &handler{api: api.NewServer(t, cfg), ctx: ctx, wg: wg}
	mux := http.NewServeMux()
	mux.HandleFunc("/", server.root)
	mux.HandleFunc(RELAY_SETUP_PATH, server.setupRelayer)
	mux.HandleFunc(RELAY_START_PATH, server.startRelayer)
	mux.HandleFunc(RELAY_STOP_PATH, server.stopRelayer)
	mux.HandleFunc(CHAIN_LINK, server.linkChain)
	mux.HandleFunc(WALLET_BUILD, server.buildWallet)
	mux.HandleFunc(IBC_SETUP, server.setupIBC)
	mux.HandleFunc(CONTRACT_ADDRESS_GET, server.getAddress)
	mux.HandleFunc(EXECUTE_CALL, server.executeCall)
	server.serve(mux)
	return server
}
