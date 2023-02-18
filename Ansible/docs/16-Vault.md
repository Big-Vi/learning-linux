## Vault

A Vault helps to encrypt the credentials.  
`ansible-vault encrypt <inventory-file>`
`ansible-vault create <file>`

To run ansible playbook with encrypted credentials.  
`ansible-playbook <playbook>.yml -i <inventory-file> --ask-vault-pass`
Or  
`ansible-playbook <playbook>.yml -i <inventory-file> --vault-password-file <password-file>`
Or  
`ansible-playbook <playbook>.yml -i <inventory-file> --vault-password-file <password>.py`

We can use python script to fetch the password from remote secret storage provider like AWS Secret Manager.  
