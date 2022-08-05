-- Your SQL goes here
create table if not exists indicators_data
(
    ticker     text      not null,
    interval   text      not null,
    data       jsonb     not null,
    close_time timestamp not null,
    primary key (ticker, interval, close_time)
)
