package trades

import "net/http"

type Implementation struct {
}

func New() *Implementation {
	return &Implementation{}
}

func (i *Implementation) Hello(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("welcome"))
}
