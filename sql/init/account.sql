create table if not exists account (
    id varchar not null primary key,
    name varchar not null,
    email varchar not null unique,
    password varchar not null,
    created_at timestamp not null,
    updated_at timestamp not null
)
