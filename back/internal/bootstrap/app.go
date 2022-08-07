package bootstrap

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/asiman161/trades/internal/app/trades"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

type AppConfig struct {
	DatabaseDSN string `env:"DATABASE_DSN,required"`
	Host        string `env:"HOST" envDefault:"0.0.0.0"`
	Port        int    `env:"PORT" envDefault:"80"`
}

func SetupServer(i *trades.Implementation, cfg AppConfig) *http.Server {
	r := chi.NewRouter()

	registerMiddlewares(r)
	registerHandlers(r, i)

	server := http.Server{Addr: fmt.Sprintf("%s:%d", cfg.Host, cfg.Port), Handler: r}
	return &server
}

func StartServer(server *http.Server) {
	serverCtx, serverStopCtx := context.WithCancel(context.Background())

	sig := make(chan os.Signal, 1)
	signal.Notify(sig, syscall.SIGHUP, syscall.SIGINT, syscall.SIGTERM, syscall.SIGQUIT)

	go func(sig chan os.Signal) {
		<-sig
		log.Println("received shutdown event")

		shutdownCtx, _ := context.WithTimeout(serverCtx, 30*time.Second)

		go func() {
			<-shutdownCtx.Done()
			if shutdownCtx.Err() == context.DeadlineExceeded {
				log.Fatal("graceful shutdown timed out... forcing exit.")
			}
		}()

		err := server.Shutdown(shutdownCtx)
		if err != nil {
			log.Fatal(err)
		}
		serverStopCtx()
	}(sig)

	err := server.ListenAndServe()
	if err != nil && err != http.ErrServerClosed {
		log.Fatal(err)
	}

	<-serverCtx.Done()
}

func registerMiddlewares(r *chi.Mux) {
	r.Use(middleware.Logger)
}

func registerHandlers(r *chi.Mux, i *trades.Implementation) {
	r.Get("/ping", i.Ping)
	r.Get("/api/bns/bns_pair", i.GetBnsPair)
	r.Get("/api/bns/klines", i.GetBnsKlines)
}
