```
$ make bench

cat priv/text100.txt| time bin/heap
sweetness: 500000000
num_steps: 30
0.00user 0.00system 0:00.00elapsed 0%CPU (0avgtext+0avgdata 2460maxresident)k
0inputs+0outputs (0major+144minor)pagefaults 0swaps


cat priv/text1_000.txt| time bin/heap
sweetness: 500000000
num_steps: 286
0.00user 0.00system 0:00.00elapsed 0%CPU (0avgtext+0avgdata 2544maxresident)k
0inputs+0outputs (0major+150minor)pagefaults 0swaps


cat priv/text10_000.txt| time bin/heap
sweetness: 500000000
num_steps: 3000
0.00user 0.00system 0:00.01elapsed 80%CPU (0avgtext+0avgdata 2500maxresident)k
0inputs+0outputs (0major+190minor)pagefaults 0swaps


cat priv/text100_000.txt| time bin/heap
sweetness: 500000000
num_steps: 30182
0.11user 0.00system 0:00.11elapsed 100%CPU (0avgtext+0avgdata 3808maxresident)k
0inputs+0outputs (0major+584minor)pagefaults 0swaps


cat priv/text1_000_000.txt| time bin/heap
sweetness: 500000000
num_steps: 299936
1.22user 0.01system 0:01.24elapsed 99%CPU (0avgtext+0avgdata 18696maxresident)k
0inputs+0outputs (0major+6718minor)pagefaults 0swaps
```
