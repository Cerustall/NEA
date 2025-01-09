use sqlx::{Pool, pool::PoolOptions, mysql::{MySql, MySqlPool}};
use tokio::runtime::Runtime;
use chrono::prelude::*;
use chrono::{Datelike, Timelike, Utc, DateTime};
use serde::{Deserialize, Serialize};
use pretty_env_logger;
use warp::Filter;

#[derive(Serialize, Deserialize, Debug)]
struct Client{
    UUID: Option<String>,
    Username: Option<String>,
    Password: Option<String>,
    Email: Option<String>,
    LastIP: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message{
    MessageID: Option<String>,
    SenderID: Option<String>,
    ChannelID: Option<String>,
    Content: Option<String>,
    DateTimeSent: Option<String>, //Call NaiveDateTime to string func to initialise
}

#[derive(Serialize, Deserialize, Debug)]
struct Chat{
    ChatID: Option<String>,
    ChatName: Option<String>,
    OwnerID: Option<String>,
    DateTimeCreated: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserChatConnection{
    userChatConnectionID: Option<String>,
    UserID: Option<String>,
    ChannelID: Option<String>,
    DateTimeUserAdded: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let pool = database_connect().await?;
    
    pretty_env_logger::init();

    let test = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./web/index.html"))
    ;

    let route = test;

    warp::serve(route).run(([127, 0, 0, 1], 8080)).await;

    /*
    let clients = get_clients(&pool).await?;
    println!("{:#?}", clients); 
    */

    Ok(())

    /*
    let rt = Runtime::new()?;
        rt.block_on( async {
        let database_url = "mysql://edward:1152@localhost:3306/messenger";
    
        let pool: Pool<sqlx::mysql::MySql> = MySqlPool::connect(database_url).await?;

        println!("Connected to database successfully");
        
        let test = Client{
            UUID: Some(String::from("1")),
            Username: Some(String::from("Cerustall")),
            Password: Some(String::from("password")),
            Email: Some(String::from("edward.bailey.100@outlook.com")),
            LastIP: Some(String::from("0.0.0.0")),
        };
        
        insert_client(&pool, &test).await;

        

        Ok(())
    })
    */
}

async fn database_connect() -> Result<MySqlPool, sqlx::Error>{
    return MySqlPool::connect("mysql://edward:1152@localhost:3306/messenger").await;
}

async fn naive_date_time_to_string(dt: NaiveDateTime) -> String{
    // Get current UTC date and time.
    let date: DateTime<Utc> = Utc::now();
    // Format current UTC date and time into string
    let formatted_date = date.format("%Y/%m/%d %H:%M:%S").to_string();
    formatted_date
}

async fn get_clients(pool: &MySqlPool) -> Result<Vec<Client>, sqlx::Error> {
    let clients = sqlx::query_as!(
        Client,
        r#"SELECT * FROM clients"#
    )
    .fetch_all(pool)
    .await?;

    Ok(clients)
}

async fn insert_client(pool: &MySqlPool, client: &Client) -> bool {
    println!("Insert fn reached");
    let result = sqlx::query(
        "INSERT INTO clients (
            UUID, 
            Username, 
            Password, 
            Email, 
            LastIP) 
        values (?, ?, ?, ?, ?)"
    ).bind(&client.UUID)
    .bind(&client.Username)
    .bind(&client.Password)
    .bind(&client.Email)
    .bind(&client.LastIP)
    .execute(pool).await;

    match result {
        Err(e) => {
            println!("Error inserting client: {:#?}", client);
            println!("Error message: [{}].\n", e.to_string());
            return false;
        }

        Ok(res) => {
            println!("Client has been inserted.");
            println!("Number of clients inserted: {}", res.rows_affected());
        }
    }

    true
}
