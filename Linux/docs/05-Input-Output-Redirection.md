## Input & output redirection

stdin = <
stdout = 1>
stderr = 2>

Redirect input:
`sort < file.txt`
`sendemail someone@gmail.com < content.txt`

Redirect ouput:
`date > file.txt` or `date 1> file.txt`
`date 2> error.txt` - to send error ouput
`date >> file.txt` - Not to overwrite file. append it.
`2>/dev/null` - Discard error message

`grep -r '^CPU' /etc/ > all_output.txt 2>&1` - To redirect output & error to same file.

Herdoc
`sort <<EOF
2
3
6
7
EOF`

Here String
`bc <<<2+6+8`

Piping:
`grep -v '^#' /etc/apache2/apache2.conf | sort | column -t`