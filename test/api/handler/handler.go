package handler

import (
	"encoding/json"
	"fmt"
	"net/http"
	"testing"

	"github.com/icon-project/ibc-integration/test/api"
	"github.com/icon-project/ibc-integration/test/chains"
)

type handler struct {
	api *api.Server
	mux *http.ServeMux
}

type IHandler interface {
	SetupIBC() error
	SetupRelay(string, string, string)
	StartRelay()
}

func (h *handler) root(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello, World!"))
}

func (h *handler) Run() func() error {
	return h.api.Serve(h.mux)
}

type setRelayRequest struct {
	Image string `json:"image"`
	Tag   string `json:"tag"`
	GID   string `json:"gid"`
}

func (h *handler) setupRelay(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		handleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	var req setRelayRequest
	if err := decodeJSONBody(r, &req); err != nil {
		handleError(w, err, http.StatusBadRequest)
		return
	}
	image := chains.GetEnvOrDefault("RELAYER_IMAGE", req.Image)
	tag := chains.GetEnvOrDefault("RELAYER_IMAGE_TAG", req.Tag)
	gid := chains.GetEnvOrDefault("RELAYER_IMAGE_GID", req.GID)
	h.api.SetupRelayer(image, tag, gid)
}

type LinkOption struct {
	ChainA string `json:"chainA"`
	ChainB string `json:"chainB"`
}

func (h *handler) linkChain(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		handleError(w, fmt.Errorf("method %s not allowd", r.Method), http.StatusMethodNotAllowed)
		return
	}
	var req LinkOption
	if err := decodeJSONBody(r, &req); err != nil {
		handleError(w, err, http.StatusBadRequest)
		return
	}
	h.api.LinkChain(req.ChainA, req.ChainB)
}

func decodeJSONBody(r *http.Request, v interface{}) error {
	defer r.Body.Close()
	return json.NewDecoder(r.Body).Decode(v)
}

func handleError(w http.ResponseWriter, err error, statusCode int) {
	w.WriteHeader(statusCode)
	w.Write([]byte(err.Error()))
}

func New(t *testing.T) *handler {
	handler := &handler{api: api.NewServer(t)}
	mux := http.NewServeMux()
	mux.HandleFunc("/", handler.root)
	mux.HandleFunc("/setup-relay", handler.setupRelay)
	mux.HandleFunc("/link-chain", handler.linkChain)
	handler.mux = mux
	return handler
}
