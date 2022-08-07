package models

import (
	"time"

	"github.com/pkg/errors"
)

var BnsPairColumns = []string{"id", "ticker", "interval", "created_at"}

type BnsPair struct {
	Id        int       `json:"id" db:"id"`
	Ticker    string    `json:"ticker" db:"ticker"`
	Interval  string    `json:"interval" db:"interval"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
}

type BnsKline struct {
	Pair                     int       `json:"pair" db:"pair"`
	OpenTime                 time.Time `json:"open_Time" db:"open_time"`
	Open                     float64   `json:"open" db:"open"`
	High                     float64   `json:"high" db:"high"`
	Low                      float64   `json:"low" db:"low"`
	Close                    float64   `json:"close" db:"close"`
	Volume                   float64   `json:"volume" db:"volume"`
	CloseTime                time.Time `json:"close_Time" db:"close_time"`
	QuoteAssetVolume         float64   `json:"quote_Asset_Volume" db:"quote_asset_volume"`
	NumberOfTrades           float64   `json:"number_Of_Trades" db:"number_of_trades"`
	TakerBuyBaseAssetVolume  float64   `json:"taker_Buy_Base_Asset_Volume" db:"taker_buy_base_asset_volume"`
	TakerBuyQuoteAssetVolume float64   `json:"taker_Buy_Quote_Asset_Volume" db:"taker_buy_quote_asset_volume"`
}

func (bp *BnsPair) ValidateInterval() error {
	if _, ok := Intervals[bp.Interval]; ok {
		return nil
	}

	return errors.Errorf("wrong interval: %s", bp.Interval)
}

var Intervals = map[string]time.Duration{
	OneMinute:      OneMinuteV,
	ThreeMinutes:   ThreeMinutesV,
	FiveMinutes:    FiveMinutesV,
	FifteenMinutes: FifteenMinutesV,
	ThirtyMinutes:  ThirtyMinutesV,
	OneHour:        OneHourV,
	TwoHours:       TwoHoursV,
	FourHours:      FourHoursV,
	SixHours:       SixHoursV,
	EightHours:     EightHoursV,
	TwelveHours:    TwelveHoursV,
	OneDay:         OneDayV,
}

const (
	OneMinuteV      = time.Minute
	ThreeMinutesV   = OneMinuteV * 3
	FiveMinutesV    = OneMinuteV * 5
	FifteenMinutesV = FiveMinutesV * 3
	ThirtyMinutesV  = OneMinuteV * 30
	OneHourV        = OneMinuteV * 60
	TwoHoursV       = OneHourV * 2
	FourHoursV      = TwoHoursV * 2
	SixHoursV       = TwoHoursV * 3
	EightHoursV     = FourHoursV * 2
	TwelveHoursV    = SixHoursV * 2
	OneDayV         = TwelveHoursV * 2
)

const (
	OneMinute      = "1m"
	ThreeMinutes   = "3m"
	FiveMinutes    = "5m"
	FifteenMinutes = "15m"
	ThirtyMinutes  = "30m"
	OneHour        = "1h"
	TwoHours       = "2h"
	FourHours      = "4h"
	SixHours       = "6h"
	EightHours     = "8h"
	TwelveHours    = "12h"
	OneDay         = "1d"
)
