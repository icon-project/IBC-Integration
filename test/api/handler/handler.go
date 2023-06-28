package handler

import (
	"context"
	"fmt"
	"net/http"
	"sync"
	"testing"

	"github.com/icon-project/ibc-integration/test/api"
	"github.com/icon-project/ibc-integration/test/api/handler/utils"
	"github.com/icon-project/ibc-integration/test/chains"
	"golang.org/x/sync/errgroup"
)

var (
	server *handler
	once   sync.Once
)

type handler struct {
	ctx context.Context
	wg  *errgroup.Group
	api *api.Server
	mux *http.ServeMux
}

func (h *handler) root(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello, World!"))
}

func (h *handler) serve() {
	once.Do(func() {
		h.api.Serve(h.ctx, h.wg, h.mux)
	})
}

type setRelayRequest struct {
	Image string `json:"image"`
	Tag   string `json:"tag"`
	GID   string `json:"gid"`
}

func (h *handler) setupRelay(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	var req setRelayRequest
	if err := utils.DecodeJSONBody(r, &req); err != nil {
		utils.HandleError(w, err, http.StatusBadRequest)
		return
	}
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
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
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

func (h *handler) startRelay(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	if err := h.api.StartRelay(); err != nil {
		utils.HandleError(w, err, http.StatusInternalServerError)
		return
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

func (h *handler) execteCall(w http.ResponseWriter, r *http.Request) {
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

func (h *handler) StopRelayer() {
	h.api.StopRelayer()
}

func New(t *testing.T, ctx context.Context, wg *errgroup.Group) *handler {
	if server != nil {
		return server
	}
	server = &handler{api: api.NewServer(t), ctx: ctx, wg: wg}
	mux := http.NewServeMux()
	mux.HandleFunc("/", server.root)
	mux.HandleFunc("/setup-relay", server.setupRelay)
	mux.HandleFunc("/link-chain", server.linkChain)
	mux.HandleFunc("/build-wallet", server.buildWallet)
	mux.HandleFunc("/setup-ibc", server.setupIBC)
	mux.HandleFunc("/start-relay", server.startRelay)
	mux.HandleFunc("/get-address/", server.getAddress)
	mux.HandleFunc("/execute-call", server.execteCall)
	server.mux = mux
	server.serve()
	return server
}
