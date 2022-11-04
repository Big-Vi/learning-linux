
## DNS server cache

bind - DNS Server app which can cache the DNS data.  

`dnf install bind bind-utils`  

`/etc/named.conf` - Bind config file  
`man named.conf` - To see all the options available  
	- `listen-on port 80 {127.0.0.1; 192.168.2.0;};` Or  
	  `listen-on port 80 {any}` - To listen on port 80 from listed ip addresses or any ip addresses  
	- `allow-query {localhost;192.168.1.0/24;};` or `allow-query {any}`  
	- `recursion yes;` - This allows the bind server gets info from other bind servers on the internet.  

`systemctl start/enable named.service`  
`firewall-cmd --add-service=dns --permanent` - Add dns service to firewall to external connection  