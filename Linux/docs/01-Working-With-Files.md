## Working with files

ls  
   -  `-l` To see long list to get detailed information  
   - `-a` To get hidden files  
   -  `-la` To get long list format with hidden files  
   - `-lah` To get size of the file too  

pwd  
- Print working directory  

`cp <source><destination>` - `cp file.txt directory/`  
- `-r` Copy recursively  
	`cp FromDirectory/ ToDirectory/`  

`mv <source> <destination>`  
> mv command don't need to use -r flag to move all files.  

`rm <file>`  - To remove file  

`rm -r <directory>` - To remove files recursively  

`rm -rf <directory>` - To remove files forcefully if there's any write protected files present in the directory.  

### Hard link
Inode - Inode is Index node and it stores metadata of file or directory.  

        `stat <file/directory>` - To check inode number  
        
Hard link is mirror copy of the file it points to inode of the original file. It works only on the same filesystem.  

        `ln <path-to-target-file> <path-to-link-file>`  

### Soft link
Symbolic link or soft link is just a short cut which points to the original file. If original file is removed, soft link won't work.  It works on files and directories of different filesystem too.  

        `ln -s <path-to-target-file> <path-to-link-file>`  
        `readlink <file-name>` - to find soft link target
        
## File permission

To change the file or directory to another group  
`chgrp <group-name> <file/dir>`

To change the file or directory to another user  
`chown <owner-name> <file/dir>`

To change the file or directory to another group/user  
`chown <owner-name>:<group-name> <file/dir>`

To set permission  
`chmod <permission> <file/dir>`

|                |options                        |example                      |
|----------------|-------------------------------|-----------------------------|
|user            |u+  u-  u=                     | u+rwx u-rwx u=rwx           |
|group           |g+  g-  g=                     | g+rwx g-rwx g=rwx           |
|others          |o+  o-  o=                     | o+rwx o-rwx o=rwx           |


`chmod u+rw, g=r, o= <file/dir>`

#### Using octal value

| rw- | r-- | --- |
|-----|---- | ----|
| 110 | 100 | 000 |

<br/>

| Binary | Decimal |                          
| ------- | --------- |
| 000 | 0 |
| 001 | 1 |
| 010 | 2 |
| 011 | 3 | 
| 100 | 4 |
| 101 | 5 |
| 110 | 6 |
| 111 | 7 |

<br/>

| Permission | Value |
| -----      | ---   |
r | 4
w | 2
x | 1


#### Binary to decimal conversion

110 = (1x2<sup>2</sup> + 1x2<sup>1</sup> + 0x2<sup>0</sup>)  

`chmod 640 <file/dir>` 



## SUID, SGID & Sticky Bit

#### SUID - Set User Identification

If this is set, it's executed as user id of the owner of the file instead of user id of the person executing it.  

example: using sudo command(/bin/su),  password file(/usr/bin/passwd). These files has SUID and SGID permission set on it.  

`chmod 4640 <file-name>` - It sets the suid permission. owner wouldn't have execute permission. it denoted as capital letter S.  
`chmod 4740 <file-name>` - It sets both the suid and execute permission. It denoted as small letters.  

#### SGID - Set Group Identification

`chmod 2640 <file-name>`  

`find . -perm /4000 `- To find SUID file  

`find . -perm /2000` - To find SGID file  

`find . -perm /6000 `- To find both  

#### Sticky Bit

users can create files, read and execute files owned by other users, but are not allowed to remove files owned by other users.  
`chmod +t <dir-name>` or `chmod 1640 <dir-name>`  
`find . -perm /1000`


## Search files

`find <path-to-dir> <search-param> <file-name>`  
`find /var/www -name "*.txt"` - Find all the text file under /var/www directory  
`find -mtime 3` - To get files modified in the last 3 days  
`find -name "*.txt" -mtime +10 -daystart` - To get all text files modified 10 or more days ago 
`find . -name "*.txt" -delete` - To delete all the matching files  

#### Search param list

- mmin - modified minute
- perm - permission
- name - name of the file
- mtime - modified in days
- cmin - change minute
- size
- type - file or directory
- user

#### Multiple search params can be added using AND, OR & NOT operators.
`find -size +512k -o -name "f*"` - OR   
`find -size +512k -name "f*"` - AND  
`find -not -size +1M` - NOT  
`find \! -size +1M` - NOT  

> Modified time is related to data of the file  
> Change time is related to metadata of the file  


## Compare & manipulate file content

`tail -n 20 /var/log/syslog` - To print last 20 lines of the file provided  
`head -n 20 /var/log/syslog` - To print first 20 lines of the file provided  

sed - stream editor  

`sed -i 's/<search-name>/<replace-with>/g' error.log`   
i = in-place - To preview the change, just remove the "i" from the above command  

`cut -d ' ' -f 1 error.log`  
d = delimiter, f = field  

`uniq error.log > filterederror.log`  
Only removes the duplicate which is adjacent and you can store the output to different file.  
options:  
-c = count the duplicated lines  
-d = only print the repeated line  
-u = only print the unique line 

`sort error.log | uniq`  
To remove all duplicate even if it's adjacent  

`diff -c <file-1> <file-2>`   
c = context

`diff -y <file-1> <file-2>` or `sdiff <file-1> <file-2>`  
To compare the files side by side  