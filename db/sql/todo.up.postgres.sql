create table if not exists todo
(
    id          serial primary key not null,
    description text               not null,
    done        boolean            not null
);
