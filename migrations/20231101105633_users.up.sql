-- Add up migration script here
create table "users"
(
    user_id       uuid primary key default gen_random_uuid(),
    username      text unique not null,
    password_hash text        not null
);