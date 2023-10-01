use std::sync::Arc;
use sqlx::{Execute, Pool, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;
use crate::models::message_structs::{CreateMessage, Message, MessageBetweenUsers};
use anyhow::Result;

pub async fn create_message(txn: &mut Transaction<'_, Postgres>, create_message: &CreateMessage) -> Result<Uuid> {


    let uuid = Uuid::new_v4();
    sqlx::query(
        "
        insert into messages(message_id, create_user_id, message, game_id)
        values ($1, $2, $3, $4);
        ")
        .bind(&uuid)
        .bind(&create_message.create_user_id)
        .bind(&create_message.message)
        .bind(&create_message.game_id)
        .execute(&mut **txn)
        .await?;

    return Ok(uuid);
}

pub async fn send_message(txn: &mut Transaction<'_, Postgres>, message_id: &Uuid, recipients: Vec<Uuid>) -> Result<()> {

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "insert into message_recipients(message_id, user_id) "
    );

    query_builder.push_values(recipients.iter().take(recipients.len()), |mut b, user| {
        b.push_bind(message_id)
            .push_bind(user);
    });

    let mut query = query_builder.build();

    query.execute(&mut **txn).await?;

    return Ok(());
}

pub async fn get_message_between_users(pool: Arc<Pool<Postgres>>, message_between_users: &MessageBetweenUsers) -> Result<Vec<Message>> {

    let incoming_friend_requests: Vec<Message> = sqlx::query_as(
        "
        select m.create_user_id, m.message, m.game_id, m.message_create_ts
        from messages m
        join message_recipients mr on m.message_id = mr.message_id
        where m.create_user_id = $1
        and mr.user_id = $2
        union
        select m.create_user_id, m.message, m.game_id, m.message_create_ts
        from messages m
        join message_recipients mr on m.message_id = mr.message_id
        where m.create_user_id = $2
        and mr.user_id = $1
        order by message_create_ts;
        ")
        .bind(&message_between_users.user_id1)
        .bind(&message_between_users.user_id2)
        .fetch_all(&*pool)
        .await?;

    return Ok(incoming_friend_requests);
}