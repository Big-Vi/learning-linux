## Jinja2 - Templating language

Ansible extended Jinja2 filters.  

> vi index.html.js
```
Host name is {{ inventory_hostname }}
```

Use template module instead of copy module for template interpolation capability.

```
tasks:
	- template: src=index.html.j2, dest=/var/www/html
```
