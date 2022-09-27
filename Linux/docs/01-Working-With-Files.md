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
     