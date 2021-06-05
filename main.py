import logging
from dataclasses import dataclass, field
from datetime import datetime, timezone, timedelta
from typing import Optional

import coloredlogs
import requests
from dataclasses_json import dataclass_json
from dateutil.parser import parse, parserinfo
from pyquery import PyQuery as pq

logger = logging.getLogger(__name__)
coloredlogs.install(level='DEBUG', logger=logger)
tz = timezone(timedelta(hours=+8))


class CustomParserInfo(parserinfo):
    HMS = [('時')]
    JUMP = ['迄', '起', ':', '(', ')', '-']


@dataclass_json
@dataclass
class ReservoirData:
    name: str = field(hash=False, repr=True, compare=False, default=None)
    capavailable: float = field(hash=False, repr=True, compare=False, default=None)
    statisticTimeS: datetime = field(hash=False, repr=True, compare=False, default=None)
    statisticTimeE: datetime = field(hash=False, repr=True, compare=False, default=None)
    rainFall: Optional[float] = field(hash=False, repr=True, compare=False, default=None)
    inFlow: Optional[float] = field(hash=False, repr=True, compare=False, default=None)
    outFlow: float = field(hash=False, repr=True, compare=False, default=None)
    waterlevediff: float = field(hash=False, repr=True, compare=False, default=None)
    recordTime: datetime = field(hash=False, repr=True, compare=False, default=None)
    caplevel: float = field(hash=False, repr=True, compare=False, default=None)
    currcap: float = field(hash=False, repr=True, compare=False, default=None)
    currcapper: float = field(hash=False, repr=True, compare=False, default=None)


class ReservoirCrawler:
    def __init__(self):
        self.url = 'http://fhy.wra.gov.tw/ReservoirPage_2011/StorageCapacity.aspx'
        self.search = {0: '防汛重點水庫', 1: '所有水庫', 2: '水庫及攔河堰'}

        self.rowlist = ['name', 'capavailable', 'statisticTimeS', 'statisticTimeE', 'rainFall', 'inFlow', 'outFlow',
                        'waterlevediff', 'recordTime', 'caplevel', 'currcap', 'currcapper']
        # self.query = pq(self.req)('table#ctl00_cphMain_gvList.list.nowrap')('tr')

    def fetch_jsp(self):
        r = requests.get(self.url)
        self.viewstate = pq(r.text)('input#__VIEWSTATE').attr('value')
        self.viewstategenerator = pq(r.text)('input#__VIEWSTATEGENERATOR').attr('value')
        self.hiddenfield = pq(r.text)('input#ctl00_ctl02_HiddenField').attr('value')

    def fetch_data(self, date: datetime, search: int = 0):
        # search
        # 0 -> 防汛重點水庫
        # 1 -> 所有水庫
        # 2 -> 水庫及攔河堰
        payload = {'ctl00$ctl02': 'ctl00$cphMain$ctl00|ctl00$cphMain$cboSearch',
                   'ctl00_ctl02_HiddenField': self.hiddenfield,
                   '__EVENTTARGET': 'ctl00$cphMain$cboSearch',
                   '__EVENTARGUMENT': '',
                   '__LASTFOCUS': '',
                   '__VIEWSTATE': self.viewstate,
                   '__VIEWSTATEGENERATOR': self.viewstategenerator,
                   'ctl00$cphMain$ucDate$cboYear': date.year,
                   'ctl00$cphMain$ucDate$cboMonth': date.month,
                   'ctl00$cphMain$ucDate$cboDay': date.day,
                   '__ASYNCPOST': True}
        payload.update({'ctl00$cphMain$cboSearch': self.search[search]})
        fetch_data = requests.get(self.url, data=payload)
        if fetch_data.status_code != 200:
            logger.critical(f'HTTP {fetch_data.status_code}')
        return fetch_data

    def clean_horizon(self, horizon_: list):
        for x in range(len(horizon_)):
            if '--' in horizon_[x]:
                horizon_[x] = None
            elif '起' in horizon_[x] or '迄' in horizon_[x] or '時' in horizon_[x]:
                horizon_[x] = parse(horizon_[x], parserinfo=CustomParserInfo())
        return horizon_

    def parse_data(self, data):
        query = pq(data.text)('table#ctl00_cphMain_gvList.list.nowrap')('tr')
        reservoir_list = list()
        for reservoir_row in list(query.items())[2:-1]:
            horizon = reservoir_row.text().split('\n')
            horizon = self.clean_horizon(horizon)
            reservoir_list.append(
                ReservoirData(
                    name=horizon[0], capavailable=horizon[1],
                    statisticTimeS=horizon[2], statisticTimeE=horizon[3],
                    rainFall=horizon[4], inFlow=horizon[5], outFlow=horizon[6],
                    waterlevediff=horizon[7], recordTime=horizon[8],
                    caplevel=horizon[9], currcap=horizon[10], currcapper=horizon[11]
                )
            )
        return reservoir_list

    def fetch(self, date: datetime = datetime.now(tz)):
        self.fetch_jsp()
        data = self.fetch_data(date)
        return self.parse_data(data)