## Networking configure

DHCP - Dynamic Host Configuration Protocol - To dynamically configure IP   

`ip link show` or `ip l` - To find name of network interface(adaptor)  

`ip address show` or `ip a` - To find IP address configured for network interface  

`lshw` - For detailed network interface details  

CIDR - Classless Inter Domain Routing  

`ip route show` or `ip r`  

`/etc/resolv.conf` - To see dns resolver conf  

`/etc/sysconfig/network-scripts` - Linux dynamically configures network based on the config file in here.  

NMTUI - Network Manager Text User Interface  
`nmtui` - To configure IP either manually or automatically.  

`nmcli device reapply <network-device-name>`  

`/etc/hosts` - Static resolution configuration  


## Network services

`dnf install NetworkManager`  
`systemctl status NetworkManager.service`  
`systemctl start/enable NetworkManager.service`  

`nmcli connection show`  
`nmcli connection modify <connection-name> autoconnect yes`  

Using ss utility
`ss -tunlp`  - To see program ready to accept incoming network connections  
	l=listening,  t=TCP connections,  u=UDP connections,  n=numeric values(port number),  p=processes  
	
[::]:22 - ipv6  
127.0.0.1:232 - internal network connection  
0.0.0.0:22 - external network connection  


## Packer filtering

Firewalld uses zones which are connected to at least one network interface.  

`dnf install firewalld`  

`firewall-cmd --get-default-zone` - All the incoming connections are blocked by default in the default zone  

firewall-cmd --set-default-zone=<zone-name> - To change the default zone  

`firewall-cmd --list-all` - To list all firewall rules  

`firewall-cmd --info-service=<service-name>`  

`firewall-cmd --add-service=<serice-name>`  
e.g, `firewall-cmd --zone=external --add-service=ftp` - if FTP transfer needs to be allowed in the external zone.  

`firewall-cmd --add-port=80/tcp`  

`firewall-cmd --remove-service=<service-name>` or  
`firewall-cmd --remove-port=80/tcp`  

`firewall-cmd --reload`  

`firewall-cmd --add-source/--remove-source=<ip-address> --zone=trusted`  

`firewall-cmd --get-active-zones`  

`firewall-cmd --runtime-to-permanent` - To apply existing temp rules  

`firewall-cmd --add-port=80/tcp --permanent` - To apply rule permanent using inline command  


## Statically route IP traffic

`ip route add <network-route-out-ip> via <network-route-in-ip> <device-name> <interface-name>`  
e,g, `ip route add 192.168.0.0 via 10.0.2.0 dev eth0`  

`ip route del <network-route-out-ip>`  

`ip route add/delete default via <network-route-in-ip>` - Gateway. If no route is found to reach the external network, default is used.  

`nmcli connection modify <network-name> +ipv4.routes "<network-route-out-ip> <network-route-in-ip>"` - To permanently apply  
e.g, `nmcli connection modify eth0 +ipv4.routes 192.168.0.0 10.0.2.0`  

`nmcli device reapply <network-name>`  

`nmcli connection modify <network-name> -ipv4.routes "<network-route-out-ip> <network-route-in-ip>"` - To remove  


## Synchronize time - Network peers

CentOS uses `chrony daemon` to keep accurate time.  

`timedatectl`  
`timedatectl list-timezones`  
`timedatectl set-timezone <region>/<city>`  

`dnf install chrony`  

`systemctl start/enable chronyd.service`  

`systemctl set-ntp true`  
NTP - Network Time Protocol  