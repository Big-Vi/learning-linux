## Playbooks

Playbooks are ansible's archestration language.  

To run ansible playbook  
`ansible-playbook <playbook-file-name>.yml`  

To dry run the Playbook  
`ansible-playbook <playbook-file-name>.yml --check`  

To start at particular task  
`ansible-playbook <playbook-file-name>.yml --start-at-task "<task-name>"`  

To run tasks which has that particular tags  
`ansible-playbook <playbook-file-name>.yml --tags "<tag-name>"`  
`ansible-playbook <playbook-file-name>.yml --skip-tags "<tag-name>"`  
