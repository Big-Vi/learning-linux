## Blocks

To group tasks together logically in order to avoid repetition of common attributes.  

```
-
hosts: all
tasks:
- block:
	- name: Nginx
	  yum: name=nginx state=present
	  become_user: www-data
    when: ansible_facts['distribution'] == 'Ubuntu'
  rescue:
	- mail:
		to: <email>
		subject: <subject>
		body: <body>
```
