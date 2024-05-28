to build: cargo build --release

expected file structure:
	2006 -> part0
	2007 -> part0
	2008 -> part1, part2, etc
	etc...
	bti precompute test.exe

to change what year to search through, change line 131's &folders[0] to &folders[index], the index starts at 0, for example:
	2006 is 0 -> &folders[0]
	2007 is 1 -> &folders[1]
	2008 is 2 -> &folders[2]
etc.