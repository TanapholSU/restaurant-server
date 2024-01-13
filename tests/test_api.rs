
use chrono::SecondsFormat;
use sqlx::postgres::PgPool;
use http_body_util::BodyExt; // for `collect`
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use serde_json::{json, Value};
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use restaurant_server::model::TableOrdersResponse;
use restaurant_server::context::ApiContext;
use restaurant_server::routes::app;


#[sqlx::test(fixtures("orders"))]
async fn test_get_all_valid_orders(db: PgPool) {
    let context: ApiContext = ApiContext::new(db);
    let app = app(context);


    // GET
    let response = app
    .oneshot(
        Request::builder()
        .uri("/api/v1/tables/11/orders")
            .method(http::Method::GET)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let table_order: TableOrdersResponse = serde_json::from_slice(&body).unwrap();

    // check two records
    assert_eq!(table_order.table_id, 11);
    assert_eq!(table_order.status_code, 200);
    assert_eq!(table_order.orders.len(), 2);
    assert_eq!(table_order.orders[0].item_name, "Kapao");
    assert_eq!(table_order.orders[0].note, Some(format!("With fried egg")));
    assert_eq!(table_order.orders[0].creation_time.to_rfc3339_opts(SecondsFormat::Micros, true), "2024-01-11T15:26:00.281247Z" );
    assert_eq!(table_order.orders[0].estimated_arrival_time.to_rfc3339_opts(SecondsFormat::Micros, true), "2024-01-11T15:30:00.000000Z" );

    assert_eq!(table_order.orders[1].item_name, "Ramen");
    assert_eq!(table_order.orders[1].note, None);
    assert_eq!(table_order.orders[1].creation_time.to_rfc3339_opts(SecondsFormat::Micros, true), "2024-01-11T15:25:00.281247Z" );
    assert_eq!(table_order.orders[1].estimated_arrival_time.to_rfc3339_opts(SecondsFormat::Micros, true), "2024-01-11T15:40:00.000000Z" );
    
}


#[sqlx::test(fixtures("orders"))]
async fn test_get_all_orders_from_incorrect_table(db: PgPool) {
    let context: ApiContext = ApiContext::new(db);

    let app = app(context);

    let response = app
    .oneshot(
        Request::builder()
        .uri("/api/v1/tables/-1/orders")
            .method(http::Method::GET)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response, json!{
        {
            "status_code": 404,
            "error_cause": "Table not found"
        }
    })
}


#[sqlx::test(fixtures("orders"))]
async fn test_get_specific_order(db: PgPool) {
    let context: ApiContext = ApiContext::new(db);


    let response = app(context.clone())
    .oneshot(
        Request::builder()
        .uri("/api/v1/tables/11/orders")
            .method(http::Method::GET)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let table_order: TableOrdersResponse = serde_json::from_slice(&body).unwrap();

    // get valid order id
    let last_known_order = &table_order.orders[1];
    let last_known_order_id = table_order.orders[1].order_id;
    
    let valid_order_response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/11/orders/{last_known_order_id}"))
                                .method(http::Method::GET)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    let body = valid_order_response.into_body().collect().await.unwrap().to_bytes();
    let valid_table_order: TableOrdersResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(valid_table_order.orders.len(), 1);
    assert_eq!(valid_table_order.orders[0], *last_known_order);
}



#[sqlx::test(fixtures("orders"))]
async fn test_get_specific_non_existence_order(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/1/orders/1"))
                                .method(http::Method::GET)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 404,
            "error_cause": "Order not found"
        }
    });

    
    
}




#[sqlx::test(fixtures("orders"))]
async fn test_add_orders(db: PgPool) {
    let context: ApiContext = ApiContext::new(db);

    let app = app(context);

    let response = app
    .oneshot(
        Request::builder()
        .uri("/api/v1/tables/44/orders")
            .method(http::Method::POST)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&json!(
                    {
                        "table_id": 44,
                        "orders": [
                          {
                              "table_id": 44,
                              "item_name": "A",
                              "note": "Some note"
                          },
                          {
                              "table_id": 44,
                              "item_name": "C"
                          }
                        ]
                    }
                )).unwrap(),
            ))
            .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let orders: TableOrdersResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(orders.table_id, 44);
    assert_eq!(orders.status_code, 200);
    assert_eq!(orders.orders.len(), 2);
    assert_eq!(orders.orders[0].table_id, 44);
    assert_eq!(orders.orders[0].item_name, "A");
    assert_eq!(orders.orders[0].note.as_deref(), Some("Some note"));

    assert_eq!(orders.orders[1].table_id, 44);
    assert_eq!(orders.orders[1].item_name, "C");
    assert_eq!(orders.orders[1].note.as_deref(), None);
}


#[sqlx::test(fixtures("orders"))]
async fn test_add_orders_with_mismatch_table_id_in_payload(db: PgPool) {
    let context: ApiContext = ApiContext::new(db);

    let app = app(context);

    let response = app
    .oneshot(
        Request::builder()
        .uri("/api/v1/tables/44/orders")
            .method(http::Method::POST)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&json!(
                    {
                        "table_id": 44,
                        "orders": [
                          {
                              "table_id": 44,
                              "item_name": "A",
                              "note": "Some note"
                          },
                          {
                              "table_id": 45,
                              "item_name": "C"
                          }
                        ]
                    }
                )).unwrap(),
            ))
            .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> table id in json request (or path) is incorrect"
        }
    });
}


#[sqlx::test(fixtures("orders"))]
async fn test_add_orders_with_mismatch_table_id_in_path(db: PgPool) {
    let context: ApiContext = ApiContext::new(db);

    let app = app(context);

    let response = app
    .oneshot(
        Request::builder()
        .uri("/api/v1/tables/55/orders")
            .method(http::Method::POST)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&json!(
                    {
                        "table_id": 44,
                        "orders": [
                          {
                              "table_id": 44,
                              "item_name": "A",
                              "note": "Some note"
                          },
                          {
                              "table_id": 44,
                              "item_name": "C"
                          }
                        ]
                    }
                )).unwrap(),
            ))
            .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> table id in json request (or path) is incorrect"
        }
    });
}


#[sqlx::test]
async fn test_add_orders_with_out_of_range_path_table_id(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/70000/orders"))
                                .method(http::Method::POST)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::from(
                                    serde_json::to_vec(&json!(
                                        {
                                            "table_id": 70000,
                                            "orders": [
                                              {
                                                  "table_id": 70000,
                                                  "item_name": "A",
                                                  "note": "Some note"
                                              },
                                              {
                                                  "table_id": 70000,
                                                  "item_name": "C"
                                              }
                                            ]
                                        }
                                    )).unwrap(),
                                ))
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> parameters in path are incorrect"
        }
    });

    
    
}




#[sqlx::test]
async fn test_get_all_orders_with_out_of_range_path_table_id(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/70000/orders"))
                                .method(http::Method::GET)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> parameters in path are incorrect"
        }
    });

}



#[sqlx::test]
async fn test_get_specific_order_with_out_of_range_path_table_id(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/70000/orders/1"))
                                .method(http::Method::GET)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> parameters in path are incorrect"
        }
    });

}



#[sqlx::test]
async fn test_get_specific_order_with_out_of_range_path_order_id(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/1/orders/2147483650"))
                                .method(http::Method::GET)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> parameters in path are incorrect"
        }
    });

}


#[sqlx::test]
async fn test_get_specific_order_with_out_of_range_path_ids(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/70000/orders/2147483650"))
                                .method(http::Method::GET)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> parameters in path are incorrect"
        }
    });

}


#[sqlx::test]
async fn test_remove_order_with_out_of_range_path_table_id(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/70000/orders/1"))
                                .method(http::Method::DELETE)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> parameters in path are incorrect"
        }
    });

}


#[sqlx::test]
async fn test_remove_order_with_out_of_range_path_order_id(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/1/orders/2147483650"))
                                .method(http::Method::DELETE)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> parameters in path are incorrect"
        }
    });

}


#[sqlx::test]
async fn test_remove_order_with_out_of_range_path_ids(db: PgPool) {    
    let context: ApiContext = ApiContext::new(db);

    let response = app(context.clone())
                        .oneshot(
                            Request::builder()
                            .uri(format!("/api/v1/tables/70000/orders/2147483650"))
                                .method(http::Method::DELETE)
                                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::empty())
                                .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let check_json_value: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(check_json_value, json!{
        {
            "status_code": 400,
            "error_cause": "Bad request -> parameters in path are incorrect"
        }
    });

}

