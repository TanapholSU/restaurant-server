-- select setval('orders_order_id_seq', (SELECT MAX(order_id) FROM orders)+1);
INSERT INTO orders(table_id, item_name, note, creation_time, estimated_arrival_time) VALUES
    (11, 'Kapao', 'With fried egg', '2024-01-11T15:26:00.281247Z', '2024-01-11T15:30:00.000000Z'),
    (11, 'Ramen', null, '2024-01-11T15:25:00.281247Z', '2024-01-11T15:40:00.000000Z');

