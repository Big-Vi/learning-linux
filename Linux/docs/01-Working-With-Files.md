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