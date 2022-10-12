## User accounts

To Create new user with default settings specified on `/etc/default/useradd` file and with configuration specified on `/etc/login.defs`
`sudo useradd <username>`

To create user home directory as `/home/<username>`
`sudo useradd -m <username>` 

To create custom user directory
`sudo useradd -m -d /opt/<username> <username>`

To specify custom shell and user directory
`useradd --shell /bin/othershell --home-dir /home/otherdir/ <user-name>`

To set the password for the new user
`sudo passwd <username>`

To create user with specific ID
`useradd --uid <number> <user-name>`

To verify the userID
`id -u <user-name>`

User account creates user, group, home directory, bin/bash, config files under `/etc/skel`. To view those defaults run below command.
`useradd --defaults` or `useradd -D`
`useradd -D --shell /bin/bash` - To change the default shell

To delete an user
`sudo userdel <user-name>`
`sudo userdel --remove <user-name>` or `userdel -r <user-name>`

To list all users
`cat /etc/passwd`

 To list user with group
 `ls -l /home/`

To list user with group with ID
`ls -ln /home/`

To find current user
`id`
`whoami`

To find user id
`id -u www-data`

To create system user
`useradd --system <system-account-name>`

Move content of the home directory to the new location
`usermod --home /home/otherdir --move-home <user-name>` 
or
`usermod -d /home/otherdir -m <user-name>`

To change the name of the existing user
`usermod --login <new-username> <old-username>`

To prevent/not prevent user from logging in
`usermod --lock/--unlock <user-name>`

To set account expirydate
`usermod --expiredate <yyyy-mm-dd> <user-name>`

To set date of last password change to today 
`chage --lastday 0 <user-name>`

To set password change to never
`chage --lastday -1 <user-name>`

To change every 30 days
`chage --maxdays 30 <user-name>`

To set never expire password
`chage --maxdays -1 <user-name>`

To see when password expires
`chage --list <user-name>`

## Groups

To add group
`groupadd <group-name>`

To find members of the group
`getent group <group-name`

To list groups
`cat /etc/group`

To add the user to the group
`gpasswd --add <user-name> <group-name>`

To find which group user belongs to
`groups <user-name>`

To delete the user from the group
`gpasswd --delete <user-name> <group-name>`

To add user to the group
`usermod --gid <group-name> <user-name>`

To change the name of existing group
`groupmod --new-name <new-group> <old-group>`

To delete the group
`groupdel <group-name>`

## Environment profile

`printenv`

`history`

`echo $<env-key>`

`vi /etc/environment`

`/etc/profile.d/<file>.sh` - Runs every time user login


## Template user environment

If you create files under `/etc/skel/`, those files gets created when new user added.


## Root account

`su -` or `sudo --login`
`logout`
`sudo passwd root`
`sudo passwd --unlock root`
`sudo passwd --lock root`


## Configure PAM

PAM - Pluggable Authentication Module

`/etc/pam.d` - Config files