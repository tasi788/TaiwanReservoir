import re
import sys
import json
import codecs
import pprint
import requests
from itertools import zip_longest as zip
from time import strftime
from pyquery import PyQuery as pq

'''
水庫資料
https://data.gov.tw/dataset/32726
'''

ros = reservoir()
b = ros.getData('???')
pp(b)
class reservoir:
	def __init__(self):
		self.url = 'http://fhy.wra.gov.tw/ReservoirPage_2011/StorageCapacity.aspx'
		try:
			viewstate = pq('http://fhy.wra.gov.tw/ReservoirPage_2011/StorageCapacity.aspx')('input#__VIEWSTATE').attr('value')
		except ConnectionError:
			sys.exit(1)
		self.payload = {'ctl00$ctl02': 'ctl00$cphMain$ctl00|ctl00$cphMain$cboSearch',
			'ctl00_ctl02_HiddenField': ';;AjaxControlToolkit, Version=3.0.20820.16598, Culture=neutral, PublicKeyToken=28f01b0e84b6d53e:zh-TW:707835dd-fa4b-41d1-89e7-6df5d518ffb5:411fea1c:865923e8:77c58d20:91bd373d:14b56adc:596d588c:8e72a662:acd642d2:269a19ae',
			'__EVENTTARGET': 'ctl00$cphMain$cboSearch',
			'__EVENTARGUMENT':'',
			'__LASTFOCUS':'',
			'__VIEWSTATE': viewstate,
			'__VIEWSTATEGENERATOR':' 5967A80E',
			'ctl00$cphMain$cboSearch': '所有水庫',
			'ctl00$cphMain$ucDate$cboYear': strftime('%Y'),
			'ctl00$cphMain$ucDate$cboMonth': int(strftime('%m')),
			'ctl00$cphMain$ucDate$cboDay': int(strftime('%d')) - 1,
			'__ASYNCPOST': True}
		self.req = requests.post(self.url,data=self.payload).text
		self.rowlist = ['name', 'capavailable', 'statisticTimeS', 'statisticTimeE', 'rainFall', 'inFlow', 'outFlow', 'waterlevediff', 'recordTime', 'caplevel', 'currcap', 'currcapper']
		self.query = pq(self.req)('table#ctl00_cphMain_gvList.list.nowrap')('tr')
	def loadParse(self):
		with codecs.open('reservoirInfo.json', 'r', encoding='utf-8-sig') as file:
			readJson = json.loads(file.read())
			reservoirArea = {}
			return readJson['TaiwanWaterExchangingData']['HydrologyReservoirClass']['ReservoirsInformation']
	def getData(self, type='list'):
		parseList, dataList, dataDict, parseDict = [], [], {}, {}
		loadParse = self.loadParse()
		for x in loadParse:
			b = {}
			a = dict(area = x['Area'], location = x['Location'])
			b[x['ReservoirName']] = a
			parseDict.update(b)
		for x in self.query.items():
			tmpDict, tmpDictN = {}, {}
			ReservoirList = x.text().split('\n')
			if ReservoirList[0] in parseDict.keys():
				for y, z in zip(self.rowlist, ReservoirList):
					if z == '--':
						tmpDict[y] = None
					else:
						tmpDict[y] = z
				tmpDict['area'] = parseDict[ReservoirList[0]]['area']
				tmpDict['location'] = parseDict[ReservoirList[0]]['location']
				tmpDictN[ReservoirList[0]] = tmpDict
				dataDict.update(tmpDictN), dataList.append(tmpDict)
		if type == 'list':
			return dataList
		elif type == 'dict':
			return dataDict
	def getAreaCate(self, area=None):
		tmplist, tmpdict = [], {}
		area = self.loadParse()
		getData = self.getData()
		#areaEn = ['north', 'middle', 'south', 'ffshoreIsland']
		#areaZh = ['台灣北區', '台灣中區', '台灣南區', '澎湖地區']
		areaZhtZh = {
			'台灣北區': 'north',
			'台灣中區': 'middle',
			'台灣南區': 'south',
			'澎湖地區': 'offshoreIsland'
		}
		for x in getData:
			tmplist[areaZhtZh[x['name']]].append(x)
		print(tmplist)


'''
[區域{
 每日蓄水統計{
name = 水庫名稱
capavailable = 有效容量(萬立方公尺)
statisticTime = 統計時間
rainFall = 降雨量(毫米)
inFlow = 進水量
outFlow = 出水量
waterlevediff = 與昨日水位差(公尺)
 }
即時水情資料{
recordTime = 水情時間
caplevel = 水位(公尺)
currcap = 有效蓄水量(萬立方公尺)
currcapper = 蓄水量百分比(%)
}
}]
'''
