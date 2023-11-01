create table comments (
    comment_id uuid primary key default gen_random_uuid(),
    post_id uuid not null references posts(post_id),
    user_id uuid not null references "users"(user_id),
    content text not null,
    created_at timestamptz not null default now()
);

create index on comments(post_id, created_at);