## DNS zone

DNS server has data associated with certain domain. Zone can group data for specific domain.  
`vi /etc/named.conf`  

	zone "website.com" IN {
		type master;
		file "website.com.zone";
	}  

master means it's the main source of truth. There are slave type which sync with master.  

`/var/named` - Bind keeps all the files here   
`cp --preserve=ownership /var/named/named.localhost /var/named/website.com.zone` - Preserve ownership keeps the same owner and group when file copied which is important for bind to read that file.  
 ```
$TTL    1D
@       IN      SOA      @    admin@website.com. (
                                  2       ; Serial
                                  1D      ; Refresh
                                  1H      ; Retry
                                  1W      ; Expire
                                  3H      ; Negative Cache TTL
);
@       IN      NS      ns1.website.com.
@       IN      NS      ns2.website.com.
ns1     IN      A       10.20.10.2
ns2     IN      A       10.20.10.3
@               A       203.10.113.15
www             CNAME   203.10.113.15
website.com.    MX 10   mail.website.com.
                MX 20   mail2.website.com.
mail1           A       203.10.113.50
mail2           A       203.10.113.60
website.com.    TXT     "any text goes here"
```

TTL - Time to live tells other DNS servers to cache the DNS data for particular time period.  
`SOA - Start Of Authority` It's Type and it has multiple data as a field.  
`CNAME - Canonical name`  

`systemctl restart named.service` Restart named service to apply bind  