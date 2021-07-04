CREATE TABLE IF NOT EXISTS todo_lists 
(
    id   integer NOT NULL,
    name text    NOT NULL,
    text text    NOT NULL,
    done boolean NOT NULL DEFAULT FALSE,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS items 
(
    id   integer NOT NULL,
    todo_list_id integer NOT NULL,
    name text    NOT NULL,
    due_date timestamp NOT NULL,
    PRIMARY KEY(id)
);


