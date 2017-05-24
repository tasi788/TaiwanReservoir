# 台灣水情資訊

[經濟部水利署](http://fhy.wra.gov.tw/fhy/)
[資料來源](http://fhy.wra.gov.tw/ReservoirPage_2011/StorageCapacity.aspx)

## Requirement

- Python3.x
- PYQuery
- pprint
- requests

## Info

name = 水庫名稱
capavailable = 有效容量(萬立方公尺)
caplevel = 水位(公尺)
currcap = 有效蓄水量(萬立方公尺)
currcapper = 蓄水量百分比(%)

## Result Preview

```
{'capavailable': '20,134.00',
  'caplevel': '237.68',
  'currcap': '14,420.05',
  'currcapper': '71.62',
  'name': '石門水庫'},
 {'capavailable': '43.69',
  'caplevel': '71.99',
  'currcap': '42.70',
  'currcapper': '97.73',
  'name': '西勢水庫'},
```
