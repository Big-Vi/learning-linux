## SSH server & client

`/etc/ssh/sshd_config` - SSH Server config file  
```
	PermitRootLogin yes  
	PasswordAuthentication yes #auth via password  
	Port 22  
	AddressFamily inet #Listen to ipv4  
	Listener 10.20.20.0 #Only listen to this ip address  
	Match User <user-name> #Enable password auth for only this user  
		PasswordAuthentication yes  
```

To have multiple host configured for password login:  
`vi .ssh/config`  
```
	Host MyLinuxMachine  
		HostName 192.168.10.0  
		Port 22  
		User <user-name>  
```
`ssh MyLinuxMachine`  

`/etc/ssh/ssh_config` - SSH Client config file  

`ssh-keygen` - To generate ssh public and private keys.  

`ssh-copy-id <user-name>@<ip>` - To store SSH public key to remote server. so that using private key from local machine, remote server can be connected to.  
 
`.ssh/authorized_keys` - To store all pubic keys of users in server.  

`ssh-keygen -R <ip>` - To remove fingerprint  

`rm known_hosts` - To clear all fingerprints  

`/etc/ssh/ssh_config.d/99-<name>.conf` - To add default value for ssh client  