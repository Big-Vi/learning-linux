## HTTP Proxy Server

`squid` - Proxy deamon  

`dnf install squid`  
`/etc/squid/squid.conf` - Config  
```
	acl <name-of-acl> dstdomain .<web-address> #Block domain and sub domains  
	http_access deny <name>  
	http_access deny to localhost #This denies the other services(DB server) from accessed.  
```