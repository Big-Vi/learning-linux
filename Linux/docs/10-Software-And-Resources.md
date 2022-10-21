## Software updates - CentOS

`dnf check-upgrade`  
`dnf upgrade`  

## Manage softwares - CentOS

DNF - Package manager for CentOS  

`dnf list` - To list all installed and available packages.  
`dnf list installed` `dnf list available`  
`dnf check-update` - To check update for all the packages  
`dnf update` - To update all the packages  

`dnf search <keyword>`  
`dnf info/install/reinstall/remove/update <package-name>`  

`dnf repolist -v`  
To display enabled repos.  

`dnf repolist --all`  
To display enable and disabled repos.  

`dnf config-manager --enable <repo-id>`  
`dnf config-manager --add-repo <git-link>`  
For external repo  

`dnf group list` - To list all group packages  
`dnf group install --with-optional <group-name>`  
`dnf group remove <group-name>`  
`dnf group list --hidden`  

`dnf autoremove`  
To remove dependencies of main package  

`dnf history`  

`dnf provides <file-name>`  
To find which package install the component.  
e.g `dnf provides docker`  

`dnf repoquery --list <package>`  
To locate files installed by that package  


## Manage Resources

df = Disk Free utility  

`df -h` - To display Size column in human readable format(in MB)  

du = disk usage  

`du -sh /bin/`  
s=summaries h=human readable format  

`free -h` `free --mega` - To see how memory is used  

`uptime` - Load of CPU  

`lscpu` - To see specific CPU usage  

`lspci` - Other hardware usage  

`xfs_repair -v /dev/vdb1` - To repair an XFS filesystem  

`systemctl list-dependencies`  