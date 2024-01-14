# Requirements (from task description)

* The client (the restaurant staff “devices” making the requests) MUST be able to: add one or more items with a 
table number, remove an item for a table, and query the items still remaining for a table.
* The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
* The application MUST, upon deletion request, remove a specified item for a specified table number.
* The application MUST, upon query request, show all items for a specified table number.
* The application MUST, upon query request, show a specified item for a specified table number.
* The application MUST accept at least 10 simultaneous incoming add/remove/query requests.
* The client MAY limit the number of specific tables in its requests to a finite set (at least 100).
* The application MAY assign a length of time for the item to prepare as a random time between 5-15 minutes.
* The application MAY keep the length of time for the item to prepare static (in other words, the time does not have 
to be counted down in real time, only upon item creation and then removed with the item upon item deletion).

# Intuitions / notes

* Like billing for each table, we can think that adding orders = update the table information (bill for that table)
  * So, adding orders won't return order item directly. It will returns the updated table orders
  * Removing order operation is similar to adding operation. It will returns the updated table orders
    * Pros -> Less communication requests. Client doesn't need to call get all tables again after adding new orders
    * Cons -> Larger payload. 

* The requirements don't contain any information about 
  * what kind of restaurants or the uniqueness of menu items 
    * Therefore, a order record in the database stores a single item  name as String directly
    * If we want limit what dishes customers can order, we can create a list menu items table and then we can join with orders table
  * security -> omitted entirely

* To make URL path readable, I decided to use sub-resource method like `/api/v1/tables/<table_id>/orders/<order_id>`
  * `table_id` is limited to `i16`  because postgres doesn't support unsigned and simple restaurant should not have much tables
  * `order_id` is `i32` because for simplicity (easy to read/test) and for simple restaurant should be enough.
* For client application, it is stored in a separated repository >>> [here](https://github.com/TanapholSU/restaurant-client/)


# Deployment (for testing purpose)
## Prerequisites
* Docker
* Rust 1.75 as official async trait support with trait variants crate are used

## Setup postgresql database 
* In the same directory, run `docker compose up -d pgdb`
* Then, install sqlx cli using command `cargo install sqlx-cli`
* Configure `DATABASE_URL` in `.env` file if necessary (see below)
* Run `sqlx db setup` to initialize database

## .env configuration
The application automatically loads config in the .env (if available).  Please configure this file or setup OS environment first.

```
DATABASE_URL = <database connection url>  # e.g., postgres://postgres:password@localhost/test
HOST = <host ip/ uri>  # e.g., 0.0.0.0  to accept all clients
PORT = 3333  # service port for client
```

## Running & test
After settingup database and config `.env`, run `cargo run` as usual to run server 

To run unit and integration tests (for DAO and REST API), execute `cargo test` command

## Docker 
It is possible to build and deploy using docker compose, please follow these steps:

1. Configure environment parameters of restaurant service in `docker-compose.yml` if necessary
2. `docker compose build` to build restaurant service
3. If necessary, run `docker compose up -d pgdb` to deploy database. After that, access to db and manually setup database schema and execute sql file in `migration` directory to create table
4. `docker compose up -d restaurant`  to run restaurant service

# Rest API details
## Add orders

* URL endpoint is `/api/v1/tables/<table id>/orders` where `<table_id>` is the target table id
  * Example:  `http://127.0.0.1/api/v1/tables/1/orders`  for adding orders to table `1`
* Send `TableOrderRequest`  object *(see below in object section)* by using `POST` method to the endpoint
* If success, Server returns the updated TableOrderResponse object (see below)  with HTTP status code `200` 
* If fail, Server returns error object with the following HTTP error status code
  *  `404` if table in the path is larger than `MAX_TABLES` setting 
  *  `400` if request payload or parameters in path are incorrect (e.g., larger than `i16`)
  *  `500` if there is something wrong with server/db.
  

## Get orders for specific table

* Similar to add orders, the URL endpoint is `/api/v1/tables/<table id>/orders` where `<table_id>` is the target table id
* Send `GET` method to the endpoint
* If success, Server returns the current TableOrderResponse object with HTTP status code `200` 
* If fail, Server returns error object with HTTP error status code. `404` is returned if table in the path is larger than `MAX_TABLES` setting. `400` if `<table_id>` in URL path is incorrect. `500` if there is anything wrong with DB/server. 


## Get specifc order from specific table

* URL endpoint is `/api/v1/tables/<table id>/orders/<order_id>` where `<table_id>` and `<order_id>` are the target table and order ids, respectively
  * Example:  `http://127.0.0.1/api/v1/tables/1/orders/5`  for getting order `5` from table `1`
* Send `GET` method to the endpoint
* If success, Server returns the TableOrderResponse object with only specific order in the `orders` field and HTTP status code `200` 
* If fail, Server returns error object with HTTP error status code. `404` is returned if table in the path is larger than `MAX_TABLES` setting or order does not exist. `400` if `<table_id>` or `order_id` in URL path is incorrect.  `500` if there is anything wrong with DB/server.


## Remove specifc order from specific table

* Similar to get specific order function, the URL endpoint is `/api/v1/tables/<table id>/orders/<order_id>` where `<table_id>` and `<order_id>` are the target table and order ids, respectively
* Send `DELETE` method to the endpoint
* If success, Server returns the updated TableOrderResponse object with HTTP status code `200` 
* If fail, Server returns error object with HTTP error status code. `404` is returned if table in the path is larger than `MAX_TABLES` setting or order does not exist. `400` if `<table_id>` or `order_id`  in URL path is incorrect.  `500` if there is anything wrong with DB/server.


## Json Payload objects


### TableOrderRequest object
This object is used for client to add orders to specific table. It includes list of OrderItemRequest objects.

| Attribute   | Type                   | Description                                                                          |
|-------------|------------------------|--------------------------------------------------------------------------------------|
| table_id    | number                 | The target table id                                                                 |
| orders      | List[OrderItemRequest object] | List of OrderItemRequest objects each of which contains order information  (for more info, please read further)            


### OrderItemRequest object

This object contains (and partial) order information of each order submitted by client.

| Attribute   | Type                   | Description                                                                          |
|-------------|------------------------|--------------------------------------------------------------------------------------|
| table_id    | number                 | The request table id                                                                 |
| item_name | String                 | item name |
| note      | String | Optional note 


#### Sample TableOrderRequest payload with two OrderItemRequest objects (for Add orders function via POST)

```
{
  "table_id": 7,
  "orders": [
    {
      "table_id": 7,
      "item_name": "Pizza",
      "note": "Without pineapple"
    },
    {
      "table_id": 7,
      "item_name": "Sushi",
      "note": "Less rice"
    }
  ]
}
```




### TableOrderResponse object
This object represents the table information returning from application. 

| Attribute   | Type                   | Description                                                                          |
|-------------|------------------------|--------------------------------------------------------------------------------------|
| table_id    | number                 | The request table id                                                                 |
| status_code | number                 | status code (just in case we want to include more fine-grained status in the future) |
| orders      | List[OrderItem object] | List of OrderItem objects each of which contains all order information                   |


### OrderItem object
This object represents  a single order item 
| Attribute   | Type                   | Description                                                                          |
|-------------|------------------------|--------------------------------------------------------------------------------------|
| order_id    | number                 | The request table id                                                                 |
| table_id    | number                 | The request table id                                                                 |
| item_name | String                 | item name |
| note      | String | Optional note 
| creation_time      | String | Order creation time 
| estimated_arrival_time      | String | Estimated time to finished cooking



#### Sample TableOrderResponse which contains one OrderItem

```
{
  "status_code": 200,
  "table_id": 1,
  "orders": [
    {
      "order_id": 1,
      "table_id": 1,
      "item_name": "Pizza",
      "note": "Without pineapple"
      "creation_time": "2024-01-13T08:51:01.846234Z",
      "estimated_arrival_time": "2024-01-13T08:51:01.846237Z"
    }
  ]
}
```


### Error object
error json object is returned when an error occurs

| Attribute   | Type                   | Description                                                                          |
|-------------|------------------------|--------------------------------------------------------------------------------------|
| status_code    | number                 | HTTP error status code
| error_cause | String                 | cause of the error |

#### sample error object

```
{
  "error_cause": "Bad request -> parameters in path are incorrect",
  "status_code": 400
}
```

