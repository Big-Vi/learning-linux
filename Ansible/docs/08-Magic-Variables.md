## Magic Variables

hostvars - To get variables defined in another host.  

E.g,

`{{ hostvars['<hostname>'].ansible_host }}`

`groups['<groupname>']`
`group_names`
`inventory_hostname`

To output the variable  
`ansible-playbook -i inventory playbook.yml -v` 
