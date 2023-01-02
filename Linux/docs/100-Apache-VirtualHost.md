## Apache server setup

Ports:  

80 - unencrypted, 443 - encrypted(TLS)  

Browser sends network packet to destination port in the server. Apache accepts this request and then sends the necessary data from the server to the browser. We can configure Apache using VirtualHost about where to look into based on the request IP and DNS name.  

Use named VirtualHost rather than IP based.  

Main config file. We can set global settings in here.  
> /etc/apache2/apache2.conf
```
	`IncludeOptional sites-enabled/*.conf`  #This line forward the request to sites-enabled folder and reads all the .conf files
```

`etc/apache2/sites-enabled` - 000-default.conf is symlink which points to /sites-available/000-default.conf

Name starts with 000 to makes sure it's first file to process  

> /etc/apache2/sites-available/my-app.conf
```
<VirtualHost  *:80> #Wildcard says any ip
ServerName website.com
ServerAlias www.website.com
ServerAdmin mail@hotmail.com
DocumentRoot /var/www/html #What dir to serve
<Directory /var/www/html> #This let the user add .htaccess file inside that dir and can override the global config for apache
	AllowOverride All
<Directory>
ErrorLog ${APACHE_LOG_DIR}/error-myapp.log #set the dir for error and custom log to go
CustomLog ${APACHE_LOG_DIR}/custom-myapp.log combined
</VirtualHost>
```

Above configuration can be included in main config file too.  

To enable the site and creates symlink in `/etc/apache2/sites-enabled`  

`a2ensite my-app.conf`  

or  

`ln -s /etc/apache2/sites-available/my-app.conf`  

`systemctl reload apache2`  
  
To look at all the configuration together.   

`ls -l /etc/apache2/sites-enabled/*`

Precedence level varies based on VirtualHost(IP or DNS named) type and where it's placed on the config files.  
```
<VirtualHost 192.168.10.20:80> #This one takes precedence since it got ip specified in.
</VirtualHost>
```
`a2dissite myapp`  
`apache2ctl -S` -> To view useful config info  
