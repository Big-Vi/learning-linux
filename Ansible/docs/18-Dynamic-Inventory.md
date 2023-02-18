## Dynamic Inventory

With large number of servers, it's impossible to maintain inventory in static file. so we might be able to retrieve server details programmatically if we're using cloud provider like AWS.  

`ansible-playbook <playbook>.yml -i <inventory-file>.py`

Write Python script to accept these params.  

`./<inventory-file>.py --list`
`./<inventory-file>.py --host <hostname>`

Or  

use inventory plugin
