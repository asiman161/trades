-- Your SQL goes here

create table if not exists bns_pairs
(
    id         serial    not null,
    ticker     text      not null,
    interval   text      not null,
    created_at timestamp not null default now(),
    primary key (ticker, interval)
);

create index if not exists idx_bns_pairs_id on bns_pairs(id);
create index if not exists idx_bns_pairs_ticker_interval on bns_pairs(ticker, interval);

create table if not exists bns_klines
(
    pair                         int4           not null,
    open_time                    timestamp      not null,
    open                         decimal(14, 8) not null,
    high                         decimal(14, 8) not null,
    low                          decimal(14, 8) not null,
    close                        decimal(14, 8) not null,
    volume                       real           not null,
    close_time                   timestamp      not null,
    quote_asset_volume           real           not null,
    number_of_trades             int4           not null,
    taker_buy_base_asset_volume  real           not null,
    taker_buy_quote_asset_volume real           not null,
    primary key (pair, close_time)
);

CREATE INDEX IF NOT EXISTS idx_bns_klines_pair_close_time ON bns_klines (pair, close_time);