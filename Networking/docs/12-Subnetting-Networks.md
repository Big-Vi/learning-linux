## Subnetting Networks

Method to break up the extensive network into smaller networks.  

10.0.0.0/8  
Network - 00001010 | 00000000 00000000 00000000  
Broadcast - 00001010 | 11111111 11111111 11111111  

10.0.0.0 - 10.255.255.255  
We want a smaller range IP address for the network.  

To split, use /24  
10.0.10.0/24  

Network - 00001010 00000000 00001010 | 00000000  
Broadcast - 00001010 00000000 00001010 | 11111111  

10.0.10.0 - 10.0.10.255  
254 IP addresses are in this range.  

Network Address - 10.0.10.0  
Broadcast Address - 10.0.10.255  

Variable Length Subnet Masking(VLSM)  
VLSM is a subnet where all subnet masks can have varying sizes.  

E.g.,  
10.0.0.0/8 -> Larger network  
10.0.10.0/24  
10.0.10.0/22  
10.1.0.0/16  
