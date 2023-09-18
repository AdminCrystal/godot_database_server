
CREATE TABLE users (
	user_id uuid PRIMARY key not null,
	username varchar (18) UNIQUE NOT null,
	username_distinct varchar (18) unique not null,
	user_create_ts timestamp default CURRENT_TIMESTAMP
);

create table friends (
	user_id uuid primary key not null,
	friend_id uuid not null,
	friend_create_ts timestamp default current_timestamp,

	foreign key (user_id) references users(user_id),
	foreign key (friend_id) references users(user_id)
);

create table friend_requests (
	user_id uuid primary key not null,
	friend_id uuid not null,
	friend_create_ts timestamp default current_timestamp,
	
	foreign key (user_id) references users(user_id),
	foreign key (friend_id) references users(user_id)
);


create index username_search on users(username_distinct);

alter database postgres set timezone to 'UTC';
SELECT pg_reload_conf();

insert into users(user_id, username, username_distinct) 
values ('0461309d-d78b-4d88-b57d-7838842b8998', 'Admincrystal', 'admiuncrystal');

SELECT * FROM users u;

select * from friends f;

drop table users;

insert into 