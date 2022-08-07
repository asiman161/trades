package trades

import (
	"github.com/adshao/go-binance/v2"
	"github.com/asiman161/trades/internal/storage"
)

type Implementation struct {
	bnsClient *binance.Client
	store     storage.Storager
}

func New(store storage.Storager, bnsClient *binance.Client) *Implementation {
	return &Implementation{store: store, bnsClient: bnsClient}
}
