package trades

import (
	"net/http"

	"github.com/asiman161/trades/internal/storage"
)

type Implementation struct {
	store storage.Storager
}

func New(store storage.Storager) *Implementation {
	return &Implementation{store: store}
}

func (i *Implementation) Hello(w http.ResponseWriter, _ *http.Request) {
	_, _ = w.Write([]byte("welcome"))
}
