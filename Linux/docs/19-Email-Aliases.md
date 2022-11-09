## Email aliases - Configure

> Postfix - Email server  

`dnf install postfix`  

`sendmail <user>@localhost <<< "Hello"`  

`mailx` client to send mails.

`/var/spool/mail/<user>` - Email stored here for that user  

`vi /etc/aliases` - To add email aliases  
```
	contact: <user>,<user2>,<user3>  
	marketing: <user>@otherwebsite.com - To store externally  
```
`newaliases` - To let postfix know about the change  