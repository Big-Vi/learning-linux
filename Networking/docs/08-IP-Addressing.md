## IP addressing

IP operates at Layer 3- Network Layer. An IP address is a unique identifier for the device and helps to communicate long distances.

E.g.,
204.10.0.114 = 11001011 00000000 01110001 00001010

Each number is between 0-255 with 4 sets. The 32-bit value is broken into 4 octets (8-bit long). It has Network Portion(First three) & Host Portion(last one).

The network Portion is like the zip code in the address and Host Portion is like the street address.

The network Portion of the IP address represents groups of network devices. Host Portion to identify the devices in the network.

## Classful Addressing - Before 1995

Class | IP Range
--- | --- |
A | 0.0.0.0 - 127.255.255.255
B | 128.0.0.0 - 191.255.255.255
C | 192.0.0.0 - 233.255.255.255
D | 224.0.0.0 - 239.255.255.255
E | 240.0.0.0 - 255.255.255.255

The first 3 classes are called Unicast addresses and only they can be used on the internet. Class D is called Multicast & not available for public internet. Only for enterprise org.

Class E is experimental.

A multicast, single server can send a single copy of data to multiple devices.
E.g., Sending an e-mail to a mailing list Or Radio channel.

## Classless Addressing - After 1995

To figure out Network and Host Portion, a subnet mask can be used. Add 1 for the Network Portion and 0 for the Host Portion.

204.10.0.114   -> IP
11111111.11111111.11111111.00000000 -> Subnet mask
255.255.255.0  -> Subnet mask

10.0.0.10
11111111 11111111 11110000 00000000
255.255.240.0
