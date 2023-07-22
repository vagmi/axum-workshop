begin;
    create table users (
        id serial primary key,
        name text not null unique,
        email varchar(255) not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now()
    );
    insert into users (name, email) values ('admin', 'admin@localhost');

    alter table todos 
                add column user_id integer not null default 1
                references users(id)  on delete cascade;
commit;
