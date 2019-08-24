
use rusqlite::{params, Connection};

#[derive(Debug, Default)]
pub struct Guild {}

#[derive(Debug, Default)]
pub struct User {}

#[derive(Debug, Default)]
pub struct Member {
    pub last_stream_notify_timestamp: i64,
}

impl Member {
}

pub struct Handle {
    connection: Connection,
}

impl Handle {
    pub fn new(db_path: &str) -> Self {
        Self {
            connection: Connection::open(db_path)
                .expect("rusqlite::Connection::open() failed"),
        }
    }

    pub fn guild(&self,
        guild_id: u64,
    ) -> Result<Option<Guild>, ()> {
        let mut stmt = match self.connection.prepare(
            // Placeholder for querying something of actual use...
            "SELECT DiscordGuildId FROM Guilds WHERE DiscordGuildId = ?1")
        {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };

        let mut result_iter = match stmt.query_map(
            params![guild_id as i64],
            |row|
        {
            Ok(Guild{})
        }) {
            Ok(result_iter) => result_iter,
            Err(_) => return Err(()),
        };

        match result_iter.next() {
            Some(guild) => Ok(Some(guild.unwrap())),
            None => Ok(None),
        }
    }

    pub fn guild_update(&self,
        guild_id: u64,
        data: &Guild,
    ) -> Result<(), ()>
    {
        let mut stmt = match self.connection.execute(
            // Placeholder for storing something of actual use...
            "INSERT OR REPLACE INTO Guilds(DiscordGuildId) VALUES(?1)",
            params![guild_id as i64],
        )
        {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };
        Ok(())
    }

    pub fn user(&self,
        user_id: u64,
    ) -> Result<Option<User>, ()> {
        let mut stmt = match self.connection.prepare(
            // Placeholder for querying something of actual use...
            "SELECT DiscordUserId FROM Users WHERE DiscordUserId = ?1")
        {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };

        let mut result_iter = match stmt.query_map(
            params![user_id as i64],
            |row|
        {
            Ok(User{})
        }) {
            Ok(result_iter) => result_iter,
            Err(_) => return Err(()),
        };

        match result_iter.next() {
            Some(user) => Ok(Some(user.unwrap())),
            None => Ok(None),
        }
    }
    pub fn user_update(&self,
        user_id: u64,
        data: &User,
    ) -> Result<(), ()>
    {
        let mut stmt = match self.connection.execute(
            // Placeholder for storing something of actual use...
            "INSERT OR REPLACE INTO Users(DiscordUserId) VALUES(?1)",
            params![user_id as i64],
        )
        {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };
        Ok(())
    }

    pub fn member(&self,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Option<Member>, ()> {
        let mut stmt = match self.connection.prepare(
            "SELECT LastStreamNotifyTimestamp FROM Members
             WHERE DiscordGuildId = ?1 AND DiscordUserId = ?2")
        {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };

        let mut result_iter = match stmt.query_map(
            params![
                guild_id as i64,
                user_id as i64,
            ],
            |row|
        {
            Ok(Member {
                last_stream_notify_timestamp: row.get(0).unwrap(),
            })
        }) {
            Ok(result_iter) => result_iter,
            Err(_) => return Err(()),
        };

        match result_iter.next() {
            Some(member) => Ok(Some(member.unwrap())),
            None => Ok(None),
        }
    }

    pub fn member_update(&self,
        guild_id: u64,
        user_id: u64,
        data: &Member
    ) -> Result<(), ()>
    {
        let mut stmt = match self.connection.execute(
            "INSERT OR REPLACE INTO Members(
             DiscordGuildId, DiscordUserId, LastStreamNotifyTimestamp)
             VALUES(?1, ?2, ?3)",
            params![
                guild_id as i64,
                user_id as i64,
                data.last_stream_notify_timestamp,
            ],
        )
        {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };
        Ok(())
    }
}

