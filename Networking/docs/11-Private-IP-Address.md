## Private IP Address

Private IP Address Range
-------
10.0.0.0 - 10.255.255.255       -> 10.0.0.0/8
172.16.0.0 - 172.31.255.255     -> 172.16.0.0/12
192.168.0.0 - 192.168.255.255   -> 192.168.0.0/16

### Automatic Private IP Addressing(APIPA)

APIPA - 169.254.0.0/16

Microsoft windows and other org uses this address. So avoid using this range.

127.0.0.1 - Loopback Address - To test local system.

Network address needs to be the same for both devices to communicate. If devices on two different network, router is needed. 

E.g.,
192.168.10.100 is different network from 192.168.11.20
