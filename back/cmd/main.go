package main

import (
	"log"

	"github.com/adshao/go-binance/v2"
	"github.com/asiman161/trades/internal/app/trades"
	"github.com/asiman161/trades/internal/bootstrap"
	"github.com/asiman161/trades/internal/storage"
	"github.com/caarlos0/env/v6"
)

func main() {
	cfg := bootstrap.AppConfig{}

	if err := env.Parse(&cfg); err != nil {
		log.Fatal(err)
	}

	app := initApp(cfg)

	server := bootstrap.SetupServer(app, cfg)
	bootstrap.StartServer(server)
}

func initApp(cfg bootstrap.AppConfig) *trades.Implementation {
	db, err := bootstrap.InitStorage(cfg.DatabaseDSN)
	if err != nil {
		log.Fatal(err)
	}

	bnsClient := binance.NewClient("apiKey", "secretKey")

	store := storage.New(db)
	return trades.New(store, bnsClient)
}
