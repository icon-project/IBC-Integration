package handler

import (
	"fmt"
	"net/http"
	"sync"
	"testing"

	"github.com/icon-project/ibc-integration/test/api"
	"github.com/icon-project/ibc-integration/test/api/handler/utils"
	"github.com/icon-project/ibc-integration/test/chains"
)

var (
	server *handler
	once   sync.Once
)

type handler struct {
	api *api.Server
	mux *http.ServeMux
}

func (h *handler) root(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello, World!"))
}

func (h *handler) ServeHTTP() func() {
	return func() {
		once.Do(func() {
			h.api.Serve(h.mux)
		})
	}
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

func (h *handler) SetupIBC(w http.ResponseWriter, r *http.Request) {
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

func (h *handler) StartRelay(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		utils.HandleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	h.api.StartRelay()
}

func New(t *testing.T) *handler {
	if server != nil {
		return server
	}
	server = &handler{api: api.NewServer(t)}
	mux := http.NewServeMux()
	mux.HandleFunc("/", server.root)
	mux.HandleFunc("/setup-relay", server.setupRelay)
	mux.HandleFunc("/link-chain", server.linkChain)
	mux.HandleFunc("/build-wallet", server.buildWallet)
	mux.HandleFunc("/setup-ibc", server.SetupIBC)
	mux.HandleFunc("/start-relay", server.StartRelay)
	server.mux = mux
	return server
}
