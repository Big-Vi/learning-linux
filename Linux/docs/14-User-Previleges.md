## User previleges

visudo - Fine permission control. 

user/group     | host=(run as user) | commandlist 
---            | ---                | ---         |
jhon           | ALL=(ALL)          | NOPASSWD: ALL
%developer     | ALL=(ALL)          | ALL

NOPASSWD = User don't need to enter password when running the listed command.  
Add % before the group name.  