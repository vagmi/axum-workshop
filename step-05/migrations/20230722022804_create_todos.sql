begin;
create table todos(
    id serial primary key,
    title text not null,
    completed boolean default false,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp
);
commit;
