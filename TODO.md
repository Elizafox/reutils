Documentation
-------------
Manpages should be written. Likely, we will want to generate manpages from something else, as `troff` is an extremely hideous and antiquated format to write anything in.

*(TODO: investigate if we can use docstrings for this purpose?)*

Utilities implemented
---------------------
See [the POSIX list of utilities](https://pubs.opengroup.org/onlinepubs/9699919799/idx/utilities.html) for a guideline of what we intend to implement. The [BusyBox list of commands](https://boxmatrix.info/wiki/BusyBox-Commands) is also a general guideline, although many utilities will likely not be implemented due to dubious usefulness.

Note that SCCS utilities, compilers, `make`, most dev tools, etc. are out of the project's scope. Obsolete utilities like `uux` are also out of scope. Basically, if you can't find it on a modern Unix system, it's out of scope. We are open to having our minds changed on certain utilities, but a good rationale (and preferably patches) will be needed.

Compatibility with POSIX is the main goal, except where POSIX is ambiguous and unclear (as happens often). Some minor extensions to POSIX are offered where applicable, but don't expect this to be as fully featured as coreutils. It's aimed for small environments, after all.

If you have anything you wish to add to the list, please open an issue! Note that many of the utilities listed are actually more appropriate as shell builtins, and not utilities per se.

Utilities
=========
- [ ] alias
- [ ] ar
- [ ] at
- [ ] atd
- [ ] awk
- [x] basename
- [ ] batch
- [ ] bc
- [ ] bg
- [x] cal
- [x] cat
- [ ] cd
- [ ] chgrp
- [ ] chmod
- [ ] chown
- [ ] cksum
- [ ] cmp
- [ ] comm
- [ ] command
- [ ] compress
- [ ] cp
- [ ] crond
- [ ] crontab
- [ ] csplit
- [ ] cut
- [ ] date
- [ ] dd
- [x] df *(Note: Linux, macOS, and FreeBSD only for now)*
- [ ] diff
- [x] dirname
- [ ] du
- [x] echo
- [ ] ed
- [ ] env
- [ ] ex
- [ ] expand
- [ ] expr
- [x] false
- [ ] fc
- [ ] fg
- [ ] file
- [ ] find
- [ ] fold
- [ ] fuser
- [ ] gencat
- [ ] get
- [ ] getconf
- [ ] getopts
- [ ] grep
- [ ] hash
- [x] head
- [ ] iconv
- [ ] id
- [ ] ipcrm
- [ ] ipcs
- [ ] jobs
- [ ] join
- [ ] kill
- [x] link
- [x] ln
- [ ] locale
- [ ] localedef
- [ ] logger
- [ ] logname
- [ ] lp
- [ ] ls
- [ ] m4
- [ ] mailx
- [ ] man
- [ ] mesg
- [ ] mkdir
- [ ] mkfifo
- [ ] more
- [ ] mv
- [ ] newgrp
- [x] nice
- [ ] nl
- [ ] nohup
- [ ] od
- [ ] paste
- [ ] patch
- [ ] pathchk
- [ ] pax
- [ ] ping
- [ ] ping6
- [ ] pr
- [ ] printf
- [ ] ps
- [x] pwd
- [ ] read
- [x] renice *(Note: no Windows support yet)*
- [ ] rm
- [ ] rmdir
- [ ] sed
- [ ] sh
- [x] sleep
- [ ] sort
- [ ] split
- [ ] strings
- [ ] stty
- [x] tail
- [ ] talk
- [x] tee
- [ ] test
- [ ] time
- [ ] touch
- [ ] tput
- [ ] tr
- [ ] traceroute
- [ ] traceroute6
- [x] true
- [ ] tsort
- [x] tty
- [ ] type
- [ ] ulimit
- [ ] umask
- [ ] unalias
- [x] uname
- [ ] uncompress
- [ ] unexpand
- [ ] uniq
- [ ] unlink
- [ ] uudecode
- [ ] uuencode
- [ ] val
- [ ] vi
- [ ] wait
- [ ] wc
- [ ] who
- [ ] write
- [ ] xargs
- [ ] zcat

Linux-specific
==============
- [ ] insmod
- [ ] modprobe
- [ ] rmmod
