package main

import (
	"net/http"

	"github.com/asiman161/trades/internal/app/trades"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

func main() {
	r := chi.NewRouter()
	r.Use(middleware.Logger)

	registerHandlers(r)
	http.ListenAndServe(":8080", r)
}

func registerHandlers(r *chi.Mux) {
	i := trades.New()
	r.Get("/", i.Hello)
}
