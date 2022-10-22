## SELinux

SELinux - Security module - Security Enhanced Linux  

`sestatus` - To view the SELinux status  
`/etc/sysconfig/selinux` - To view/edit the selinux mode and security policy.  

`ps axZ` - To list selinux context label(user:role:domain:level) for processes  
`ls -Z <file-path>` - To list selinux context label(user:role:type:level) for files. Only file that marked with certain type can start the process.  

`id -Z` - To view security context of current user  


SELinux maps the currently logged in user to SELinux user or user to SELinux roles. To see the mapping run below commands:  

`semanage login -l`  
`semanage user -l`  

`getenforce` - To view the selinux status  
`setenforce` to change the selinux status  

`chcon` to change security context 
`chcon -u bigv_u /file` - To change the user security context  
`chcon -u dev_r /file` - To change the role security context  
`chcon -u s0 /file` - To change the level security context  
`chcon -R -t httpd_sys_content_t /directory/` - To change the type security context of the directory recursively.  
