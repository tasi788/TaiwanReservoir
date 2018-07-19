# 台灣水情資訊

[經濟部水利署](http://fhy.wra.gov.tw/fhy/)  
[資料來源](http://fhy.wra.gov.tw/ReservoirPage_2011/StorageCapacity.aspx)

## Requirement
- Python3.x
- PyQuery==1.4.0
- requests==2.19.1

## Info
name = 水庫名稱   
capavailable = 有效容量(萬立方公尺)  
caplevel = 水位(公尺)   
currcap = 有效蓄水量(萬立方公尺)  
currcapper = 蓄水量百分比(%)  
statisticTimeS = 統計時間(起)  
statisticTimeE = 統計時間(迄)  
rainFall = 降雨量(毫米)  
inFlow = 進水量  
outFlow = 出水量  
waterlevediff = 與昨日水位差(公尺)  
recordTime = 水情時間  

## Result Preview
```
res = reservoir()
resData = res.getData('dict')
print(data)
{
	'鳳山水庫': {
		'area': '臺灣南區',
		'capavailable': '751.17',
		'caplevel': '44.10',
		'currcap': '354.20',
		'currcapper': '47.15 %',
		'inFlow': '32.00',
		'location': '高雄市林園區',
		'name': '鳳山水庫',
		'outFlow': '26.34',
		'rainFall': '0.00',
		'recordTime': '2018-07-18(8時)',
		'statisticTimeE': '迄:2018-07-19(0時)',
		'statisticTimeS': '起:2018-07-18(0時)',
		'waterlevediff': None
	.........}

resData = res.getData()
[{
	'area': '澎湖地區',
	'capavailable': '28.36',
	'caplevel': '18.00',
	'currcap': '0.05',
	'currcapper': '0.16 %',
	'inFlow': '0.00',
	'location': '澎湖縣七美鄉',
	'name': '七美水庫',
	'outFlow': '0.00',
	'rainFall': '0.00',
	'recordTime': '2018-07-18(7時)',
	'statisticTimeE': '迄:2018-07-19(0時)',
	'statisticTimeS': '起:2018-07-18(0時)',
	'waterlevediff': '0.00'
		},
		...........]
```
