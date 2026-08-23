-- Schema iniziale, estratto da crates/web/db.sqlite.
--
-- La tabella `tower_sessions` NON è qui: la crea session_store.migrate().
-- Se la aggiungessi, le due migrazioni si contenderebbero la stessa tabella.

CREATE TABLE "users" (
    "id"         INTEGER      NOT NULL PRIMARY KEY AUTOINCREMENT,
    "email"      VARCHAR(255) NOT NULL,
    "password"   VARCHAR(255) NOT NULL,
    "steam_id"   TEXT,
    "created_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX "unique_email"       ON "users" ("email" ASC);
CREATE UNIQUE INDEX "idx_users_steam_id" ON "users" ("steam_id");

CREATE TABLE "nodes" (
    "id"         VARCHAR(255) NOT NULL PRIMARY KEY,
    "user_id"    INTEGER      NOT NULL,
    "parent_id"  VARCHAR(255),
    "type"       VARCHAR(255) NOT NULL,
    "name"       VARCHAR(100) NOT NULL,
    "created_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY ("user_id")   REFERENCES "users" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("parent_id") REFERENCES "nodes" ("id") ON DELETE CASCADE
);

CREATE TABLE "notes" (
    "id"         VARCHAR(255) NOT NULL PRIMARY KEY,
    "user_id"    INTEGER      NOT NULL,
    "content"    TEXT,
    "created_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY ("id")      REFERENCES "nodes" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE TABLE "steam_search_cache" (
    "query"      VARCHAR(255) NOT NULL PRIMARY KEY,
    "results"    TEXT      	  NOT NULL,
    "created_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "games" (
    "id"      	 INTEGER	  NOT NULL PRIMARY KEY,
    "name"    	 VARCHAR(255) NOT NULL,
    "assets"	 TEXT		  DEFAULT NULL,
    "created_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "games_users" (
    "id"      	 INTEGER	  NOT NULL PRIMARY KEY AUTOINCREMENT,
    "game_id"    INTEGER	  NOT NULL,
    "user_id"	 INTEGER	  NOT NULL,
    "created_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY ("game_id") REFERENCES "games" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE UNIQUE INDEX "idx_games_users" ON "games_users" ("user_id", "game_id");

CREATE TRIGGER "nodes_updated_at"
AFTER UPDATE ON "nodes"
FOR EACH ROW BEGIN
    UPDATE "nodes" SET "updated_at" = CURRENT_TIMESTAMP WHERE "id" = NEW."id";
END;

CREATE TRIGGER "games_updated_at"
AFTER UPDATE ON "games"
FOR EACH ROW BEGIN
    UPDATE "games" SET "updated_at" = CURRENT_TIMESTAMP WHERE "id" = NEW."id";
END;

CREATE TRIGGER "games_users_updated_at"
AFTER UPDATE ON "games_users"
FOR EACH ROW BEGIN
    UPDATE "games_users" SET "updated_at" = CURRENT_TIMESTAMP WHERE "id" = NEW."id";
END;
