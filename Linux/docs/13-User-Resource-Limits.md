## User resource limits

It's important to limit how many processes users can create.  
`ulimit -u` - To view the number of processes users can create  
`ulimit -u <number>` - To set the number of processes(nproc) users can create  
Setting the nproc value by this way is only temporary. In order to set it permanently, edit `/etc/security/limits.conf`  

`ulimit -aS` - To show all the Soft limits for the user  
`ulimit -aH` - To show all the Hard limits for the user  


domain   |   type   |   item   |   value
---      | ---      | ---      | ---     |
@group   | soft     | nproc    | 30
user     | hard     | cpu      | 1
*        | -        | fsize    | 1024

**Type**  
- soft  
- hard  
- '-'. hyphen enforces both hard and soft limits together.  