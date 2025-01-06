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


## state

 Databases
  website
}

struct connection {

}


struct client

## Database structure
- Client entity (JSON)
  - UUID
  - Username
  - Password
  - IP address of current session when logged on
  - Logged on or not
- Message entity (JSON)
  - Data (String)
  - Sender (Client)
  - Reciever (Client)
  - Time sent

