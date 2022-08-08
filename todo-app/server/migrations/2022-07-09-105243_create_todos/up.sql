CREATE TABLE todos (
   id SERIAL PRIMARY KEY,
   value VARCHAR(200) NOT NULL,
   created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
   updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

SELECT diesel_manage_updated_at('todos');