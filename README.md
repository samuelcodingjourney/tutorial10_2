### Reflection Tutorial 10

2.1 Original code of broadcast chat.

Server screenshot:

![alt text](<assets/Screenshot (1098).png>)

Client 1 screenshot:

![alt text](<assets/Screenshot (1099).png>)

Client 2 screenshot:

![alt text](<assets/Screenshot (1100).png>)

Client 3 screenshot:

![alt text](<assets/Screenshot (1101).png>)

Explanation:
The server here acts as the one to accept incoming connections from multiple clients and the messages being received from the will be broadcasted to all the clients that are connected to the server. The client will then connect to the server, sends those messages and receive the broadcasted messages by the server that was generated because another client was sending a message to the server in order to broadcast to other users of course besides the server itself as can be seen in client screenshot for the client terminal and the respective numbered clients and their terminal screenshots. This way, the clients can communicate indirectly through the server. Especially in this case that follows three clients and a server communication. 

2.2 Modifying the websocket port

Explanation:
In the server side, the port specified in the let listener = TcpListener::bind("127.0.0.1:8080").await?; the port was changed to 8080. In order for the communication to be successful between client and server, in the client side, the port specified when connectin to the server using ClientBuilder::from_uri(), the port must be changed to 8080. So basically both server and client must be using the same WebSocket protocol meaning that their port after modified have to be the same for both sides to guarantee compatibility between them too. 

2.3 Small changes. Add some information to client

Server Screenshot:
![alt text](<assets/Screenshot (1103).png>)

Client 1 Screenshot:
![alt text](<assets/Screenshot (1104).png>)

Client 2 Screenshot:
![alt text](<assets/Screenshot (1105).png>)

Client 3 Screenshot:
![alt text](<assets/Screenshot (1106).png>)

Explanation:
The changes I made was adding client_count to keep track of the number of clients connected and updating the new connection message from Samuel's Computer with the included client count. These were done in the server.rs. On the other hand, in the client.rs file, what I did was removed unnecessary prefix Server broadcast: from printed messages. 