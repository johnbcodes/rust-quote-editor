create table line_items (
    id text not null primary key,
    line_item_date_id integer not null,
    name text not null,
    description text,
    quantity integer not null,
    unit_price decimal(10,2) not null,
    created_at text not null,
    updated_at text not null,
    foreign key(line_item_date_id) references line_item_dates(id)
);

create index idx_line_item_date_id on line_items (line_item_date_id);