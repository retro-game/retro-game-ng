-- Users

create table users (
    id uuid primary key,
    name text not null,
    email text not null,
    password text not null
);

create unique index users_lower_name_idx
                 on users (lower(name) text_pattern_ops);
create unique index users_lower_email_idx
                 on users (lower(email) text_pattern_ops);

-- Bodies

create table bodies (
    id uuid primary key,
    user_id uuid references users,
    name text not null,
    galaxy int not null,
    system int not null,
    position int not null,
    kind int not null,
    metal double precision not null,
    crystal double precision not null,
    deuterium double precision not null,
    resources_updated_at timestamptz not null,
    created_at timestamptz not null,
    diameter int not null,
    temperature int not null,
    type int not null,
    image int not null,
    buildings int[] not null default '{}',
    units int[] not null default '{}',
    building_queue int[] not null default '{}',
    shipyard_queue int[] not null default '{}',
    unique (galaxy, system, position, kind)
);

create index bodies_user_id_idx
          on bodies (user_id);
create index bodies_lower_name_idx
          on bodies (lower(name) text_pattern_ops);
