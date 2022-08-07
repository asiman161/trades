package trades

import (
	"context"
	"encoding/json"
	"net/http"
)

func (i *Implementation) GetBnsKlines(w http.ResponseWriter, r *http.Request) {
	ticker := r.URL.Query().Get("ticker")
	interval := r.URL.Query().Get("interval")
	klines, err := i.bnsClient.NewKlinesService().Symbol(ticker).Interval(interval).Do(context.TODO())
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		_, _ = w.Write([]byte(err.Error()))
		return
	}

	w.Header().Set("content-type", "application/json")
	err = json.NewEncoder(w).Encode(klines)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		_, _ = w.Write([]byte(err.Error()))
	}
}
