## Boot, reboot & shutdown

`systemctl reboot` - system control  
`systemctl poweroff`  
`shutdown 21:00` or `shutdown +15`  
`shutdown -r 21:00` or `shutdown -r +15`  - Shutdown and reboot with wall message  
  

## Boot into different operating modes

`systemctl get-default` - To find operating mode  
`systemctl set-default multi-user.target`  

Options:  
- Graphical target  
- Multi-user target  

`systemctl isolate graphical.target`  
  

## Bootloader
Bootloader helps to load OS into memory on boot time. There're several boot loaders. e.g, GRUB, LILO(Linux Loader) & BURG.  

GRUB - Grand Unified Boot Loader  
 
`lsblk` - List all block devices  


## Scripting to automate system admin tasks

`man systemd.service`  
`systemctl cat <service-name>.service`  
`systemctl edit --full <service-name>.service`  

`sytemctl start/stop/restart/reload/status/revert <service-name>.service`  
`systemctl reload/restart <service-name>.service`  

`systemctl disable <service-name>.service`  
`systemctl is-enabled <service-name>.service`  
`systemctl enable --now <service-name>.service` - enable and start  

`systemctl mask <service-name>.service`  
Prevent the service from starting accidentally.  

`systemctl list-units --type service --all`  