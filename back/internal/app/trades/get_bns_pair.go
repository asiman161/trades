package trades

import (
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"

	"github.com/asiman161/trades/internal/models"
	"github.com/pkg/errors"
)

func (i *Implementation) GetBnsPair(w http.ResponseWriter, r *http.Request) {
	ticker, err := getTickerWithInterval(r.URL.Query().Get("ticker"), r.URL.Query().Get("interval"))
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		_, _ = w.Write([]byte(err.Error()))
		return
	}

	pair, err := i.store.GetBnsPair(context.TODO(), ticker.Ticker, ticker.Interval)
	if err != nil {
		if err == sql.ErrNoRows {
			w.WriteHeader(http.StatusNotFound)
			_, _ = w.Write([]byte(err.Error()))
			return
		}

		w.WriteHeader(http.StatusInternalServerError)
		_, _ = w.Write([]byte(err.Error()))
		return
	}

	w.Header().Set("content-type", "application/json")
	err = json.NewEncoder(w).Encode(pair)
	if err != nil {
		log.Println(err.Error())
	}
}

func getTickerWithInterval(ticker, interval string) (models.BnsPair, error) {
	t := models.BnsPair{
		Ticker:   ticker,
		Interval: interval,
	}
	if err := t.ValidateInterval(); err != nil {
		return models.BnsPair{}, errors.Wrap(err, "wrong ticker with interval")
	}

	return t, nil
}
