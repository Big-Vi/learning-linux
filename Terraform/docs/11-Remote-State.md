## Remote state

State Locking - To prevent multiple operation to update the state file at the same time.  

Terraform keeps the remote file in sync.

To create secure state file, use combination of S3 & DynamoDB(state locking).

```
terraform {
	backend "s3" {
		bucket = "<bucket-name>"
		key = "project/terraform.tfstate"
		region = "<region>"
		dynamo_table = "state-locking"
	}
}
```  
