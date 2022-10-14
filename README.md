# calc-sectors-expire

1. export sectors info

```shell
# all sectors expiration info
lotus state  sectors f01245  >f01245_sectors_64G.txt 
```
```text
➜  cal_expire head -n 10 f01245_sectors_32G.txt
1:2563403
2:2563403
3:2563403
4:2563403
5:2563403
6:2563403
7:2563403
8:2563403
9:2563403
10:2563403
``` 




2. run
```shell
./target/debug/calc-sectors-expire run 
--begin_epoch 2198732 
--expect_exp_power=12P  
--files ~/fsdownload/cal_expire/f01245_sectors_32G.txt  
--files ~/fsdownload/cal_expire/f012456_sectors_64G.txt 
```