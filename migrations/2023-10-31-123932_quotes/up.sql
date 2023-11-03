create table quotes (
    id text not null primary key,
    name text not null,
    created_at text not null,
    updated_at text not null
);

insert into quotes
    (id, name, created_at, updated_at)
values
    ('01HE2X4FKPDTVHHB6C2HZD5Z53','First quote',strftime('%Y-%m-%dT%H:%M:%fZ'),strftime('%Y-%m-%dT%H:%M:%fZ')),
    ('01HE2X51WKBYSJ7ZPETRB9STCQ','Second quote',strftime('%Y-%m-%dT%H:%M:%fZ'),strftime('%Y-%m-%dT%H:%M:%fZ')),
    ('01HE2X5BB7JNHNXP17H9HM7H9C','Third quote',strftime('%Y-%m-%dT%H:%M:%fZ'),strftime('%Y-%m-%dT%H:%M:%fZ'));