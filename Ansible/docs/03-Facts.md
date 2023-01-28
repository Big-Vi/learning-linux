## Facts

Ansible collects all the information from the server such as process, os, dns config, memory, device info, volume, hostname, fqdn, date and time. These info known as facts. Ansible uses `setup` module for this.  

We have the option to disable gathering facts in several places.  

> Playbook  
  
`gather_facts: no`

> vi /etc/ansible/ansible.cfg  

	gathering = implicit


Setting in playbook overrides setting in /etc/ansible/ansible.cfg.  

implicit means gather facts. explicit means doesn't gather facts.  

`ansible -m setup localhost`  
