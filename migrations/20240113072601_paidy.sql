-- Add migration script here

CREATE TABLE orders(   
    order_id SERIAL PRIMARY KEY,
    table_id SMALLINT NOT NULL,
    item_name VARCHAR(255) NOT NULL,
    note VARCHAR(255),
    creation_time TIMESTAMPTZ NOT NULL,
    estimated_arrival_time TIMESTAMPTZ NOT NULL
);


-- INSERT INTO orders(table_id, item_name, note, creation_time, estimated_arrival_time) VALUES(1, 'A', 'b', now(), now())