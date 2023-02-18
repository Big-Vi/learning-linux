## SSH keys to manage nodes

Create ssh keys in the local system and copy it to the remote server.  
`ssh-copy-id -i id_rsa user@server`

`etc/ansible/hosts`

```
web ansible_host=10.20.12.0 ansible_user=user ansible_ssh_private_key_file=<file-path>
```
