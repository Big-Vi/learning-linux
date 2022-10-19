## Schedule tasks
### Cron

`crontab -e` - Edit the cron of current user  
`sudo crontab -e -u <user>` - Edit cron as other user  

`crontab -l` - To list cron  

`crontab -r` - Remove cron  
`sudo crontab -r -u <user>` - Remove cron of other user  

To setup cron differenly, use `/etc/cron.daily`  `/etc/cron.hourly`  `/etc/cron.monthly` `/etc/cron.weekly`  

### Anacron

`sudo vi /etc/anacrontab`  

number of days  |  delay in minutes  |  job-identifier  |  command
---             | ---                | ---              | ---
3               | 10                 | job-123          | /var/log/init.sh
@weekly         | 15                 | job-234          | /usr/bin/touch /var/file.txt

First job runs every 3 days with the delay of 10 minutes.  
Second job runs once in a week with the delay of 15 minutes.  

`anacron -T` - To test if anything wrong with anacron scheduled  
`anacron -n` - To run the jobs now.  
`anacron -n -f` - To force the jobs to run.  

`at 13:00`, `at '12:00 August 20 2022'`, `at 'now + 30 minutes'` and CTRL + d to save the job  
`atq` - To list scheduled jobs  
`at -c <job-number>` - To see what command at job contains  
`atrm <job-number>`  

`cat /var/log/cron` or `cat /var/log/syslog` - To view log of cron jobs  