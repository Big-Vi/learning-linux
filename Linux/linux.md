# learning-linux

# Essential commands

## System documentation

    man <command> to access manual

    apropos <command-keyword> - helper funtion to find commands
    apropos -s 1,8 <command-keyword> - s indicates section 
    system admistration command saved in section 8

    run `sudo mandb` to create db before running apropos

    tab suggestion and auto completion
    tab twice to see the options

## File permission
    chgrp <group-name> <file/dir>
    groups
    chown <owner-name> <file/dir>
    chown <owner-name>:<group-name> <file/dir>

    chmod <permission> <file/dir>
    user      u+  u-  u=   u+rwx u-rwx u=rwx
    group     g+  g-  g=   g+rwx g-rwx g=rwx
    others    o+  o-  o=   o+rwx o-rwx o=rwx

    chmod u+rw, g=r, o= <file/dir>
    Use octal value
    rw- | r-- | ---
    110 | 100 | 000

    Binary | Decimal        Permission  | Value
    000    | 0              r           | 4
    001    | 1              w           | 2
    010    | 2              x           | 1
    011    | 3
    100    | 4
    101    | 5
    110    | 6
    111    | 7

    Binary to decimal
    110 = (1x2<sup>2</sup> + 1x2<sup>1</sup> + 0x2<sup>0</sup>)

    chmod 640 <file/dir>

## SUID, SGID & Sticky Bit
    SUID - Set User Identification
        If this is set, it's executed as user id of the owner of the file instead of user id of the person executing it.
    chmod 4640 <file-name>

    SGID - Set Group Identification
        chmod 2640 <file-name>

    find . -perm /4000 - To find SUID file
    find . -perm /2000 - To find SGID file
    find . -perm /6000 - To find both

    Sticky Bit 
        chmod 1640 <dir-name>

## Search files
    find <path-to-dir> <search-param> <file-name>

    Search param list
        - mmin - modified minute
        - perm - permission
        - name - name of the file
        - mtime - modified in days
        - cmin - change minute
        - size 

    Multiple search params - AND, OR, NOT operator
    e.g 
        find -size +512k -o -name "f*" - OR
        find -size +512k -name "f*" - AND
        find -not -size +512k - NOT
        find \! -size +512k - NOT

        
    Modified time is related to data of the file
    Change time is related to metadata of the file

## Compare & manipulate file content
    cat vs tac
    tail -n 20 /var/log/error.log
    head -n 20 /var/log/error.log
    sed - stream editor
    sed -i 's/<search-name>/<replace-with>/g' error.log - i means in-place
    cut -d ' ' -f 1 error.log - d = delimiter, f = field 
    uniq error.log - Only removes the duplicate which is adjacent
    sort error.log | uniq - To let you remove duplicate
    diff -c <file-1> <file-2> - c = context
    diff -y <file-1> <file-2> or sdiff <file-1> <file-2> 

## Pagers & Vim
    less & more

## Grep
    grep <options> <search-pattern> file
    options
    i - ignore case
    r - recursive
    v - invert-match
    w - only match words
    o - only-matching

## Regular expression
    ^  -  The line begins with             - ^RAM
    $  -  The line ends with               - CPU$
    .  -  Match any one character          - c.u
    \  -  Escaping the special characters
    *  -  Match the previous element 0     - Rum* = Ru, Rum, Rumm
          or more times
    +  -  Match the previous element 1     - Rum+ = Rum, Rumm
          or more times
    {} -  Previous element can exist       - Rum{2,} = Rumm, Rummmm - Atleast 2 previous element
          number of times                  - Rum{,2} = Ru, Rum, Rumm - Most 2 prvious element
                                           - Rum{2} = Rumm - Exactly previous element
    ?  -  Make the previos element         
          optional 
    |  -  Match one thing or other
    [] -  Range or sets                    - R[au]m
    () -  Subexpressions                   - R([a-z][0-9])*m
    [^] - Negated ranges or sets           - http[^s] = http, R[^a-d]m


    Grep has basic regular expression(?, {, |, (, ), +) buit in. So in order to use + operator, escape it using backslash(\). Use extended regex to avoid converting regular operator to regular plus or vice versa.

## Extended Regex
    grep -E 'Ru+' /var/ or egrep 'Ru+' /var/

## Archive
    Archive -> Compress -> Backup
    tar --list --file <filename>.tar or
    tar -tf <filename>.tar or
    tar tf <filename>.tar

    tar --create --file <filename>.tar <file> or tar cf <filename>.tar <file>

    tar --append --file <filename>.tar <file> or
    tar rf <filename>.tar <file>

    tar --extract --file <filename>.tar --directory /tmp/ or tar xf <filename>.tar -C /tmp/

## Compress
    gzip <filename>
    bzip2 <filename>
    xz <filename>
    zip -r <filename>.zip <file>/

    gunzip <filename>.gz or gzip --decompress <filename>.gz
    bunzip <filename>.gz or bunzip --decompress <filename>.gz
    unxz <filename>.gz or unxz --decompress <filename>.gz
    unzip <filename>.zip

    options:
    --keep - Original files are untouched

    To archive and compress
    tar --create --gzip --file <filename>.tar.gz <file> or
    tar czf <filename>.tar.gz <file>
    tar --create --bzip2 --file <filename>.tar.bz2 <file> or
    tar cjf <filename>.tar.bz2 <file>
    tar --create --xz --file <filename>.tar.xz <file> or
    tar cjf <filename>.tar.xz <file>

    tar autocompress
    tar --create --auto-compress --file <filename>.tar.gz <file> or
    tar caf <filename>.xz <file>

## Backup
    rsync - remote synchronization
    rsync -a <local-dir> <remote-user-name>@<remote-ip>:/<remote-dir-path>

    Disk imaging
    sudo dd if=/dev/vda of=diskimage.raw bs=1M status=progress
    if - input file, of - output file, bs - block size

## Input & output redirection
    Redirect input:
        date <file.txt

    Redirect ouput:
        date > file.txt or date 1> file.txt 
        date 2> error.txt - to send error ouput
        date >> file.txt - Not to overwrite file
        2>/dev/null - Discard error message
        grep -r '^CPU' /etc/ all_output.txt 2>&1 -  To redirect output & error to same file.
    Redirect input:
        sort file.txt
        sendemail someone@gmail.com < content.txt
    Herdoc & Here String:
        sort <<EOF
        2
        3
        6
        7
        EOF

        bc <<<2+6+8
    Piping:
        grep -v '^#' /etc/apache2/apache2.conf | sort | column -t
    
# Operations of running system
## Boot, reboot & shutdown
    systemctl reboot - system control
    systemctl poweroff
    shutdown 21:00 or shutdown +15
    shutdown -r 21:00 or shutdown -r +15 'Scheduled restart' - shutdown and reboot with wall message

## Boot into different operating modes
    systemctl get-default
    systemctl set-default multi-user.target
    systemctl isolate graphical.target

## Bootloader
    GRUB - Grand Unified Boot Loader
    lsblk

## Scripting to automate system admin tasks
    man systemd.service
    systemctl cat <service-name>.service
    systemctl edit --full <service-name>.service
    sytemctl start/stop/restart/reload/status/revert <service-name>.service
    systemctl reload-or-restart <service-name>.service
    systemctl disable <service-name>.service
    systemctl is-enabled <service-name>.service
    systemctl enable --now <service-name>.service - enable and start
    systemctl mask <service-name>.service - Prevent the service from starting accidentally.
    systemctl list-units --type service --all

## Process management
    ps aux - To display all processess formatted way
    top - To list and watch processess
    ps <PID> or ps u <PID>
    ps -U <user> or ps u -U <user> - To list processess start by user
    pgrep -a <process-name> - Search the process by name

    Niceness - Lower number is high priority/low niceness. Only root user can set negative niceness value.
    nice -n <numeric-value> <process-name>
    renice -n <numeric-value> <PID> 
    ps l - To display long list of column which includes niceness column 
    ps lax
    ps fax or ps faux -> f = forced - to display tree like display

    Signal
    kill -L
    kill -SIGHUP <PID>
    pkill <PID> - To kill processes gracefully
    cntl + z - Pause the app(e.g vim) and exit

    Note: User can send signals to their own processes

    fg - foreground - Brings the paused app to foreground
    add & at the end of command to make the app work in the background
    jobs to check the apps that running in background
    fg 1 - to bring the background app to foreground
    bg 1 - to resume the stopped app

    Which files and dir particular app is using
        lsof -p <PID> -> ls=list of=openfile p=process 

## System log files
    tail -F /var/log/syslog - Follow mode to watch log file
    journalctl <command> - Program to collect log data smarter way. Only show log generated by particular command.
        e.g - journalctl /bin/sudo
    journalctl -u <service-name> 
        e.g - which ssh, journalctl -u sshd.service
    journalctl -e - to get to end of the output
    journalctl -f - follow mode
    journalctl -p - to get priority list
    journalctl -p err - priority
    journalctl -S 22:00 or journalctl -S '2020-11-3 12:04:22' - display logs only after this time or date. s=synth
    journalctl -S 22:00 -U 23:00 - display logs between time. u=until
    journalctl -b 0 - To display logs since the current boot
    journalctl -b -1 - to display logs since the previous boot
    last and lastlog

## Schedule tasks
    crontab -e - Edit the cron of current user
    crontab -l - To list cron
    sudo crontab -e -u <user> - Edit cron as other user
    crontab -r - Remove cron
    sudo crontab -r -u <user> - Remove cron of other user

    sudo vi /etc/anacrontab
    #period in days |  delay in minutes |  job-identifier  |  command
    3                  10                  job-123            /var/log/init.sh | systemd-cat --identifier=job-123
    Piping output to systemd utility.
    anacron -T - To test
    anacron -n - To run the jobs now.
    anacron -n -f - To force the jobs to run.


    at 13:00, at '12:00 August 20 2022', at 'now + 30 minutes' and CTRL + d to save the job
    atq - To list scheduled jobs
    at -C <job-number> - To see what command at job contains
    atrm <job-number>

    cat /var/log/cron or cat /var/log/syslog - To view log of cron jobs

## Software updates - CentOS
    dnf check-upgrade
    dnf upgrade

## Manage softwares - CentOS
    dnf repolist -v
    dnf repolist --all
    dnf config-manager --enable <package-name>
    dnf config-manager --add-repo <git-link> - For external repo
    dnf search <keyword>
    dnf info/install/reinstall/remove <package-name>
    dnf group list
    dnf group install --with-optional <group-name>
    dnf group remove <group-name>
    dnf group list --hidden
    dnf autoremove - To remove dependencies of main package
    dnf history

    dnf provides <file-name> - To find which package install the component.
        e.g dnf provides docker
    dnf repoquery --list <package> - To locate files installed by that package

## Manage Resources
    df - disk free utility
    df -h - To display in human readable format
    du - disk usage
    du -sh /bin/ - s=summaries h=human readable format
    free -h, free --mega - To see how memory is used
    uptime - Load of CPU
    lscpu - To see specific CPU usage
    lspci - Other hardware usage
    xfs_repair -v -f -p /dev/vdb1 - To verify -> f=force, p=preen mode
    systemctl list-dependencies

## Kernel
    sysctl -a
    sysctl -w <key>=<value> - Non-persistent
    sysctl <key-name>
    vi /etc/sysctl.d/*.conf - Persistent
    sysctl -p /etc/sysctl.d/*.conf - To apply the changes

## SELinux/AppArmor
    SELinux - Security module
    ls -Z 
    user/role/type/level
    ps axZ
    id -Z
    semanage login -l
    semanage user -l
    getenforce
    setenforce to change the selinux status
    chcon to change security context

## User accounts
    useradd <user-name>
    useradd --uid <number> <user-name> - To add user id
    user account create user, group, home directory, bin/bash, config files under /etc/skel
    useradd --defaults or useradd -D
    cat /etc/login.defs
    passwd <user-name>
    userdel <user-name>
    userdel --remove <user-name> or userdel -r <user-name>
    useradd --shell /bin/othershell --home-dir /home/otherdir/ <user-name>
    cat /etc/passwd
    ls -l /home/ - To list user with group 
    ls -ln /home/ - To list user with group with ID
    id
    whoami
    useradd --system <system-account-name>
    usermod --home /home/otherdir --move-home <user-name> or 
    usermod -d /home/otherdir -m <user-name>
    usermod --login <new-username> <old-username>
    usermod --shell /bin/othershell <user-name> 
    usermod --lock/--unlock <user-name>
    usermod --expiredate <yyyy-mm-dd> <user-name>
    chage --lastday 0 <user-name> -> chage - change age
    chage --lastday -1 <user-name> - To cancel
    chage --maxdays 30 <user-name> - To change every 30 days
    chage --maxddays -1 <user-name> - Never expired password
    chage --list <user-name> - To see when pw expires

## Groups
    groupadd <group-name>
    gpasswd --add <user-name> <group-name>
    groups <group-name>
    gpasswd --delete <user-name> <group-name>
    usermod --gid <group-name> <user-name>
    groupmod --new-name <new-gropu> <old-group>
    groupdel <group-name>

## Environment profile
    printenv
    history
    echo $<env-key>
    vi /etc/environment
    vi /etc/profile.d/<file>.sh - Runs every time user login

## Template user environment
    vi /etc/skel/README - This creates README file when user added

## User resouce limits
    vi /etc/security/limits.conf
    domain -> type -> item -> value
    @group    soft    nproc   30
    user      hard    cpu     1
    *         -       fsize   1024

    -iu <user> - login as user
    ulimit -a
    
## User previleges
    visudo - Fine permission control
    user/group host=(run as user) commandlist
    e.g jhon  ALL=(ALL) NOPASSWD: ALL
        %developer ALL=(ALL) ALL

## Root account
    su - or sudo --login
    logout
    sudo passwd root
    sudo passwd --unlock root
    sudo passwd --lock root

## Configure PAM
    PAM - Pluggable Authentication Module
    /etc/pam.d - Config files

## Networking configure
    ip link show or ip l - Name of network interface
    ip address show or ip a
    lshw - For detailed networ k interface details
    cidr - classless inter domain routing
    ip route show or ip r
    /etc/resolv.conf - To see dns resolver conf
    /etc/sysconfig/network-scripts - Linux dynamically configures network based on the config file in here
    nmtui - network manager text user interface
    nmcli device reapply <network-device-name>
    /etc/hosts - Static resolution config

## Network services
    systemctl status NetworkManager.service
    dnf install NetworkManager
    systemctl start/enable NetworkManager.service
    nmcli connection show
    nmcli connection modify <connection-name> autoconnect yes

    ss -ltunp -> l=listening, t=TCP connections, u=UDP connections, n=numeric values, p=processes

## Packer filtering
    firewall-cmd get-default-zone
    firewall-cmd --set-default-zone=<zone-name>
    firewall-cmd --list-all
    firewall-cmd --info-service=<service-name>
    firewall-cmd --add-service=<serice-name> or firewall-cmd --add-port=80/tcp
    firewall-cmd --remove-service=<service-name> or firewall-cmd --remove-port=80/tcp
    firewall-cmd --add-source/--remove-source=<ip-address> --zone=trusted
    firewall-cmd --get-active-zones
    firewall-cmd --runtime-to-permanent - to apply existing temp rules
    firewall-cmd --add-port=80/tcp --permanent - to apply rule permanent using inline command

## Statically route IP traffic
    ip route add <network-route-out-ip> via <network-route-in-ip> <device-name> <interface-name>
    ip route del <network-route-out-ip>
    ip route add/delete default via <network-route-in-ip>
    nmcli connection modify <network-name> +ipv4.routes "<network-route-out-ip> <network-route-in-ip>" - To permanently apply
    nmcli device reapply <network-name>
    nmcli connection modify <network-name> -ipv4.routes "<network-route-out-ip> <network-route-in-ip>" - To remove 

## Synchronize time - Network peers
    centos uses chrony daemon
    timedatectl
    timedatectl set-timezone <region>/<city>
    timedatectl list-timezones
    dnf install chrony
    systemctl start/enable chronyd.service
    systemctl set-ntp true

## DNS server cache
    bind - DNS server app
    dnf install bind bind-utils
    /etc/named.conf
        listen-on port 80 {any}/{127.0.0.1; 192.168.2.0;};
        allow-query {any}/{localhost;192.168.1.0/24;};
        recursion yes;
    firewall-cmd --add-service=<serive-name> - Add dns service to firewall to external connection

## DNS zone
    zone "website.com" IN {
        type master;
        file "website.com.zone";
    }
    /var/named - Bind keeps all the fiels here
    restart named service to apply bind

## Email aliases - configure
    dnf install postfix(email server)
    sendmail <user>@localhost <<< "Hello"
    /var/spool/mail/<user> - Email stored here
    vi /etc/aliases - To add email aliases
    contact: <user>,<user2>,<user3>
    newaliases - To let postfix know about the change
    contact: <user>@otherwebsite.com - To store externally

## IMAP & IMAPS
    Internet message access protocol secure(ssl)
    def install dovecot
    /etc/dovecot/ - Config files
    /etc/devecot/conf.d - Config are grouped here
    /etc/devecot/conf.d/10-master.conf - To change ports
    /etc/dovecot/conf.d/10-mail.conf - to change the location where mail stored
    /etc/dovecot/conf.d/1-ssl.conf - To change tls
    
## SSH server & client
    /etc/ssh/sshd_config - Server config file
    /etc/ssh/ssh_config - Client config file
    ssh-keygen
    ssh-copy-id <user-name>@<ip> - To stroe ssh public key
    .ssh/authorized_keys - To store all pubic keys of users in server
    ssh-keygen -R <ip> - To remove fingerprint
    rm known_hosts - To clear all fingerprints
    /etc/ssh/ssh_config.d/99-<name>.conf - To add default value for ssh client

## HTTP proxy server
    squid - Proxy deamon
    /etc/squid/squid.conf - Config
    acl <name> dstdomain .<web-address> - Block domain and sub domains
    http_access deny <name>

## Configure HTTP server
    dnf install httpd
    firewall-cmd --add-service=http
    firewall-cmd --add-service=https
    dnf install mod_ssl - To enable ssl
    httpd -m - To list modules
    man httpd.conf - Manual for how to configure 
    /etc/httpd/ - Config files
    /etc/httpd/conf/httpd.conf - Primary config file
    /etc/httpd/conf.d/two-website.conf - To add multiple website
    apachectl configtest - To test added virtual host
    /etc/httpd/conf.d/ssl.conf
    /etc/httpd/conf.modules.d - To enable/disable modules
    mpm module vs pre fork module

## Configure HTTP server log files
    /etc/httpd/conf/httpd.conf - To find where log files get stored
    add CustomLog, ErrorLog to VirtualHost to have separate logs
    
## Restrict access to a web page
    mv /etc/httpd/conf.d/two-website.conf /etc/httpd/conf.d/two-website.conf.disabled
    "Options Indexes FollowSymlinks" Indexes option let user browse folder if there's no index file
    FollowSymlinks - Allow httpd to follow symlink
    AllowOverride - Enable/Disable the .htaccess file
    Require ip <address1> <address2> - To allow connection from these ip's.
    htpasswd -c /etc/httpd/passwords <user-name> - To protect the file with password
    c option creates file if it doesn't exist
    htpasswd -D /etc/httpd/passwords <user-name> - To delete password for that user

## Database server
    dnf install mariadb-server
    start, enable and add the servie to firewall.
    mysql_secure_installation - To secure the DB
    mysql -u root -p
    /etc/my.cnf.d/mariadb-server.cnf - Config file

## Manage & configure containers
    dnf install docker
    /etc/containers/registries.conf
    nc localhost 8080 -> GET /

## Manage & configure virtual machines
    qemu - quick emulator
    kvm - kernal based virtual machine
    virsh - manage virtual machine on command line
    dnf install libvirt qemu-kvm
    vi testmachine.xml
        <domain type="qemu>
            <name>TestMachine</name>
            <memory unit="GiB>1</memory>
            <vcpu>1</vcpu>
            <os>
                <type arch="x86_64>hvm</type>
            </os>
        </domain>
    virsh define testmachine.xml
    virsh destroy testmachine.xml
    virsh undefine testmachine.xml
    autostart - To start when server boots
    autostart --disable - Not to autostart
    dominfo - To explore the assigned resources of vm
    setvcpus - To change cpu
    virsh help <command>
    virsh setvcpus TestMachine 2 --config --maximum

## Physical storage partitions
    lsblk - list block devices
    fdisk --list /dev/sda - To show partition
    cfdisk /dev/sdb - To create/delete partition
    guid partition table - gpt
    swap - partition type
    sector size

## Swap space
    Linux moves temp data from RAM
    swapon --show
    mkswap --verbose <partition> - Prepare swap to be used
    swapoff <partition>
    dd if=/dev/zero of=/swap bs=1M count=128 status=progress - to create swap file
    chomd 600 /swap - Only for user access

## File system
    xfs, ext4 - File systems
    mkfs.xfs -L "LabelName" -i size=512 /dev/sdb1
    L = Lable, i = size of inode
    xfs
    sudo xfs_admin -l /dev/sdb1 - To find lable

    mkfs.ext4 -L "LabelName" -N 40000 /dev/sdb2
    N = number of inode
    tune2fs -l /dev/sdb2 - To list attribute of filesystem

## Mounting file systems
    ls /mnt/ - To list file system
    mount /dev/vdb1/ /mnt/
    umount /mnt/
    vi /etc/fstab - Define what should mount auto when system boots
    blkid /dev/vdb1 - To get uuid

## Mounting file systems on demand
    autofs - Used for on demand mounting
    nfs - network file sharing
    dnf install nfs-utils
    systemctl start/enable nfs-server.service
    vi /etc/exports
        /etc 127.0.0.1(ro)
    Remote server can access /etc(read only) directory from ip 127.0.0.1
    vi /etc/auto.master - To tell autofs to mount nfs file 
        /shares/ /etc/auto.shares --timeout=400
        /- /etc/auto.shares -> No parent directory
    vi /etc/auto.shares
        <directory-name> -fstype=auto 127.0.01:/etc
        /<directory-name> -fstype=auto 127.0.01:/etc - With no parent directory
    
## Basic file system
    findmnt
    findmnt -t xfs
    mount -o ro /dev/vdb2 /mnt
    -o ro,noexec,nosuid
    -o remount,rw

## LVM storage
    Logical volume manager
    dnf install lvm2
    pv=physical volume
    lvmdiskscan
    pvcreate /dev/sdc /dev/sdd
    pvremove /dev/sde
    pvs

    vg=volume group
    vgcreate <group-name> /dev/sdc /dev/sdd
    vgextend <volume-group> /dev/sde
    vgs
    vgreduce <volume-group> /dev/sde

    lv=logical volume
    lvcreate --size 2G --name <partition-name> <volume-group>
    lvs
    lvresize --extents 100%VG <colume-name>/<partition-name>
    lvresize --size 2G <volume-name>/<partition-name>
    lvdisplay
    mkfs.xfs /dev/<volume-name>/<partition-name>
    lvcreate --resizefs 3G <volume-group>/<partition-name> - To resize if file system installed on lv
    
    pe=physical extent

## Encrypted storage
    linux unified key setup - luks
    plain mode
    cryptsetup --verify-passphrase open --type plain /dev/vde <secure-disk-name>
    open action - open it for read and write
    mkfs.xfs /dev/mapper/<secure-disk-name>
    mount /dev/mapper/<secure-disk-name> /mnt
    cryptsetup close <secure-disk-name>

    cryptsetup luksFormat /dev/vde
    cryptsetup lukschangekey /dev/vde
    cryptsetup open /dev/vde <secure-disk-name>

## RAID devices
    redundant array of independent disks
    level0 - striped array - not redundant
    leve1 - mirrored array - redundant
    level5 - min 3 disk - redundant - parity feature
    level6 - min 4 disk - redundant
    level10 - RAID 1+0 - more storage space + mirrored feature
    mdadm - multiple devices administration
    mdadm --create /dev/md0 --level=0 --raid-devices=3 /dev/vdc /dev/vdd /dev/vde
    mdadm --stop /dev/md0
    mdadm --zero-superblock /dev/vdc /dev/vdd /dev/vde - To prevent linux to automatically reassemble into array when boot time. wipe the data in the superblock
    mdadm --create /dev/md0 --level=1 --raid-devices=2 /dev/vdc /dev/vdd --spare-devices=1 /dev/vde
    mdadm --manage /dev/md0 --add /dev/vde - to add spare to an existing array
    /proc/mdstat - to see raid devices

## Advanced file system permission
    setfac - set file access control
    setfacl --modify user:<user-name>:rw <file-name>
    getfacl <file-name> - to see acl
    setfacl --modify mask:r <file-name> - mask limits the permission
    setfacl --modify group:<group-name>:rw <file-name>
    setfacl --modify user:<user-name>:--- <file-name> - to ban user not to do anything
    setfacl --remove user:<user-name> <file-name>
    setfacl --remove group:<group-name> <file-name>
    setfacl --recursive -m user:<user-name>:rwx <dir-name>/

    chattr +a <file-name> - to add an append only attribute to file
    chattr -a <file-name>
    chattr +i <file-name> - i=immutable
    lsattr <file-name>

## User & Group disk quotas for filesystems
    dnf install quota
    vi /etc/fstab
        /dev/vdb1 /mybackups xfs defaults,usrquota,grpquota 0 2
    quotacheck --create-file --user --group /dev/vdb2 - for ext4 file system
    quotaon /mnt
    fallocate --length 100M /<dir>/<dir>/<file>
    edquota --user <user>
    edquota --group <group>
    quota --user <user>
    quota --edit-period - to edit grace period









    





