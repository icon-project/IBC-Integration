package api

import "net/http"

func main() {
	http.ListenAndServe(":8080", nil)
}
