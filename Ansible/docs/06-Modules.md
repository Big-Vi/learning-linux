## Modules

System, Commands, Files, Database, Cloud, Windows, etc...  

```
-
name: Play1
hosts: localhost
vars:
	dns_server: 10.12.12.0
tasks:
	- name: Display host
	  command: cat hosts chdir=/etc

	- name: Run script
	  script: test.sh -arg1 -arg2

	- name: Run service
	  service:
	  name: httpd
	  state: started

	- lineinfile:
		path: /etc/resolv.conf
		line: 'nameserver {{ dns_server }}'

	- name: Add the user
	  user:
		  name: johnd
		  uid: 1040
		  group: admin
```

Packages module   -> yum, apt, and package.  
Service module    -> service  
Firewall module   -> firewalld  
Storage module    -> lvg(volume group) and lvol(logical volume)
Filesystem module -> filesystem and mount
Archive module    -> archive and unarchive
Cron module       -> cron
Users & Groups module -> user, group and authorized_keys
File module -> file

  

chdir means change directory before running the command.  
started(idempotency) means if the service is not already started, start it.  
lineinfile(idempotency)  means search for a line in a file and replace it or add it if it doesn't exist.  
