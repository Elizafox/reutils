Utilities implemented
---------------------
See [https://pubs.opengroup.org/onlinepubs/9699919799/idx/utilities.html](the POSIX list of utilities) for a guideline of what we intend to implement. The [https://boxmatrix.info/wiki/BusyBox-Commands](BusyBox list of commands) is also a general guideline, although many utilities will likely not be implemented due to dubious usefulness.

Note that SCCS, compilers, `make`, most dev tools, etc. are out of the project's scope. Obsolete utilities like `uux` are also out of scope. Basically, if you can't find it on a modern Unix system, it's out of scope.

If you have anything you wish to add to the list, please open an issue!

Utilities
=========
[ ] alias
[ ] ar
[ ] at
[ ] atd
[ ] awk
[x] basename
[ ] batch
[ ] bc
[ ] bg
[x] cal
[x] cat
[ ] cd
[ ] chgrp
[ ] chmod
[ ] chown
[ ] cksum
[ ] cmp
[ ] comm
[ ] command
[ ] compress
[ ] cp
[ ] crond
[ ] crontab
[ ] csplit
[ ] cut
[ ] date
[ ] dd
[ ] df
[ ] diff
[x] dirname
[ ] du
[x] echo
[ ] ed
[ ] env
[ ] ex
[ ] expand
[ ] expr
[x] false
[ ] fc
[ ] fg
[ ] file
[ ] find
[ ] fold
[ ] fuser
[ ] gencat
[ ] get
[ ] getconf
[ ] getopts
[ ] grep
[ ] hash
[x] head
[ ] iconv
[ ] id
[ ] ipcrm
[ ] ipcs
[ ] jobs
[ ] join
[ ] kill
[ ] link
[x] ln
[ ] locale
[ ] localedef
[ ] logger
[ ] logname
[ ] lp
[ ] ls
[ ] m4
[ ] mailx
[ ] man
[ ] mesg
[ ] mkdir
[ ] mkfifo
[ ] more
[ ] mv
[ ] newgrp
[x] nice
[ ] nl
[ ] nm
[ ] nohup
[ ] od
[ ] paste
[ ] patch
[ ] pathchk
[ ] pax
[ ] pr
[ ] printf
[ ] ps
[ ] pwd
[ ] read
[ ] renice
[ ] rm
[ ] rmdir
[ ] sed
[ ] sh
[x] sleep
[ ] sort
[ ] split
[ ] strings
[ ] stty
[x] tail
[ ] talk
[x] tee
[ ] test
[ ] time
[ ] touch
[ ] tput
[ ] tr
[x] true
[ ] tsort
[x] tty
[ ] type
[ ] ulimit
[ ] umask
[ ] unalias
[ ] uname
[ ] uncompress
[ ] unexpand
[ ] uniq
[ ] unlink
[ ] uudecode
[ ] uuencode
[ ] val
[ ] vi
[ ] wait
[ ] wc
[ ] who
[ ] write
[ ] xargs
[ ] zcat

Linux-specific
==============
[ ] insmod
[ ] modprobe
[ ] rmmod
