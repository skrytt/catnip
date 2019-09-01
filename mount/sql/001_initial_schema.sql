BEGIN;

PRAGMA encoding = "UTF-8";
PRAGMA foreign_keys = ON;

-- SQLite commands to create a database.
-- To execute this, create a database using the sqlite3 binary,
-- then type ".read setup.sql".

-- Table storing data about guilds the bot is in
CREATE TABLE IF NOT EXISTS Guilds (
    DiscordGuildId INTEGER PRIMARY KEY
);
CREATE UNIQUE INDEX IF NOT EXISTS IndexDiscordGuildId ON Guilds(DiscordGuildId);

-- Table storing data about users the bot has seen in its guilds
CREATE TABLE IF NOT EXISTS Users (
    DiscordUserId INTEGER PRIMARY KEY
);
CREATE UNIQUE INDEX IF NOT EXISTS IndexDiscordUserId ON Users(DiscordUserId);

-- Table storing data about members (that is: users in the context of particular guilds)
CREATE TABLE IF NOT EXISTS Members (
    MemberId INTEGER PRIMARY KEY AUTOINCREMENT,
    DiscordGuildId INTEGER NOT NULL,
    DiscordUserId INTEGER NOT NULL,
    LastStreamNotifyTimestamp INTEGER NOT NULL,

    FOREIGN KEY (DiscordGuildId) REFERENCES Guilds(DiscordGuildId),
    FOREIGN KEY (DiscordUserId) REFERENCES Users(DiscordUserId),
    UNIQUE (DiscordGuildId, DiscordUserId)
);
CREATE UNIQUE INDEX IF NOT EXISTS IndexDiscordMemberId ON Members(DiscordGuildId, DiscordUserId);

PRAGMA user_version=1;

COMMIT;
