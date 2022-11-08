## Process management

`ps aux` - To display all processess formatted way  

`top` - To list and watch processess  

`ps <PID>` or `ps u <PID>`  

`ps -U <user>` or `ps u -U <user>` - To list processess start by user  

`pgrep -a <process-name>` - Search the process by name  

Niceness - Lower number is high priority/low niceness. Only root user can set negative niceness value.  

`nice -n <numeric-value> <process-name>`  

`renice -n <numeric-value> <PID>`  

`ps l` - To display long list of column which includes niceness column  

`ps lax`  

`ps fax` or `ps faux` -> f = forced - to display tree like display  

### Signal

`kill -L`- To list all the available signals on the system  
`kill -SIGHUP <PID>` - Signal hangup which kills the process when the terminal is closed. Opposite is nohup   
`kill -2 <pid>` - To send a SIGKILL  - Equivalent of Ctrl + c   
`kill -9 <pid>` - To send a SIGQUIT  - To forcefully terminate the process  
`kill --signal SIGQUIT <pid>`  
`pkill <PID>` - To kill processes gracefully  
`kill-all ssh` - To kill all processes of same type  

> Note: User can send signals to their own processes  

add ampersand character(&) at the end of command to make the app work in the background. e.g, `sleep 1000 &`
`Ctrl + z` - Pause/Suspend the app and exit if we're running the process in the foreground  

`fg <job-id>` - to bring the background app to foreground  
`bg <job-id>` - to resume the stopped app  
`jobs` - List all jobs  
Which files and directory particular app is using  

`lsof -p <PID>` ls=list of=openfile p=process  
`rpm -qf <file-path>` - To find what package owns the file	