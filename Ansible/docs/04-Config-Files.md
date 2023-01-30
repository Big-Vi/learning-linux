## Config files

Copy main config file(`/etc/ansible/ansible.cfg`) and paste into playbook folder in order to override default ansible configurations.  

Custom specify the ansible config file.  
`$ANSIBLE_CONFIG=/opt/ansible.cfg ansible-playbook playbook.yml`

Priority:
- via command line env
- local copy of ansible.cfg
- user home dir .ansible.cfg
- default ansible.cfg

How to pass env value: 
`ANSIBLE_GATHERING=explicit ansible-playbook playbook.yml  `

or

`export ANSIBLE_GATHERING=explicit`

To list all configs.  
`ansible-config list`

To view the current config file.  
`ansible-config view`

To view the current settings.  
`ansible-config dump`

