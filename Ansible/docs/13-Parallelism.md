## Parallelism

Ansible runs tasks parellely in all servers. Ansible proceeds to next task if the previous task is complete in all other servers. This is linear strategy(default).  
With free strategy, Ansible runs tasks indedpendant of other servers task.  

```
- name: Free Strategy
  hosts: all
  strategy: free
```

Batch strategy is similar to linear but tasks exection can happen on number(batch) of servers.  

```
- name: Strategy
  hosts: host1, host2, host3
  serial: 2
```

Ansible can execute tasks in 5 servers at a time by default.  

> vi /etc/ansible/ansible.cfg

```
forks = 5
```
