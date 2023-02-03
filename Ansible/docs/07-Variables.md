## Variables

Variables can be added into playbook.yml file, inventory file or separate file as `key: value`. Variables are referenced using Jinja2 templating.  

Precedence:
- Role Defaults
- Group var
- Host var
- Host Facts
- Playbook var
- ...

- ...

- Command line var



To capture the output of the tasks, register the variables.

```
tasks:

- shell: cat /etc/hosts
  register: result

- debug:
  var: result
```

Register variables are host scoped and available through the Playbook execution.  
