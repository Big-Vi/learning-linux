## Roles

Roles helps to develop and re-use Ansible playbooks and to organize the code.

Role folder structure:
- tasks
- vars
- defaults
- handlers
- templates


 To initialise the role directory structure.   
`ansible-galaxy init mysql`

To search for the role.  
`ansible-galaxy search mysql`

To use the Role from galaxy repo.  
`ansible-galaxy install mysql`

To install Roles in the current directory.  
`ansible-galaxy install mysql -p ./roles`

To list Roles.  
`ansible-galaxy list`

To view the Role configs.  
`ansible-config dump | grep ROLE`

```

- name: Install mysql
  hosts: db
  roles:
  - name: mysql
    vars:
      nginx_service_enabled: false
```

Roles directory can be placed under here.  
`/etc/ansible/roles`
Or
under the playbook project folder  
`<project-playbook>/roles/mysql`

For custom Roles folder, path can be modified at  
`/etc/ansible/ansible.cfg`

Roles can be uploaded to galaxy repo.  
