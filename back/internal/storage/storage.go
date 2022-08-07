package storage

import (
	"context"
	"database/sql"

	sq "github.com/Masterminds/squirrel"
	"github.com/asiman161/trades/internal/models"
	"github.com/jmoiron/sqlx"
	"github.com/pkg/errors"
)

type Storager interface {
	GetBnsPair(ctx context.Context, ticker, interval string) (models.BnsPair, error)
}

type Storage struct {
	db *sqlx.DB
}

func (s Storage) GetBnsPair(ctx context.Context, ticker, interval string) (models.BnsPair, error) {
	pair := models.BnsPair{}
	q, args, _ := sq.Select(models.BnsPairColumns...).
		From("bns_pairs").
		Where(sq.Eq{"ticker": ticker, "interval": interval}).
		PlaceholderFormat(sq.Dollar).ToSql()
	err := s.db.GetContext(ctx, &pair, q, args...)
	if err != nil {
		if err == sql.ErrNoRows {
			return models.BnsPair{}, err
		}
		return models.BnsPair{}, errors.New("can't find bns pair")
	}

	return pair, err
}

func New(db *sqlx.DB) Storager {
	return &Storage{db: db}
}
