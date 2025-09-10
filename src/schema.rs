// @generated automatically by Diesel CLI.

pub mod indexer {
    diesel::table! {
        indexer.did_record (did) {
            did -> Varchar,
            ckbAddress -> Varchar,
            handle -> Varchar,
            txHash -> Varchar,
            txIndex -> Int4,
            document -> Varchar,
            height -> Int8,
            createdAt -> Varchar,
            valid -> Bool,
        }
    }
}
