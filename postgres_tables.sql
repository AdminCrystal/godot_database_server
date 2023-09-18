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
	user_id uuid not null,
	friend_id uuid not null,
	friend_request_ts timestamptz default current_timestamp,
	
	foreign key (user_id) references users(user_id) on delete cascade,
	foreign key (friend_id) references users(user_id) on delete cascade,
	CONSTRAINT unique_friend_requests PRIMARY KEY (user_id, friend_id)
);

create table if not exists blocked_users (
	user_id uuid not null,
	blocked_id uuid not null,
	blocked_ts timestamptz default current_timestamp,

	foreign key (user_id) references users(user_id) on delete cascade,
	foreign key (blocked_id) references users(user_id) on delete cascade,
	CONSTRAINT unique_blocked_users PRIMARY KEY (user_id, blocked_id)

);

