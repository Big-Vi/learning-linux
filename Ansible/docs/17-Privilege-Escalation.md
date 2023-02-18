## Privilege Escalation

become super user to assume super user privileges.

`sudo su`
`su <user>` - To become another user.  

become method - sudo, pfexec, doas, ksu, runas  

```
- name: Install Nginx
  become: yes
  become_method: doas
  become_user: <user>
  hosts: all
```

become can be set on inventory files or ansible config file.

Flag on the command line. 
--ask-become-pass -> To prompt to enter password. 
