## Kernel

There may be times to modify default kernel behavior. So `sysctl` command can be used to view and modify Kernel parameters at runtime.   

`sysctl -a` - To view all the Kernel parameters  
`sysctl <key-name>` - To view Kernel parameter value  

`sysctl -w <key>=<value>` - Setting value of Kernel parameter this way it's not persistent. Once the system reboots, it's set to default value.  

`vi /etc/sysctl.conf` or `vi /etc/sysctl.d/*.conf` - To set the value permanently, edit this file.  

`sysctl -p /etc/sysctl.d/*.conf` - To apply the changes  