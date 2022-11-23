## Manage & configure virtual machines

> qemu - Quick Emulator  
    kvm - Kernel based Virtual Machine  

Use `virsh` to manage virtual machine on command line.  

`dnf install libvirt qemu-kvm`  

`vi testmachine.xml`  
```
	<domain type="qemu">  

		<name>TestMachine</name>  

		<memory unit="GiB>1</memory>  

		<vcpu>1</vcpu>  

		<os>  

			<type arch="x86_64>hvm</type> #Hardware virtual machine  
		</os>  

	</domain>  
```

`virsh help <command>`  

`virsh define testmachine.xml`  
`virsh list --all` - To list active and not active virtual machine.  
`virsh start/reboot/reset/shutdown/destroy/undefine <VM-Domain-name>`  

`virsh autostart <VM-Domain-name>`- To start when server boots  
`virsh autostart --disable <VM-Domain-name>` - Not to autostart  
`virsh dominfo <VM-Domain-name>` - To explore the assigned resources of vm  

`virsh setvcpus <VM-Domain-name> 2 --config --maximum` - To change cpu to max 2.   