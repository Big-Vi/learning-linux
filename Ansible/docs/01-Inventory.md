## Inventory

Inventory holds list of hosts upon which Ansible run it's command to automate stuff.  

> vi /etc/ansible/hosts  

```
web ansible_host=server1.example.com ansible_connection=ssh

db ansible_host=server2.example.com ansible_connection=ssh ansible_user=root ansible_password=password

localhost ansible_connection=localhost


# For windows
db ansible_host=server2.example.com ansible_connection=winrm ansible_user=root ansible_password=password

# Group
[db_servers]
db

[web_servers]
web

# Group of groups
[all_servers:children]
web_servers
db_servers
``` 
