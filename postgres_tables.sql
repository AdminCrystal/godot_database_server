CREATE TABLE if not exists users (
	user_id uuid PRIMARY key not null,
	username varchar (18) UNIQUE NOT null,
	username_distinct varchar (18) unique not null,
	user_create_ts timestamptz default current_timestamp
);

create index if not exists username_search on users(username_distinct);


create table if not exists friends (
	user_id uuid not null,
	friend_id uuid not null,
	friend_create_ts timestamptz default current_timestamp,

	foreign key (user_id) references users(user_id) on delete cascade,
	foreign key (friend_id) references users(user_id) on delete cascade,
	
	CONSTRAINT unique_friends PRIMARY KEY (user_id, friend_id)

);

create table if not exists friend_requests (
	create_user_id uuid not null,
	recipient_id uuid not null,
	friend_request_ts timestamptz default current_timestamp,
	
	foreign key (create_user_id) references users(user_id) on delete cascade,
	foreign key (recipient_id) references users(user_id) on delete cascade,
	CONSTRAINT unique_friend_requests PRIMARY KEY (create_user_id, recipient_id)
);



create table if not exists blocked_users (
	user_id uuid not null,
	blocked_id uuid not null,
	blocked_ts timestamptz default current_timestamp,

	foreign key (user_id) references users(user_id) on delete cascade,
	foreign key (blocked_id) references users(user_id) on delete cascade,
	CONSTRAINT unique_blocked_users PRIMARY KEY (user_id, blocked_id)

);

create table if not exists games (
	game_id uuid primary key not null,
	created_ts timestamptz default current_timestamp,
	create_user_id uuid not null,
	game_ip cidr not null,
	is_active bool not null,
	game_name varchar(25) not null,
	is_public bool not null,
	game_code varchar(7) not null,
	
	foreign key (create_user_id) references users(user_id) on delete cascade

);
create index if not exists users_created_games on games(create_user_id);
create index if not exists game_codes on games(game_code);

create table if not exists game_participants (
	game_id uuid not null,
	join_ts timestamptz default current_timestamp,
	user_id uuid not null,
	user_ip cidr not null,

	foreign key (game_id) references games(game_id) on delete cascade,
	foreign key (user_id) references users(user_id) on delete cascade,
	CONSTRAINT games_participating_in PRIMARY KEY (game_id, user_id)

);
