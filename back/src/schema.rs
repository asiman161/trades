table! {
    bns_klines (pair, close_time) {
        pair -> Int4,
        open_time -> Timestamp,
        open -> Numeric,
        high -> Numeric,
        low -> Numeric,
        close -> Numeric,
        volume -> Float4,
        close_time -> Timestamp,
        quote_asset_volume -> Float4,
        number_of_trades -> Int4,
        taker_buy_base_asset_volume -> Float4,
        taker_buy_quote_asset_volume -> Float4,
    }
}

table! {
    bns_pairs (ticker, interval) {
        id -> Int4,
        ticker -> Text,
        interval -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    bns_klines,
    bns_pairs,
);
