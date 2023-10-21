create table quotes (
    id text not null primary key,
    name text not null,
    created_at text not null,
    updated_at text not null
);

insert into quotes
    (id, name, created_at, updated_at)
values
    ('HLB99rYWEnVeZmzTpACXW','First quote',strftime('%Y-%m-%dT%H:%M:%fZ'),strftime('%Y-%m-%dT%H:%M:%fZ')),
    ('2iCNgm7s44hn-V4ofYbMY','Second quote',strftime('%Y-%m-%dT%H:%M:%fZ'),strftime('%Y-%m-%dT%H:%M:%fZ')),
    ('ez-f61kn7U5-YqpoGAdvU','Third quote',strftime('%Y-%m-%dT%H:%M:%fZ'),strftime('%Y-%m-%dT%H:%M:%fZ'));