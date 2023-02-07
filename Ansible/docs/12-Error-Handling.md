## Error handling

`any_error_fatal` would stop all the execution if any task fails in any server.  
`max_fail_percentage` would ensure that if percentage of failute occurs, ansible would stop executing the remaining tasks.  
`ignore_errors` would ignore the error if the task fails.  
`failed_when` would fail the playbook if some condition met.  

```
- 
name: Error
hosts: all
any_errors_fatal: true
max_fail_percentage: 30
tasks:
- command: cat /var/log/syslog.log
  register: output
  failed_when: "'Err' in output.stdout"
- mail:
	to: <email>
	subject: <subject>
	body: <body>
  ignore_errors: yes
```
