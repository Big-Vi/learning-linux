## Transport Control Protocol(TCP)

The transport layer establishes and maintains a session between the server and the client using TCP. TCP is a protocol mechanism built in to verify the message sent or received and it sets up the session and nicely ends the session.

### 3-way Handshake

The client sends a SYN(synchronize) message to the server and the server sends SYN-ACK back to the client. The client sends another message(ACK - acknowledgment) to the server again. Now the session is established.

SYN -> SYN-ACK -> ACK

Once the connection is established, the client can send an HTTP request to the server.

### 4-way Disconnect

FIN - Finish signal from server
FIN-ACK - Acknowledgement signal from the client
FIN - Finish signal from the client
FIN-ACk - Acknowledgement signal from server

After 4 signals, the connection is closed. So no more HTTP request happens after that.

RST - Reset the signal to quickly close the connection.
