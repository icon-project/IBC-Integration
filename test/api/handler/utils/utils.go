package utils

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
)

const (
	baseUrl     = "http://localhost:8080"
	contentType = "application/json"
)

var (
	client = new(http.Client)
)

func DecodeJSONBody(r *http.Request, v interface{}) error {
	defer r.Body.Close()
	return json.NewDecoder(r.Body).Decode(v)
}

func JSONResponse(v interface{}, w http.ResponseWriter) error {
	body, err := json.Marshal(v)
	if err != nil {
		return err
	}
	w.Header().Set("Content-Type", contentType)
	w.WriteHeader(http.StatusOK)
	w.Write(body)
	return nil
}

func HandleError(w http.ResponseWriter, err error, statusCode int) {
	w.WriteHeader(statusCode)
	w.Write([]byte(err.Error()))
}

func HandleSuccess(w http.ResponseWriter, msg []byte, statusCode int) {
	w.WriteHeader(statusCode)
	w.Write([]byte(msg))
}

func Request(method, path string, body interface{}) ([]byte, error) {
	var (
		req *http.Request
		err error
		url = fmt.Sprintf("%s/%s", baseUrl, strings.TrimPrefix(path, "/"))
	)

	if method == http.MethodGet {
		req, err = http.NewRequest(method, url, nil)
		if err != nil {
			return nil, err
		}
	} else {
		jsonBody, err := json.Marshal(body)
		if err != nil {
			return nil, err
		}

		req, err = http.NewRequest(method, url, bytes.NewBuffer(jsonBody))
		if err != nil {
			return nil, err
		}

		req.Header.Set("Content-Type", contentType)
	}

	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	return bodyBytes, nil
}
