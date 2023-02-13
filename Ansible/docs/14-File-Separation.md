## File separation

> vi inventory file

```
[web_servers]
web
db
```

Name the file as the same as the host(web_servers). So ansible automaticaly fetches those files.  

Folder structure

- inventory
- host_vars
- web.yml
- db.yml
- group_vars
- web_servers.yml

Use `include_vars` module to get info of file defined outside of the above inventory folder.  

To view inventory  

`ansible-inventory -i inventory -y`

`include_tasks` to better manage the tasks relevant to server type.  

```
- name: Include example
  hosts: all
  tasks:
  - include_tasks: <file-path-to-playbook>
```