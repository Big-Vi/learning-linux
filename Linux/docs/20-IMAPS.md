## IMAP & IMAPS

Internet Message Access Protocol Secure(SSL)  

IMAP allows to access emails stored in remote server using email clients. Email client like Outlook logs into Dovecot(IMAPS) to retrieve email stored on the remote server.  

`def install dovecot`  
`firewall-cmd --add-service=imap` - To allow outside world to login to this service, add the service to firewall.  

`/etc/dovecot/dovecot.conf` - To change protocol. listeners and other stuff.  

`/etc/devecot/conf.d` - Config are grouped here  

`/etc/devecot/conf.d/10-master.conf` - To change ports where dovecot listens on for incoming connections.  

`/etc/dovecot/conf.d/10-mail.conf` - To change the location where mail stored(/var/spool/mail/<user>).  

`/etc/dovecot/conf.d/10-ssl.conf` - To change TLS settings.  