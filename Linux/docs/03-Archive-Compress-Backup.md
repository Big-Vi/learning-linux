Archive -> Compress -> Backup

## Archive

tar - tape archiver - an archiving utility

`tar --list --file <filename>.tar` or
`tar -tf <filename>.tar` or
`tar tf <filename>.tar`

`tar --create --file <filename>.tar <file>` or 
`tar cf <filename>.tar <file>`

`tar --append --file <filename>.tar <file>` or
`tar rf <filename>.tar <file>`

`tar --extract --file <filename>.tar --directory /tmp/` or 
`tar xcof <filename>.tar -C /tmp/`


## Compress

`gzip <filename>`
`bzip2 <filename>`
`xz <filename>`
`zip -r <filename>.zip <file>/`

`gunzip <filename>.gz` or 
`gzip --decompress <filename>.gz`

`bunzip <filename>.gz` or 
`bunzip --decompress <filename>.gz`

`unxz <filename>.gz` or 
`unxz --decompress <filename>.gz`
`unzip <filename>.zip`

Options:

--keep - Original files are untouched


### To archive and compress

`tar --create --gzip --file <filename>.tar.gz <file>` or 
`tar czf <filename>.tar.gz <file>`

`tar --create --bzip2 --file <filename>.tar.bz2 <file>` or
`tar cjf <filename>.tar.bz2 <file>`

`tar --create --xz --file <filename>.tar.xz <file>` or
`tar cjf <filename>.tar.xz <file>`

tar autocompress

`tar --create --auto-compress --file <filename>.tar.gz <file>` or
`tar caf <filename>.xz <file>`


## Backup

rsync - remote synchronization
`rsync -a <local-dir>/ <remote-user-name>@<remote-ip>:/<remote-dir-path>/`
a = archive

Disk imaging - To backup the entire disk
`sudo dd if=/dev/vda of=diskimage.raw bs=1M status=progress`
if - input file, of - output file, bs - block size