table! {
    words (id) {
        id -> Int8,
        curr -> Text,
        next -> Nullable<Text>,
        start_sentinel -> Bool,
        end_sentinel -> Bool,
    }
}
