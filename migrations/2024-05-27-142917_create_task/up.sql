-- Your SQL goes here
CREATE TABLE "tasks"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"title" VARCHAR NOT NULL,
	"is_completed" BOOL NOT NULL
);

