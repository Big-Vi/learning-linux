## Configure HTTP server

`dnf install httpd` - Apache server  

`firewall-cmd --add-service=http`  
`firewall-cmd --add-service=https`  

`dnf install mod_ssl` - To enable ssl and use `Certbot` for certification.  

`httpd -M` - To list modules  

`man httpd.conf` - Manual for how to configure  

`/etc/httpd/` - Config files  

`/etc/httpd/conf/httpd.conf` - Primary config file  

`/etc/httpd/conf.d/second-website.conf` - To add multiple website  
```
	<VirtualHost *:80>  
		ServerName blog.website.com  
		DocumentRoot /var/www/blog/  
	</VirtualHost>  
	<VirtualHost *:80>  
		ServerName marketing.website.com  
		DocumentRoot /var/www/marketing/  
	</VirtualHost> . 
```

`apachectl configtest` - To test added virtual host  

`/etc/httpd/conf.d/ssl.conf`  

`/etc/httpd/conf.modules.d` - To enable/disable modules  

Multi-Processing Modules (MPMs). MPM enables Apache server to run on wide variety of platforms.  
  

## Configure HTTP server log files

`/etc/httpd/conf/httpd.conf` - To find where log files get stored  

add CustomLog, ErrorLog to VirtualHost to have separate logs  

## Restrict access to a web page

`mv /etc/httpd/conf.d/second-website.conf /etc/httpd/conf.d/second-website.conf.disabled` - To disable the second website.  

`vi /etc/httpd/conf/httpd.conf`  

```
	<Directory "/var/www/html">  
		"Options Indexes FollowSymlinks" #Indexes let user browse folder if there's no index file. FollowSymlinks allow httpd to follow symlink.  
		AllowOverride All/None #Enable/Disable the .htaccess file.  
		Require ip <address1> <address2> #To allow connection from these ip's.  
	<Directory>
```

`htpasswd -c /etc/httpd/passwords <user-name>` - To protect the file with password. `c` option creates file if it doesn't exist.  
`htpasswd -D /etc/httpd/passwords <user-name>` - To delete password for that user.  
`cat /etc/httpd/passwords` - To list passwords for users.  

```
	<Directory "/var/www/html/admin">  
		AuthType Basic  
		AuthBasicProvider file  
		AuthName "Admin page"  
		AuthUserFile /etc/httpd/passwords  
		Require valid-user  
	<Directory>  
```