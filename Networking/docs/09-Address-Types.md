## Address Types

Network Address:
- Identifier for a group of devices. It's called Network Prefix.

Broadcast Address:
- Identifier for all devices on a network.

Host Address:
- Identifies unique devices on a network. It's used on the internet.

Network address:
203.0.113.0
11111111 11111111 11111111 00000000
The host portion of the address has all binary 0's. It means its Network Address.

Broadcast address:
203.0.113.255
10111000 11001010 01110001 11111111
The host portion of the address has all binary 1's. It means it's Broadcast Address.

Host Address:
203.0.113.10
10111000 11001010 01110001 00001010
The host portion of the address has anything except all binary 0's or 1's.

Host address example:
10.128.225.0 - IP address
00001010 10000000 11100001 00000000
255.255.254.0 - Subnet mask
11111111 11111111 11111110 00000000
