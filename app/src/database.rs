
use rusqlite::{params, Connection};
use std::{env, fs, io::Read};

const DEFAULT_DATABASE_PATH: &'static str = "/catnip/mount/catnip.db3";

#[derive(Clone, Debug, Default)]
pub struct Guild {}

#[derive(Clone, Debug, Default)]
pub struct User {
    pub title: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Member {
    pub last_stream_notify_timestamp: i64,
}

pub struct Handle {
    connection: Connection,
}

impl Handle {
    pub fn new() -> Self {
        // Set DATABASE_PATH in the mount/env file to override the default path.
        let db_path: String = match env::var("DATABASE_PATH") {
            Err(_) => String::from(DEFAULT_DATABASE_PATH),
            Ok(path) => path,
        };
        Self {
            connection: Connection::open(db_path)
                .expect("rusqlite::Connection::open() failed"),
        }
    }

    pub fn update_schema(&self) -> Result<(), ()> {
        // Read file names into a Vec for sorting
        let mut sql_files: Vec<_> = match fs::read_dir("/catnip/mount/sql") {
            Ok(files) => files,
            Err(_) => {
                error!("Could not find any sqlfiles");
                return Err(())
            }
        }
        .map(|r| r.unwrap())
        .collect();
        sql_files.sort_by_key(|dir| dir.path());
        debug!("Found sql files {:?}", sql_files);

        let mut sql_file_iter = sql_files.iter();
        loop {
            // Read current user_version
            let user_version: i32 = match self.connection.pragma_query_value(
                None, "user_version", |row| row.get(0))
            {
                Ok(value) => value,
                Err(_) => {
                    error!("Could not query user_version of database");
                    return Err(())
                }
            };

            // Try to find the next user_version (N+1) to upgrade to
            let prefix_to_find = format!("{:03}", user_version+1);
            debug!("searching for sql file matching prefix {}", prefix_to_find);

            match sql_file_iter.next() {
                Some(sql_file) => if sql_file.file_name()
                    .into_string()
                    .unwrap()
                    .starts_with(prefix_to_find.as_str())
                {
                    let mut file = match fs::File::open(sql_file.path()) {
                        Ok(file) => file,
                        Err(_) => return Err(()),
                    };
                    debug!("Opened sql file {:?}", sql_file.path());

                    let mut sql_content = String::new();
                    if let Err(_) = file.read_to_string(&mut sql_content) {
                        error!("Could not read SQL file {:?}", sql_file.path());
                        return Err(())
                    }
                    debug!("Read sql file content");

                    info!("Applying DB schema migration {:?}", sql_file.path());
                    match self.connection.execute_batch(sql_content.as_str()) {
                        Ok(_) => info!("Migrated successfully"),
                        Err(_) => {
                            error!("Failed to apply schema migration");
                            return Err(())
                        }
                    };
                },
                None => break,
            }
        }

        Ok(())
    }

    pub fn guild(&self,
        guild_id: u64,
    ) -> Result<Guild, ()> {
        let mut stmt = match self.connection.prepare(
            // Placeholder for querying something of actual use...
            "SELECT DiscordGuildId FROM Guilds WHERE DiscordGuildId = ?1")
        {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };

        let mut result_iter = match stmt.query_map(
            params![guild_id as i64],
            |_row|
        {
            Ok(Guild{})
        }) {
            Ok(result_iter) => result_iter,
            Err(_) => return Err(()),
        };

        match result_iter.next() {
            Some(guild) => Ok(guild.unwrap()),
            None => {
                debug!("No db entry found for guild_id {}, returning default Guild instance",
                    guild_id);
                Ok(Default::default())
            }
        }
    }

    pub fn guild_update(&self,
        guild_id: u64,
        _data: &Guild,
    ) -> Result<(), ()>
    {
        if let Err(_) = self.connection.execute(
            // Placeholder for storing something of actual use...
            "INSERT OR REPLACE INTO Guilds(DiscordGuildId) VALUES(?1)",
            params![guild_id as i64],
        )
        {
            return Err(())
        };
        Ok(())
    }

    /// Get a user's data.
    /// Return a default User instance if no record was found.
    pub fn user(&self,
        user_id: u64,
    ) -> Result<User, ()> {
        let mut stmt = match self.connection.prepare(
            "SELECT Title FROM Users WHERE DiscordUserId = ?1")
        {
            Ok(stmt) => stmt,
            Err(_) => return Err(()),
        };

        let mut result_iter = match stmt.query_map(
            params![user_id as i64],
            |row|
        {
            Ok(User{
                title: row.get(0).unwrap(),
            })
        }) {
            Ok(result_iter) => result_iter,
            Err(_) => return Err(()),
        };

        match result_iter.next() {
            Some(user) => Ok(user.unwrap()),
            None => {
                debug!("No db entry found for user_id {}, returning default User instance",
                    user_id);
                Ok(Default::default())
            },
        }
    }

    pub fn user_update(&self,
        user_id: u64,
        data: &User,
    ) -> Result<(), ()>
    {
        if let Err(_) = self.connection.execute(
            "INSERT OR REPLACE INTO Users(DiscordUserId, Title) VALUES(?1, ?2)",
            params![
                user_id as i64,
                data.title,
            ],
        )
        {
            return Err(())
        };
        Ok(())
    }

    pub fn member(&self,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Member, ()> {
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
            Some(member) => Ok(member.unwrap()),
            None => {
                debug!("No db entry found for member_id {}, returning default Member instance",
                    guild_id);
                Ok(Default::default())
            },
        }
    }

    pub fn member_update(&self,
        guild_id: u64,
        user_id: u64,
        data: &Member
    ) -> Result<(), ()>
    {
        if let Err(_) = self.connection.execute(
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
            return Err(())
        };
        Ok(())
    }
}

