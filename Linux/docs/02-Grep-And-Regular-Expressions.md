## Grep

`grep <options> <search-pattern> <file>/<dir>`  

options:  
- i - case-insensitive  
- r - recursively look under specified directory and it's sub directories  
- c - Total number of times that string appeared in the file  
- v - invert-match  
- w - only exactly matched words  
- o - only-matching  

## Regular expression

^ - The line begins with - ^RAM  

$ - The line ends with - CPU$  

. - Match any one character - c.u  

\ - Escaping the special characters  

\* -  Match the previous element 0 or more times - Rum* = Ru, Rum, Rumm  

\+ - Match the previous element 1 or more times - Rum\\+ = Rum, Rumm  

{} - Previous element can exist number of times  
 - Rum{2,} = Rumm, Rummmm - Atleast 2 previous element  
 - Rum{,2} = Ru, Rum, Rumm - Most 2 prvious element  
 - Rum{2} = Rumm - Exactly previous element  

? - Make the previos element optional  

| - Match one thing or other  

[] - Range or sets - R[au]m  

() - Subexpressions - R([a-z][0-9])*m  

[^] - Negated ranges or sets - http[^s] = http, R[^a-d]m  

Grep has basic regular expression(?, {, |, (, ), +) built in. So in order to use + operator, escape it using backslash(\). Use extended regex to avoid converting regular grep operator to regular plus or vice versa.  


## Extended Regex

grep -E 'Ru+' /var/ or egrep 'Ru+' /var/  