all:
	rustc -O heap.rs -o bin/heap


dev:
	nodemon --exec \
		'rustc heap.rs -o bin/heap && cat priv/text1_000.txt | bin/heap' -w heap.rs

bench:
	cat priv/text100.txt| time bin/heap
	@echo
	@echo
	cat priv/text1_000.txt| time bin/heap
	@echo
	@echo
	cat priv/text10_000.txt| time bin/heap
	@echo
	@echo
	cat priv/text100_000.txt| time bin/heap
	@echo
	@echo
	cat priv/text1_000_000.txt| time bin/heap
