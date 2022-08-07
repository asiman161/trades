package storage

import "github.com/jmoiron/sqlx"

type Storager interface {
}

type Storage struct {
	db *sqlx.DB
}

func New(db *sqlx.DB) Storager {
	return &Storage{db: db}
}
