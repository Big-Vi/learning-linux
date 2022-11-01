## Physical storage partitions

`lsblk` - list block devices  

fdisk is text based command line utility to manage partitions.  
`fdisk --list` - To show partitions  
`fdisk <partition>`  

`cfdisk <partition>` - To manage partitions using text based graphical interface.  

`guid partition table` - gpt  

`swap` - partition type  

what is sector size and how to find it  

  

## Swap space

Swap is a space on a disk that can be used by Linux to move temp data from RAM when it's full.  

To create swap file  
`dd if=/dev/zero of=/swapfile bs=1M count=128 status=progress`  
`chomd 600 /swapfile` - Only for root user to read or write the swap file  
`mkswap --verbose /swapfile` - Prepare swap to be used  
`swapon /swapfile`  

`swapon --show`  
`swapoff /swapfile`  
`rm /swapfile`  


## File system

File systems types: xfs & ext4  
fstab stands for "file system table" - it is a list of filesystems that are to be mounted, their designated mount points  

`mkfs.xfs -L "LabelName" -i size=512 /dev/sdb1`  

L = Lable, i = size of inode(Files uses inode to store metadata)  

`sudo xfs_admin -l /dev/sdb1` - To find lable  

`mkfs.ext4 -L "LabelName" -N 40000 /dev/sdb2`  

N = number of inode  

`tune2fs -l /dev/sdb2` - To list attribute of filesystem  

  

## Mounting file systems

`ls /mnt/` - To list file system  

`mount /dev/vdb1 /mnt`  

`df -T` - To list mounted file system  

`umount /mnt`  

`vi /etc/fstab` - Define what should mount auto when system boots  

`blkid /dev/vdb1` - To get uuid  


## Mounting file systems on demand

`autofs` - Used for on demand mounting hence saves system resources.  
`dnf install autofs`  
`systemctl start/enable autofs.service`  

> NFS - Network File Sharing

`dnf install nfs-utils`  

`systemctl start/enable nfs-server.service`  

`vi /etc/exports` - config to let NFS know what directory it should share to the network  
	`/etc 127.0.0.1(ro)` - Remote server can access /etc(read only) directory from ip 127.0.0.1  

`vi /etc/auto.master` - To tell autofs to mount nfs file  
	`/shares/ /etc/auto.shares --timeout=400`  

`vi /etc/auto.shares`  
	`<directory-name> -fstype=auto 127.0.0.1:/etc` - Mount when the directory accessed  
	`/<directory-name> -fstype=auto 127.0.0.1:/etc` - With no parent directory  
	

## Basic file system

`findmnt`  

`findmnt -t <file-system>` - Show only particular mounted file system  

`mount -o ro /dev/vdba /mnt` - When mounting block device, read only option can be supplied.  

`mount -o remount,rw,noexec,nosuid /dev/vdba /mnt`  - To overwrite already mounted block device, use remount option.  


## LVM storage

> LVM = Logical Volume Manager  

`dnf install lvm2`  

> PV = Physical Volume  

`lvmdiskscan`  - To see what pv's available to use

`pvcreate /dev/sdc /dev/sdd`  

`pvremove /dev/sde`  

`pvs`  

> VG = Volume Group  

`vgcreate <group-name> /dev/sdc /dev/sdd`  

`vgextend <volume-group> /dev/sde`  - To expand the storage by adding another PV into volume group.

`vgs`  

`vgreduce <volume-group> /dev/sde`  

> LV=logical volume  - Partition for LVM

`lvcreate --size 2G --name <partition-name> <volume-group>`  

`lvs`  

`lvresize --extents 100%VG <volume-name>/<partition-name>`  

`lvresize --size 2G <volume-name>/<partition-name>`  

`lvdisplay`  

`mkfs.xfs /dev/<volume-name>/<partition-name>`  - Add file system on LV  

`lvcreate --resizefs 3G <volume-group>/<partition-name>` - To resize if file system installed on LV   


## Encrypted storage

Plain mode:  

`cryptsetup --verify-passphrase open --type plain /dev/vde <secure-disk-name>`  

open - open it for read and write.  

`mkfs.xfs /dev/mapper/<secure-disk-name>` - Write xfs file system  

`mount /dev/mapper/<secure-disk-name> /mnt`  

`cryptsetup close <secure-disk-name>` - Close it once the file system in established  

Luks mode:  

> luks = Linux Unified Key Setup   

`cryptsetup luksFormat /dev/vde` - Format first  

`cryptsetup luksChangeKey /dev/vde`  

`cryptsetup open /dev/vde <secure-disk-name>`  


## RAID devices

> RAID = Redundant Array of Independent Disks  

Adding storage disk as single unit.  

level0 -> striped array - not redundant  

leve1 -> mirrored array - redundant  

level5 -> min 3 disk - redundant - parity feature(backup data used to rebuild data)  

level6 -> min 4 disk - redundant  

level10 -> RAID 1+0 - more storage space + mirrored feature  

> mdadm - Multiple Devices ADMinistration  

`mdadm --create /dev/md0 --level=0 --raid-devices=3 /dev/vdc /dev/vdd /dev/vde`  

`mdadm --stop /dev/md0`  

`mdadm --zero-superblock /dev/vdc /dev/vdd /dev/vde` - To prevent linux to automatically reassemble into array when boot time. wipe the data in the superblock  

`mdadm --create /dev/md0 --level=1 --raid-devices=2 /dev/vdc /dev/vdd --spare-devices=1 /dev/vde`  

`mdadm --manage /dev/md0 --add /dev/vde` - To add spare to an existing array  
`mdadm --manage /dev/md0 --remove /dev/vde` - To remove device from an array  
`/proc/mdstat` - To list raid devices  


## Advanced file system permission

> setfac - Set File Access Control - For fine grained control  

`setfacl --modify user:<user-name>:rw <file-name>` - To give permission to one particular user  

`getfacl <file-name>` - to see acl  
mask = max permission the file can have  

`setfacl --modify mask:r <file-name>` - mask limits the permission. effective indicates the real permission  

`setfacl --modify group:<group-name>:rw <file-name>`  

`setfacl --modify user:<user-name>:--- <file-name>` - to ban user not to do anything  

`setfacl --remove user:<user-name> <file-name>`  

`setfacl --remove group:<group-name> <file-name>`  

`setfacl --recursive -m user:<user-name>:rwx <dir-name>/`  

`chattr +a <file-name>` - to add an "append only" attribute to file which enables user to add data not to edit the existing one.  

`chattr -a <file-name>`  

`chattr +i <file-name>` - i=immutable attribute  

`lsattr <file-name>`  


## User & Group disk quotas for filesystems

`dnf install quota`  

`vi /etc/fstab`  
	/dev/vdb1 /mybackups xfs defaults,usrquota,grpquota 0 2 - To enable quota  

`quotacheck --create-file --user --group /dev/vdb2` - for ext4 file system  

`quotaon /mnt`  

`fallocate --length 100M /<dir>/<dir>/<file>` - To create 100M size file  

`edquota --user <user>` - Edit quota for user  

`edquota --group <group>`  

`quota --user <user>`  

`quota --edit-period` - to edit grace period  

`setquota -g <group> 20M 100M 0 0 /dev/vdc1` - Set quota for particular group on device with 20M soft limit and 100M hard limit.    