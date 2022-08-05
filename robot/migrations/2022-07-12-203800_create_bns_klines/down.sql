-- This file should undo anything in `up.sql`

drop index idx_bns_pairs_id;
drop index idx_bns_pairs_ticker_interval;
drop index idx_bns_klines_pair_close_time;

drop table if exists bns_klines;
drop table if exists bns_pairs;