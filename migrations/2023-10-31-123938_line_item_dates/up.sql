create table line_item_dates (
    id text not null primary key,
    quote_id text not null,
    "date" text not null,
    created_at text not null,
    updated_at text not null,
    foreign key(quote_id) references quotes(id)
);

create unique index idx_quote_id_and_date on line_item_dates (quote_id, "date");
create index idx_date on line_item_dates ("date");
create index idx_quote_id on line_item_dates (quote_id);
