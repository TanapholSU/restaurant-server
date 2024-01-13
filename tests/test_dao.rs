
use sqlx::postgres::PgPool;

use restaurant_server::model::*;
use restaurant_server::dao::pg_order_dao::PgTableOrderDAO;
use restaurant_server::dao::order_dao::TableOrderDAO;
use restaurant_server::handlers::process_order_requests;


#[sqlx::test]
async fn test_add_one_order_record(pool: PgPool) {
    
    let dao = PgTableOrderDAO::new(pool.clone());
    
    // prepare table orders
    let mut table_orders = TableOrdersRequest::new(6);
    table_orders.add_order("sss", "large portion");
    let order_items: Vec<OrderItem> = process_order_requests(table_orders);

    // insert
    dao.add_table_orders(&order_items).await.expect("error in DAO impl");

    let last_order = sqlx::query_as!(OrderItem, "SELECT * FROM orders ORDER BY order_id DESC LIMIT 1")
        .fetch_one(&pool)
        .await
        .expect("cannot execute checking query");

    // check
    assert_eq!(last_order.table_id, 6);
    assert_eq!(last_order.item_name, "sss");
    assert_eq!(last_order.note, Some("large portion".to_string()));
    assert!((last_order.estimated_arrival_time - last_order.estimated_arrival_time).abs().num_minutes() < 15);

}



#[sqlx::test]
async fn test_add_multiple_order_records(pool: PgPool) {
    let dao = PgTableOrderDAO::new(pool.clone());

    let mut table_orders = TableOrdersRequest::new(6);
    table_orders.add_order_wihtout_note("kkk");
    table_orders.add_order("sss", "large portion");
    let order_items: Vec<OrderItem> = process_order_requests(table_orders);
    
    
    dao.add_table_orders(&order_items).await.expect("error in DAO impl");
    
    let orders: Vec<OrderItem> = sqlx::query_as!(OrderItem, "SELECT * FROM orders ORDER BY order_id ASC")
        .fetch_all(&pool)
        .await        
        .expect("cannot execute checking query");
    
    assert_eq!(orders.len(), 2);
    
    let order = &orders[0];
    assert_eq!(order.table_id, 6);
    assert_eq!(order.item_name, "kkk");
    assert_eq!(order.note, None);
    assert!((order.estimated_arrival_time - order.estimated_arrival_time).abs().num_minutes() < 15);

    
    let order = &orders[1];
    assert_eq!(order.table_id, 6);
    assert_eq!(order.item_name, "sss");
    assert_eq!(order.note, Some("large portion".to_string()));
    assert!((order.estimated_arrival_time - order.estimated_arrival_time).abs().num_minutes() < 15);


}



#[sqlx::test]
#[should_panic]
async fn test_add_order_with_invalid_records(pool: PgPool)  {
    
    let dao = PgTableOrderDAO::new(pool.clone());

    let mut table_orders = TableOrdersRequest::new(6);
    table_orders.add_order_wihtout_note(&"k".repeat(256));
    let order_items: Vec<OrderItem> = process_order_requests(table_orders);
    dao.add_table_orders(&order_items).await.unwrap();
}


#[sqlx::test(fixtures("orders"))]
async fn test_get_all_orders(pool: PgPool) {
    
    let dao = PgTableOrderDAO::new(pool.clone());
    let orders = dao.get_table_orders(11).await.expect("error in DAO impl");

    assert_eq!(orders.len(), 2);

    // first record
    let order = &orders[0];
    assert_eq!(order.table_id, 11);
    assert_eq!(order.item_name, "Kapao");
    assert_eq!(order.note, Some("With fried egg".to_string()));
    assert_eq!(order.creation_time.to_rfc3339_opts(chrono::SecondsFormat::Micros, true), "2024-01-11T15:26:00.281247Z");
    assert_eq!(order.estimated_arrival_time.to_rfc3339_opts(chrono::SecondsFormat::Micros, true), "2024-01-11T15:30:00.000000Z");
    
    // second record
    let order = &orders[1];
    assert_eq!(order.table_id, 11);
    assert_eq!(order.item_name, "Ramen");
    assert_eq!(order.note, None);
    assert_eq!(order.creation_time.to_rfc3339_opts(chrono::SecondsFormat::Micros, true), "2024-01-11T15:25:00.281247Z");
    assert_eq!(order.estimated_arrival_time.to_rfc3339_opts(chrono::SecondsFormat::Micros, true), "2024-01-11T15:40:00.000000Z");
}


#[sqlx::test(fixtures("orders"))]
async fn test_get_all_orders_with_empty_table(pool: PgPool) {
    let dao = PgTableOrderDAO::new(pool.clone());
    let orders = dao.get_table_orders(100).await.expect("error in DAO impl");

    assert_eq!(orders.len(), 0);
}


#[sqlx::test(fixtures("orders"))]
async fn test_get_specic_order(pool: PgPool) {
    let dao = PgTableOrderDAO::new(pool.clone());

    let check_order = sqlx::query_as!(OrderItem, "SELECT * FROM orders ORDER BY order_id ASC LIMIT 1").fetch_one(&pool).await.expect("cannot execute check query");
    let orders = dao.get_specific_table_order(check_order.table_id, check_order.order_id).await.expect("error in DAO impl");
    
    assert_eq!(orders.len(), 1);

    let order = &orders[0];
    assert_eq!(order.order_id, check_order.order_id);
    assert_eq!(order.table_id, check_order.table_id);
    assert_eq!(order.item_name, check_order.item_name);
    assert_eq!(order.note, check_order.note);
    assert_eq!(order.creation_time, check_order.creation_time);
    assert_eq!(order.estimated_arrival_time, check_order.estimated_arrival_time);
}


#[sqlx::test(fixtures("orders"))]
#[should_panic]
async fn test_get_order_with_invalid_ids(pool: PgPool) {
    let dao = PgTableOrderDAO::new(pool.clone());
    dao.get_specific_table_order(-1,-1).await.unwrap();
}


#[sqlx::test(fixtures("orders"))]
async fn test_remove_order(pool: PgPool) {
    let dao = PgTableOrderDAO::new(pool.clone());

    let check_order = sqlx::query_as!(OrderItem, "SELECT * FROM orders ORDER BY order_id ASC LIMIT 1").fetch_one(&pool).await.expect("cannot execute check query");
    dao.remove_order(check_order.table_id, check_order.order_id).await.expect("error in DAO impl");

    
    let remaining_orders = sqlx::query_as!(OrderItem, "SELECT * FROM orders ORDER BY order_id ASC").fetch_all(&pool).await.expect("cannot execute check query");
    assert_eq!(remaining_orders.len(), 1);

    let order = &remaining_orders[0];
    assert_eq!(order.table_id, 11);
    assert_eq!(order.item_name, "Ramen");
    assert_eq!(order.note, None);
    assert_eq!(order.creation_time.to_rfc3339_opts(chrono::SecondsFormat::Micros, true), "2024-01-11T15:25:00.281247Z");
    assert_eq!(order.estimated_arrival_time.to_rfc3339_opts(chrono::SecondsFormat::Micros, true), "2024-01-11T15:40:00.000000Z");   
}

