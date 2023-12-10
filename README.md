# Asyncronous CHAT with RUST

TO RUN:

## START THE SERVER AT LOCALHOST PORT 8080
```
         cargo run --release --bin server localhost:8080
```

## CLIENT (in different shell)
```
          cargo run --release client localhost::8080
```

SUMMARY: The client and the server work together by establishing a connection to each other with the client sending request to the server and the server responding to those requests.
The server manages chatrooms using the chats and the chatMaps modules while the client sends request to join chatrooms and send messages using the client module.
Both the server and client use shared functionalities provided in the lib.rs and utils modules.


## The server is responsible for receiving incoming connections from Clients and handling thier request.


It's composed of 4 files - connection.rs chats.rs chats_maps.rs and main.rs - that work together to implement the server funcitonality.

## MAIN

Main is the entrypoint of the program and it is responsible for setting up the server listening socket as well as acceptin incoming connections. Threads are spawned to handle each incoming connection and then each thread created by main is responsible for processing incoming requests from a single client

## CONNECTION

Connection is used to represent a single connection between the server and a client. It contains methods to read and write to and from the client as well as to handle specific requests such as sending a message to the chatroom.

## CHATS

Chats contains the logic for managing the chatrooms and provides methods for creating new chatrooms and sending messages to the chatrooms.

# CHATMAPS

Chatmaps is responsible for maintaining and mapping our chatroom ids to the actual chatroom object 

## ON THE CLIENT SIDE

On the client side the client is responsible for connecting to the server and sending requests to it.
The client is not doing too much. 

# NOTES:
Both server and clients utilize the library file and utils.   

