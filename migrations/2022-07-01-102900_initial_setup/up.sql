-- Your SQL goes here
create table users
(
    id          varchar not null primary key,
    chat_id     varchar not null,
    name        varchar not null
);

create table ledgers
(
    id          varchar not null primary key,
    borrower      varchar not null,
    owes     varchar not null,
    sum         int not null
);

create table transactions
(
    id                  varchar not null primary key,
    transaction_type    varchar not null,
    initiator           varchar not null,
    reciever            varchar not null,
    sum                 int not null,
    description         varchar not null,
    created             varchar not null
);