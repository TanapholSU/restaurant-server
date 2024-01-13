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
    * Pros -> Less communication requests. Client doesn't need to call get all tables again
    * Cons -> Larger payload. 

* The requirements don't contain any information about 
  * what kind of restaurants or the available menu items is limited  or not
    * A order record in the database stores a single item  name as String directly
    * If we want limit what dishes customers can order, we can create a separated menu items table and join with orders table
  * security -> omitted entirely

* To make URL path readable, I decided to use sub-resource method like `/api/v1/tables/<table_id>/orders/<order_id>`
  * `table_id` is limited to `i16`  because postgres doesn't support unsigned and simple restaurant should not have much tables
  * `order_id` is `i32` because for simplicity (easy to read/test) and for simple restaurant should be enough. 