use std::{fmt, str::FromStr};

use anyhow::Context as _;
use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

#[derive(Clone, Copy)]
pub enum Publicity {
    Public,
    Anonymous,
}

impl FromStr for Publicity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(Self::Public),
            "anonymous" => Ok(Self::Anonymous),
            _ => anyhow::bail!("Invalid publicity `{s}`, must be `public` or `anonymous`"),
        }
    }
}

impl Publicity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Anonymous => "anonymous",
        }
    }
}

#[derive(FromRow)]
pub struct Shoutout {
    pub sender: Option<String>,
    pub recipients: Vec<String>,
    pub message: String,
    pub created: OffsetDateTime,
}

impl Shoutout {
    // pub fn publicity(&self) -> &'static str {
    //     if self.sender.is_empty() {
    //         "anonymous"
    //     } else {
    //         "public"
    //     }
    // }

    pub fn recipient_list(&self) -> String {
        match &self.recipients[..] {
            [] => "no recipients".to_owned(),
            [recipient] => recipient.to_owned(),
            [first, second] => format!("{first} and {second}"),
            [all @ .., last] => format!("{}, and {last}", all.join(", ")),
        }
    }

    pub async fn create_table_if_missing(pool: &PgPool) -> anyhow::Result<()> {
        sqlx::query(
            "CREATE table IF NOT EXISTS shoutouts(
                 created    TIMESTAMP WITH TIME ZONE PRIMARY KEY NOT NULL,
                 sender     TEXT,
                 recipients TEXT[] NOT NULL,
                 message    TEXT NOT NULL,
             )",
        )
        .execute(pool)
        .await
        .context("Failed to create shoutouts table")?;

        Ok(())
    }

    pub async fn create(&self, pool: &PgPool) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO shoutouts(created, sender, recipients, message)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(&self.created)
        .bind(&self.sender)
        .bind(&self.recipients)
        .bind(&self.message)
        .execute(pool)
        .await
        .context("Failed to insert new shoutout into database")?;

        Ok(())
    }

    pub async fn for_last_week(pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        sqlx::query_as(
            "SELECT * FROM shoutouts
            WHERE created BETWEEN LOCAL_TIMESTAMP - INTERVAL '7 days' AND LOCAL_TIMESTAMP",
        )
        .fetch_all(pool)
        .await
        .context("Failed to retrieve the last week's worth of shoutouts")
    }
}

impl fmt::Display for Shoutout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Shoutout to {} for _\"{}\"_",
            self.recipient_list(),
            self.message
        )?;

        if let Some(sender) = &self.sender {
            write!(f, " from {}", sender)?;
        }

        Ok(())
    }
}

impl std::str::FromStr for Shoutout {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // publicity, rest = body['text'][0].strip().split(' ', 1)
        // if publicity == 'public':
        //     user_id = body['user_id'][0]
        //     sender = f'<@{user_id}>'
        // elif publicity == 'anonymous':
        //     sender = ''
        // else:
        //     raise Exception('Publicity must be "public" or "anonymous"')

        // recipient, rest = rest.split(' ', 1)
        // if not recipient.startswith('<@'):
        //     raise Exception('recipients must be users')

        // recipients = [recipient]
        // while rest.startswith('<@'):
        //     recipient, rest = rest.split(' ', 1)
        //     recipients.append(recipient)

        // if rest.startswith('"') or rest.startswith('\''):
        //     rest = rest[1:]

        // if rest.endswith('"') or rest.endswith('\''):
        //     rest = rest[:-1]

        // return Shoutout(sender, recipients, rest)
    }
}
