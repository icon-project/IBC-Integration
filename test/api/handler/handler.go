package handler

import (
	"net/http"
	"testing"

	"github.com/icon-project/ibc-integration/test/api"
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

func (h *handler) setupRelay(w http.ResponseWriter, r *http.Request) error {

}

func New(t *testing.T) *handler {
	handler := &handler{api: api.NewServer(t)}
	mux := http.NewServeMux()
	mux.HandleFunc("/", handler.root)
	handler.mux = mux
	return handler
}
