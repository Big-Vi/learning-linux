## Conditionals & Loops

```
-
name: Play1
hosts: localhost
vars:
	packages:
	- name: nginx
	  required: True
tasks:
- name: Install nginx
  apt:
	name: {{item.name}}
	state: present
	when: ansible_os_family == "Debian" and/or ansible_distribution_version == "15.0"
	loop: "{{packages}}"
```

`with_*` directive can be used loop if needed.  

E.g,
```
with_file
with_url
etc...
```
