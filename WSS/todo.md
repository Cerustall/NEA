## Frontend
- Build website
- Send requests from server
- Recieve requests from server 
- Allow users to register and deregister
## Backend
- Serves the website to client
- Recieve requests from client
- Process requests
- Return correct respponse to correct client in correct format
- Open, maintain, and close websocket connections with the client
- Databases to store client messages

## Database structure
- Database name: messenger
- Client entity
  - UUID (String, One)
  - Username (String, One)
  - Password (String, One)
  - IP address of current session when logged on (String, One)
  - Email (String, one)
- Message entity (JSON)
  - Data (String, One)
  - Sender (Client, One)
  - Reciever (Client, One)
  - Time sent (Time, One)
- Chat entity
  - Users (Client, Many) 
  - Messages (Message, Many)

## Additional thoughts and notes
- Using JSON
  - Inputs can be serialized/deserialized client side
  - Can also be serialized/deserialized server side for storage purposes
- Database
  - MySQL

## Crates to use
- Tokio (Async I/O)
  - https://crates.io/crates/tokio
- Tokio-tungstenite (Websockets)
  - https://crates.io/crates/tokio-tungstenite
- sqlx (Databases)
  - https://crates.io/crates/sqlx

## Sites found to help me
- Serializing data in JS
  - https://stackoverflow.com/questions/3316762/what-is-deserialize-and-serialize-in-json, 08/01/25 @ 0950
  - https://api.jquery.com/serialize/ 08/01/25 @ 0951
- Choosing a database library for Rust
  - https://rust-trends.com/posts/database-crates-diesel-sqlx-tokio-postgress/ 08/01/25 @ 0955
- SQLx help
  - https://crates.io/crates/sqlx 08/01/25 @ 0957
  - https://www.w3resource.com/rust-tutorial/master-database-interactions-rust-sqlx.php 08/01/25 @ 1001
- How to host MySQL
  - https://www.prisma.io/dataguide/mysql/5-ways-to-host-mysql 08/01/25 @ 1003
- Installing and setting up MySQL
  - https://www.prisma.io/dataguide/mysql/setting-up-a-local-mysql-database#setting-up-mysql-on-linux 08/01/25 @ 1005
  - https://www.atlantic.net/dedicated-server-hosting/how-to-install-mysql-on-arch-linux/ 08/01/25 @ 1007
- MySQL commands help
  - https://www.geeksforgeeks.org/how-to-show-a-list-of-all-databases-in-mysql/ 08/01/25 @ 1011
  - https://www.geeksforgeeks.org/how-to-show-list-tables-in-mysql-database/ 08/01/25 @ 1012
  - https://www.geeksforgeeks.org/mysql-create-table/ 08/01/25 @ 1049
  - https://www.w3schools.com/mysql/mysql_datatypes.asp 08/01/25 @ 1054
