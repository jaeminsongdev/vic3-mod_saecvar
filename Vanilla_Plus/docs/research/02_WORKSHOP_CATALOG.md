# 02. Steam Workshop 모드 사례 카탈로그

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

**등록된 고유 Workshop ID: 111개.** 같은 후속판과 유사 기능을 독립 사례로 보존하되, 개발 기능은 18개 기능군으로 중복 통합했다. 목록의 `참고/규칙변경/구형참고/고위험/기본제외`는 설계 분류이지 개별 모드의 품질 평가가 아니다.

**검증 상태:** `2026-10-17 설명 열람`은 이 문서화 과정에서 실제 설명 페이지를 다시 확인한 대표 항목이다. 나머지는 앞선 조사 링크의 보존본이며, 지금 호환성·공개상태는 불명이다.

## UI (12개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3724865565 | [GORA UI+](https://steamcommunity.com/sharedfiles/filedetails/?id=3724865565) | 시장·건설·외교·의회·국유자산·연구 UI 통합, 1.14 지원 표기 | 참고 | F04,F06,F14 | 재열람 |
| 3713208305 | [Voice's Enhanced Topbar](https://steamcommunity.com/sharedfiles/filedetails/?id=3713208305) | 상단바에 GDP·생활수준·악명·문해율·군사력 등 상시 표시 | 참고 | F06 | 미재검증 |
| 3636736389 | [Advanced Construction Queue](https://steamcommunity.com/sharedfiles/filedetails/?id=3636736389) | 건설 대기열의 주별 조건·세부 정보·필터 | 참고 | F01,F06 | 미재검증 |
| 3676318398 | [Advanced Market Panel](https://steamcommunity.com/sharedfiles/filedetails/?id=3676318398) | 상품 수급·가격 변화·무역 기회 분석 | 참고 | F04,F10 | 미재검증 |
| 3527702083 | [Improved Company List](https://steamcommunity.com/sharedfiles/filedetails/?id=3527702083) | 기업 목록 정렬·검색·유형 필터 | 참고 | F04 | 미재검증 |
| 3733651973 | [Trade Partners](https://steamcommunity.com/sharedfiles/filedetails/?id=3733651973) | 국가별 주요 수출입·교역 상대 시각화 | 참고 | F04,F10 | 미재검증 |
| 2898482972 | [Visible Pop Needs](https://steamcommunity.com/sharedfiles/filedetails/?id=2898482972) | 계층별 주요 소비재 수요 시각화 | 참고 | F15 | 미재검증 |
| 3312478537 | [Global Stats (GDP and POP)](https://steamcommunity.com/sharedfiles/filedetails/?id=3312478537) | 세계 GDP·인구 대비 자국 비중 표시 | 참고 | F16 | 미재검증 |
| 3722701706 | [War Goal & Treaty UI](https://steamcommunity.com/sharedfiles/filedetails/?id=3722701706) | 전쟁 목표 국가 필터·강화조약 결과 구분 | 참고 | F13 | 미재검증 |
| 3753749567 | [No Commander Alert](https://steamcommunity.com/sharedfiles/filedetails/?id=3753749567) | 지휘관 미배치·지휘 한도 초과 알림 | 참고 | F05,F13 | 미재검증 |
| 3142463417 | [Headlines](https://steamcommunity.com/sharedfiles/filedetails/?id=3142463417) | 주요 국제사건을 뉴스 형식으로 표시 | 참고 | F16 | 미재검증 |
| 3625787964 | [More Icons for Formations](https://steamcommunity.com/sharedfiles/filedetails/?id=3625787964) | 군 편제 아이콘·색상 선택 확대 | 참고 | F13,F06 | 미재검증 |

## 자동화 (8개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3092955121 | [The OGAS – Your Build Advisor](https://steamcommunity.com/sharedfiles/filedetails/?id=3092955121) | 수익성 기반 생산방식·노동 균형·건설 자동화; 기본 레시피 한계 | 참고 | F01,F02,F03 | 재열람 |
| 3353797125 | [Auto-Apply PMs](https://steamcommunity.com/sharedfiles/filedetails/?id=3353797125) | 지역 수익성을 반영한 생산방식 자동 적용 | 참고 | F02 | 미재검증 |
| 3344726320 | [Auto-Apply Automation PMs](https://steamcommunity.com/sharedfiles/filedetails/?id=3344726320) | 노동력·비용 기준 노동절약형 생산방식 전환 | 참고 | F02 | 미재검증 |
| 3357811223 | [Supply Chain Semi-Auto Builder](https://steamcommunity.com/sharedfiles/filedetails/?id=3357811223) | 공급망별 건설 및 철도·발전소·행정시설 보충 | 참고 | F01,F03 | 미재검증 |
| 3278770055 | [Controllable Private Construction](https://steamcommunity.com/sharedfiles/filedetails/?id=3278770055) | 민간 건설의 지역·업종·수익성 기준 통제 | 참고 | F11 | 미재검증 |
| 3432933121 | [Auto-Diplomacy And Advanced Subject Management](https://steamcommunity.com/sharedfiles/filedetails/?id=3432933121) | 외교관계 개선·속국 법률·경제·외교 관리 | 참고 | F12 | 미재검증 |
| 3279201462 | [Auto-Recruit & One-Click Recruit Commanders](https://steamcommunity.com/sharedfiles/filedetails/?id=3279201462) | 지휘관 자동 모집·일괄 충원 | 참고 | F13 | 미재검증 |
| 3467896823 | [Choose Laws For Released Subjects](https://steamcommunity.com/sharedfiles/filedetails/?id=3467896823) | 해방 속국의 초기 법률 선택 | 참고 | F12,F14 | 미재검증 |

## 추가UI (12개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3803950977 | [Build All: Bulk Construction, One Click](https://steamcommunity.com/sharedfiles/filedetails/?id=3803950977) | 주별 건물을 1·5·10단계씩 일괄 건설 | 참고 | F01 | 미재검증 |
| 3760606574 | [Decree State Insights](https://steamcommunity.com/sharedfiles/filedetails/?id=3760606574) | 칙령 대상 주의 GDP·인구·생활수준·농민 수 비교 | 참고 | F09,F06 | 미재검증 |
| 3807209131 | [Visible Pop Qualifications](https://steamcommunity.com/sharedfiles/filedetails/?id=3807209131) | 인구의 직업 자격 정보 표시 | 참고 | F15 | 미재검증 |
| 3751688101 | [Visible Pop Needs 1.13.x](https://steamcommunity.com/sharedfiles/filedetails/?id=3751688101) | 소비재 수요 표시 모드의 1.13 대응판 | 후속판 | F15 | 미재검증 |
| 3588231593 | [Consumption Tax Efficiency](https://steamcommunity.com/sharedfiles/filedetails/?id=3588231593) | 권위 소비 대비 소비세 예상 수입 비교 | 참고 | F04,F09 | 미재검증 |
| 3633799211 | [Detailed Character Notifications](https://steamcommunity.com/sharedfiles/filedetails/?id=3633799211) | 인물 알림에 이해집단·이념·특성 정보 표시 | 참고 | F05,F14 | 미재검증 |
| 3687767007 | [Parliament 2.0](https://steamcommunity.com/sharedfiles/filedetails/?id=3687767007) | 의회·선거 정치세력 구성 시각화 | 참고 | F14 | 미재검증 |
| 3422044570 | [Intuitive Acceptance Icons](https://steamcommunity.com/sharedfiles/filedetails/?id=3422044570) | 문화 수용 단계 아이콘 순서 개선 | 참고 | F06,F15 | 미재검증 |
| 3752681561 | [Streamlined Navy](https://steamcommunity.com/sharedfiles/filedetails/?id=3752681561) | 해군 임무를 통합하고 공격·균형·방어 태세 추가 | 규칙변경 | F13 | 미재검증 |
| 3741619775 | [Colonial Administration Incorporation Fix](https://steamcommunity.com/sharedfiles/filedetails/?id=3741619775) | 식민 행정부의 인접 영토 편입 가능화 | 규칙변경 | F07 | 미재검증 |
| 3282791507 | [Cultural Communities Dropdown List](https://steamcommunity.com/sharedfiles/filedetails/?id=3282791507) | 문화 공동체 목록 사용성 개선 | 참고 | F06,F15 | 미재검증 |
| 3508296963 | [Expanded Topbar Framework](https://steamcommunity.com/sharedfiles/filedetails/?id=3508296963) | 상단 정보창을 확장하는 UI 기반 | 참고 | F06 | 미재검증 |

## 인구 (5개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3336914177 | [Grey's Soft Pop Adjustments](https://steamcommunity.com/sharedfiles/filedetails/?id=3336914177) | 동화·개종·이주·소규모 POP·교육의 복합 밸런스 | 밸런스 | F07,F15,F18 | 미재검증 |
| 3613304558 | [Merciful Incorporation](https://steamcommunity.com/sharedfiles/filedetails/?id=3613304558) | 문화 유사성에 따라 편입 기간 0.8~10년 | 밸런스 | F07 | 미재검증 |
| 2882947823 | [Double Incorporation Speed](https://steamcommunity.com/sharedfiles/filedetails/?id=2882947823) | 편입 속도 배가 | 구형참고 | F07 | 미재검증 |
| 3042917383 | [Faster Assimilation](https://steamcommunity.com/sharedfiles/filedetails/?id=3042917383) | 동화 및 편입 속도 강한 보정 | 고위험 | F07 | 미재검증 |
| 2897378189 | [Ultra Historical Research & Education](https://steamcommunity.com/sharedfiles/filedetails/?id=2897378189) | 교육·문해율·기술 전파·자격 밸런스 | 고위험 | F14,F15 | 미재검증 |

## 외교정치 (8개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3308595632 | [Scaling Infamy Decay](https://steamcommunity.com/sharedfiles/filedetails/?id=3308595632) | 현재 악명에 따라 감쇠 속도가 달라짐 | 밸런스 | F08 | 재열람 |
| 3121842534 | [MW Infamy Decay](https://steamcommunity.com/sharedfiles/filedetails/?id=3121842534) | 악명량에 따른 비선형 감쇠 | 밸런스 | F08 | 미재검증 |
| 3736349401 | [Configurable Infamy 1.13](https://steamcommunity.com/sharedfiles/filedetails/?id=3736349401) | 악명 발생 비용을 게임 중 옵션으로 조절 | 밸런스 | F08 | 미재검증 |
| 2895956555 | [Better Decrees 1.13](https://steamcommunity.com/sharedfiles/filedetails/?id=2895956555) | 칙령 효과·비용·종류 조정 | 밸런스 | F09 | 재열람 |
| 3642608053 | [Scaling Decree Cost (Revived)](https://steamcommunity.com/sharedfiles/filedetails/?id=3642608053) | 보유 주 규모에 따른 칙령 비용 감소 | 밸런스 | F09 | 미재검증 |
| 3351014672 | [Grey's Populist Legal Power](https://steamcommunity.com/sharedfiles/filedetails/?id=3351014672) | 법률·제도·권위·정치운동 등 조정 | 고위험 | F14 | 미재검증 |
| 3609384092 | [Grey's Populist Legal Innovation](https://steamcommunity.com/sharedfiles/filedetails/?id=3609384092) | 법률에 따른 혁신·투자·기술 확산 변화 | 고위험 | F14 | 미재검증 |
| 3504963278 | [Recognition Expanded](https://steamcommunity.com/sharedfiles/filedetails/?id=3504963278) | 미승인국 승인 경로의 대체 조건 | 규칙변경 | F08,F14 | 미재검증 |

## 경제전쟁 (8개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3345217364 | [Grey's Soft Econ Adjustments](https://steamcommunity.com/sharedfiles/filedetails/?id=3345217364) | 가격 범위·고용·축소·관세·배당 등 경제 밸런스 | 고위험 | F11,F17 | 미재검증 |
| 3115017647 | [Grey's Smarter Private Construction](https://steamcommunity.com/sharedfiles/filedetails/?id=3115017647) | 민간 투자 판단에 원자재·수익성·인프라 반영 | 참고 | F11 | 미재검증 |
| 3227982912 | [Kuromi's AI](https://steamcommunity.com/sharedfiles/filedetails/?id=3227982912) | AI 경제·법률·연구·예산 판단 개선 | 참고 | F11,F14 | 미재검증 |
| 3532567862 | [Expertise for Production Methods](https://steamcommunity.com/sharedfiles/filedetails/?id=3532567862) | 생산방식 전환에 숙련도·손실 추가 | 대형개편 | F02,F17 | 미재검증 |
| 3638997092 | [JD Fervor War Fix](https://steamcommunity.com/sharedfiles/filedetails/?id=3638997092) | 열성의 전쟁 지지도 영향 상한 보정 | 밸런스 | F13 | 미재검증 |
| 3718146118 | [Carry Capacity Increased](https://steamcommunity.com/sharedfiles/filedetails/?id=3718146118) | 해군의 육군 수송 수용량 증가 | 밸런스 | F13 | 미재검증 |
| 3760550713 | [Performance Optimizer](https://steamcommunity.com/sharedfiles/filedetails/?id=3760550713) | 이벤트·AI·POP 처리 주기 조절; 규칙 변경 가능 | 위험검증 | F18 | 미재검증 |
| 3459881016 | [Victoria 3 Performance](https://steamcommunity.com/sharedfiles/filedetails/?id=3459881016) | 연산 주기 및 일부 연출·AI 처리 조정 | 위험검증 | F18 | 미재검증 |

## 소형유틸 (18개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3776818197 | [Enter Key](https://steamcommunity.com/sharedfiles/filedetails/?id=3776818197) | 다수 확인·수락 버튼에 Enter 키 사용 | 참고 | F06,F18 | 재열람 |
| 3772525185 | [National History](https://steamcommunity.com/sharedfiles/filedetails/?id=3772525185) | 국가의 게임 중 사건 연혁 기록 | 참고 | F16 | 미재검증 |
| 3814823352 | [Smart Treaties](https://steamcommunity.com/sharedfiles/filedetails/?id=3814823352) | 상품 이전 조약의 예상 순이익·운송비·최적 수량 | 참고 | F10 | 미재검증 |
| 3773140049 | [Choose Company Headquarters](https://steamcommunity.com/sharedfiles/filedetails/?id=3773140049) | 회사 본사 위치 선택 | 규칙변경 | F04,F11 | 미재검증 |
| 3797038621 | [Customizable Topbar Information](https://steamcommunity.com/sharedfiles/filedetails/?id=3797038621) | 상단 표시 통계 약 30종 사용자 정의 | 참고 | F06 | 미재검증 |
| 3792501785 | [State Fiscal Charts](https://steamcommunity.com/sharedfiles/filedetails/?id=3792501785) | 주별 수입·지출 차트 정렬 | 참고 | F04 | 미재검증 |
| 3783347952 | [View more information](https://steamcommunity.com/sharedfiles/filedetails/?id=3783347952) | 타국의 행정·권위·재정 등 정보 확장 | 참고 | F04,F06 | 미재검증 |
| 3815841501 | [Achievement Guide](https://steamcommunity.com/sharedfiles/filedetails/?id=3815841501) | 도전과제 조건·진행상황 표시 | 참고 | F16 | 미재검증 |
| 3774563295 | [Adjustable UI Opacity](https://steamcommunity.com/sharedfiles/filedetails/?id=3774563295) | UI 투명도 조절 | 참고 | F18 | 미재검증 |
| 3774236562 | [Adjust Middle Mouse Wheel Scrolling Speed](https://steamcommunity.com/sharedfiles/filedetails/?id=3774236562) | UI 스크롤 속도 조절 | 참고 | F18 | 미재검증 |
| 3783675625 | [Auto Adjustment Tariff](https://steamcommunity.com/sharedfiles/filedetails/?id=3783675625) | 가격·재정 조건을 고려한 관세 자동 조정 | 참고 | F10 | 미재검증 |
| 3775129439 | [Auto Expand for Ports, Trade Centers and Power Plants](https://steamcommunity.com/sharedfiles/filedetails/?id=3775129439) | 항구·무역센터·발전소 자동확장 | 참고 | F03 | 미재검증 |
| 3793631825 | [Automatic Downsizing for State-Owned Buildings](https://steamcommunity.com/sharedfiles/filedetails/?id=3793631825) | 국영 건물의 공실·수익성을 고려한 자동 축소 | 참고 | F01,F11 | 미재검증 |
| 3791071271 | [Building relocation](https://steamcommunity.com/sharedfiles/filedetails/?id=3791071271) | 특정 건물을 주간 이전 | 규칙변경 | F01,F17 | 미재검증 |
| 3772712271 | [Subject Production Method Control](https://steamcommunity.com/sharedfiles/filedetails/?id=3772712271) | 종주국의 속국 생산방식 관리 | 규칙변경 | F12,F02 | 미재검증 |
| 3777893599 | [Transfer All Ships](https://steamcommunity.com/sharedfiles/filedetails/?id=3777893599) | 조약 중 선박 일괄 이전 | 참고 | F13 | 미재검증 |
| 3753665894 | [Idle Law Alert](https://steamcommunity.com/sharedfiles/filedetails/?id=3753665894) | 제정 중인 법이 없을 때 알림 | 참고 | F05,F14 | 미재검증 |
| 3779275113 | [Power Bloc Leader Action Alerts](https://steamcommunity.com/sharedfiles/filedetails/?id=3779275113) | 세력권 지도자 행동 가능 알림 | 참고 | F05,F12 | 미재검증 |

## 소폭규칙 (7개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3790184703 | [More Reasonable Infamy](https://steamcommunity.com/sharedfiles/filedetails/?id=3790184703) | 영토의 자국 주요문화 인구비중에 따라 악명 비용 수정 | 밸런스 | F08 | 미재검증 |
| 3771426869 | [Tiered State Incorporation](https://steamcommunity.com/sharedfiles/filedetails/?id=3771426869) | 주 편입 상태를 단계별로 세분화 | 대형개편 | F07 | 미재검증 |
| 3749655399 | [Population Scaled Decree Cost](https://steamcommunity.com/sharedfiles/filedetails/?id=3749655399) | 주 인구에 따라 칙령 권위비용 차등 | 밸런스 | F09 | 미재검증 |
| 3773963536 | [State Taxation Adjustment](https://steamcommunity.com/sharedfiles/filedetails/?id=3773963536) | 주별 세금 강도를 개별 조절 | 규칙변경 | F17 | 미재검증 |
| 3774247196 | [Subject Tribute Adjuster](https://steamcommunity.com/sharedfiles/filedetails/?id=3774247196) | 속국 조공 비율 선택 | 규칙변경 | F12 | 미재검증 |
| 3790394405 | [Enact Multiple Laws Simultaneously](https://steamcommunity.com/sharedfiles/filedetails/?id=3790394405) | 상이한 법률군 법안 동시 제정 | 고위험 | F14 | 미재검증 |
| 3800016745 | [Improve and Damage Relations Tiers](https://steamcommunity.com/sharedfiles/filedetails/?id=3800016745) | 관계 개선·악화 강도에 따른 차등 영향력 비용 | 규칙변경 | F12 | 미재검증 |

## 조작 (6개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3593113294 | [UI – Easy withdraw treaty](https://steamcommunity.com/sharedfiles/filedetails/?id=3593113294) | 만료 조약의 취소·재협상 버튼 접근 개선 | 참고 | F10,F06 | 미재검증 |
| 3720145399 | [Set All Tariffs](https://steamcommunity.com/sharedfiles/filedetails/?id=3720145399) | 여러 상품의 관세 설정 일괄 적용 | 위험검증 | F10 | 미재검증 |
| 3592539197 | [QOL – Baseline construction](https://steamcommunity.com/sharedfiles/filedetails/?id=3592539197) | 여러 주에 기반시설 최소 단계 일괄 건설 | 참고 | F01,F03 | 미재검증 |
| 3255320685 | [GDP Growth Rate Improved](https://steamcommunity.com/sharedfiles/filedetails/?id=3255320685) | GDP 성장률 시계열·이동평균 | 참고 | F04 | 미재검증 |
| 3512900057 | [UI Shows Pop Ownership Shares](https://steamcommunity.com/sharedfiles/filedetails/?id=3512900057) | 직업별 실제 소유지분·배당 비중 확인 | 참고 | F04,F15 | 미재검증 |
| 3529774644 | [Investment Queue](https://steamcommunity.com/sharedfiles/filedetails/?id=3529774644) | 우측 건설·투자풀 요약 | 참고 | F01,F04 | 미재검증 |

## 추가설정 (5개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3762039832 | [Ctrl+Z (Undo)](https://steamcommunity.com/sharedfiles/filedetails/?id=3762039832) | 지원하는 일부 조작만 이전 설정으로 복원 | 참고 | F06 | 미재검증 |
| 3782096132 | [Population-Based Incorporation Speed](https://steamcommunity.com/sharedfiles/filedetails/?id=3782096132) | 인구에 따라 주 편입 속도 보정 | 밸런스 | F07 | 미재검증 |
| 3784758261 | [Ten-thousand and Hundred-million](https://steamcommunity.com/sharedfiles/filedetails/?id=3784758261) | 만·억 등 동아시아식 숫자 단위 표시 | 참고 | F18 | 미재검증 |
| 3785172318 | [Bolster and Suppress Tiers](https://steamcommunity.com/sharedfiles/filedetails/?id=3785172318) | 정치운동 지지·억압 강도 선택 | 규칙변경 | F14 | 미재검증 |
| 3769526100 | [Call a Party Election](https://steamcommunity.com/sharedfiles/filedetails/?id=3769526100) | 특정 정치 교착 상황에서 재선거 | 규칙변경 | F14 | 미재검증 |

## 세부UX (2개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 2979696033 | [Improved LessClicks](https://steamcommunity.com/sharedfiles/filedetails/?id=2979696033) | 필요 기반시설 계산·대기열 앞 30개 삭제·대기열 10페이지 이동 | 참고 | F01,F03 | 미재검증 |
| 2882752079 | [Better Canals](https://steamcommunity.com/sharedfiles/filedetails/?id=2882752079) | 운하를 실질 경제 활동에 연결 | 구형참고 | F17 | 미재검증 |

## 게임플레이 (4개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3282418847 | [Automobiles Won't Destroy Your Railways Anymore](https://steamcommunity.com/sharedfiles/filedetails/?id=3282418847) | 자동차 도입 뒤 철도 수요 급락 완화 | 밸런스 | F17 | 미재검증 |
| 3278371002 | [Private Investments Only Expand Existing Buildings](https://steamcommunity.com/sharedfiles/filedetails/?id=3278371002) | 민간 투자 신규건설 억제·기존 시설 확장 우선 | 밸런스 | F11 | 미재검증 |
| 3284950938 | [Remove Unwanted Buildings](https://steamcommunity.com/sharedfiles/filedetails/?id=3284950938) | 비용을 지불하고 불필요한 건물 정리 | 규칙변경 | F01,F17 | 미재검증 |
| 3537145231 | [Land Battle Fixed](https://steamcommunity.com/sharedfiles/filedetails/?id=3537145231) | 방어 지휘관 선정 및 전쟁 목표 지역 우선 개선 | 밸런스 | F13 | 미재검증 |

## 후속자동화 (6개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3007279569 | [Production Method Auto Conversion (Glitch Fixed)](https://steamcommunity.com/sharedfiles/filedetails/?id=3007279569) | 정복·교환·주 이전 시 자국의 다수 생산방식에 통일 | 후속판 | F02 | 재열람 |
| 2951586988 | [Improved Auto-Expand](https://steamcommunity.com/sharedfiles/filedetails/?id=2951586988) | 철도·행정청·발전소 상황에 따른 자동 확장 | 참고 | F03 | 미재검증 |
| 3168149565 | [Improved Improved Auto-Expand](https://steamcommunity.com/sharedfiles/filedetails/?id=3168149565) | 자동 확장 조건의 과잉건설 제한 | 후속판 | F03 | 미재검증 |
| 3786344818 | [News Events](https://steamcommunity.com/sharedfiles/filedetails/?id=3786344818) | 국제뉴스·역사 사건·국가별 신문 | 참고 | F16,F05 | 미재검증 |
| 3786321132 | [Smart Economy Full](https://steamcommunity.com/sharedfiles/filedetails/?id=3786321132) | 경제 자동 건설·PM·관세·민간 투자·기술 이전 통합 | 고위험 | F01,F02,F10,F11 | 재열람 |
| 3592862659 | [[1.13] Halved Employment](https://steamcommunity.com/sharedfiles/filedetails/?id=3592862659) | 고용·투입·산출·자원 상한 등을 광범위하게 수정 | 대형개편 | F17 | 미재검증 |

## 개편제외 (1개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3819399957 | [Production Method Rework](https://steamcommunity.com/sharedfiles/filedetails/?id=3819399957) | 생산방식 전환에 비용과 점진적 전환 시스템 추가 | 대형개편 | F02,F17 | 미재검증 |

## 후속정보 (7개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3600893109 | [UI – Latest Research Indicator](https://steamcommunity.com/sharedfiles/filedetails/?id=3600893109) | 최근 연구 완료 기술 최대 3개 표시 | 참고 | F05,F14 | 미재검증 |
| 3645252084 | [Coup Possibility Tooltip](https://steamcommunity.com/sharedfiles/filedetails/?id=3645252084) | 쿠데타 가능성의 내부 판단 정보 표시 | 참고 | F14 | 미재검증 |
| 3669482073 | [State Assets and National Capital Tracking](https://steamcommunity.com/sharedfiles/filedetails/?id=3669482073) | 국유자산·민영화 가치·국고·전년 대비 변화 추적 | 참고 | F04 | 미재검증 |
| 3341735910 | [See Useful Info](https://steamcommunity.com/sharedfiles/filedetails/?id=3341735910) | 전투 너비·지휘관 능력·노동력·평균 생산성 확대 | 참고 | F04,F13 | 미재검증 |
| 3522882173 | [Better War Info](https://steamcommunity.com/sharedfiles/filedetails/?id=3522882173) | 교전국·전선·군단별 병력·사기·조직력 표시 | 위험검증 | F13 | 미재검증 |
| 3663825839 | [Transparent London Conference](https://steamcommunity.com/sharedfiles/filedetails/?id=3663825839) | 런던 회의의 강대국 지지안 공개 | 참고 | F12,F14 | 미재검증 |
| 3626829744 | [Upgradable Everything Updated](https://steamcommunity.com/sharedfiles/filedetails/?id=3626829744) | 군 병종 간 업그레이드 경로 추가 | 규칙변경 | F13 | 미재검증 |

## 제외사례 (1개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 2889288063 | [Save your CPU](https://steamcommunity.com/sharedfiles/filedetails/?id=2889288063) | POP 문화·종교 강제 변경을 동반할 수 있는 성능 모드 | 기본제외 | F18 | 미재검증 |

## 모음집 (1개)

| ID | 모드 | 핵심 기능 | 상태/주의 | 연계 기능 | 검증 상태 |
| --- | --- | --- | --- | --- | --- |
| 3773905598 | [by天草 제작 모드 모음집](https://steamcommunity.com/sharedfiles/filedetails/?id=3773905598) | 다수 소형 QoL·규칙 변경 모드의 집합; 상호 호환성 주의 | 모음집 | F01,F06,F12,F14 | 재열람 |

## 직접 링크가 확보되지 않은 보조 사례·아이디어

이 표에 대해 실제 창작마당 게시물이나 작동을 확인한 것으로 인용하지 않는다.

| 항목 | 원본 대화상의 근거·상태 | 연계 |
| --- | --- | --- |
| Statistics Korean | 한국어 통계 화면 참고 사례; 이전 조사에서 언급, 링크 미확보 | F04,F18 |
| Custom Government Type Names | by天草 제작 모음집의 정부 유형 명칭 개인화 모드, 개별 링크 미확보 | F06 |
| Colored Market Price Graph | GORA UI+ 원문 크레딧에서 이름 확인, 독립 ID 미확보 | F04 |
| Nuanced Charts+ | GORA UI+ 크레딧에서 이름 확인, 독립 ID 미확보 | F04 |
| Pop Acceptance Chart | GORA UI+ 크레딧에서 이름 확인, 독립 ID 미확보 | F15 |
| Collapsable Buildings List | GORA UI+ 크레딧에서 이름 확인, 독립 ID 미확보 | F01,F06 |
| V3_mod_GDP_Chart | GORA UI+ 크레딧에서 이름 확인, 독립 ID 미확보 | F04 |
| More Icons For Different Orders | GORA UI+ 크레딧에서 이름 확인, 독립 ID 미확보 | F13 |
| 세계 상품 순위 확대 | 모드 조합 사례집에 언급, 개별 워크숍 ID 미확보 | F04 |
| GDP 그래프 UI 간소화 | 모드 조합 사례집에 언급, 개별 ID 미확보 | F04 |
| IBM Plex Font 등 대체 글꼴 | 대표 사례의 개별 링크 미확보 | F18 |
| Back to Top Button | 명칭만 이전 조사에서 확인, 링크 미확보 | F06 |
| UI – Capital state indicator | 명칭만 이전 조사에서 확인, 링크 미확보 | F06,F13 |
| 건설창에서 자원 매장량 노출 | 커뮤니티 제안; 기성 모드 확인되지 않음 | F01,F04 |
| More Treaty Binding Periods | 모음집 내부 항목으로 확인, 독립 ID 미확보 | F10,F12 |
| Better Military Assistance | 모음집 내부 항목, 독립 ID 미확보 | F12,F13 |
| Custom Country 시리즈 | 모음집 내부 항목, 개별 ID 미확보 | F06 |
| Dense Market Details | 개별 링크 미확보 | F04,F06 |
| Complete Pop List | 개별 링크 미확보 | F15 |
| Dense Military Tab | 개별 링크 미확보 | F13 |
| More Spreadsheets | 개별 링크 미확보 | F04 |
| Pops Overview Radicals | 개별 링크 미확보 | F15 |
| Outliner 확장 | 특정 ID 미확보 | F05,F06 |
| Improved Building Grid | 개별 링크 미확보 | F02,F06 |
| Canal Dividend Income | 개별 링크 미확보 | F17 |
| Universal Names | 개별 링크 미확보 | F18 |
| Clear Map 계열 | 개별 링크 미확보 | F18 |
| Construction Queue with States | 구형·본편 기능 흡수 가능; 재검증 필요 | F01 |
| Practical Heatmaps | 구형·본편 기능 흡수 가능; 재검증 필요 | F06 |
| Deal or no deal? | 구형·본편 기능 흡수 가능; 재검증 필요 | F10 |
| Dense Trade Routes Tab | 구형·본편 기능 흡수 가능; 재검증 필요 | F10 |
| Production Method Auto Conversion (원본) | 후속 Glitch Fixed를 대표로 수록 | F02 |

## 목록 활용법

- 동일한 기능군에 여러 모드가 있는 경우 최신 버전 대응과 GUI override 범위를 비교해 하나의 구현 방향을 선택한다.
- 후속판은 구형 원본 대신 후속판부터 검토한다. 예: `Production Method Auto Conversion (Glitch Fixed)`.
- 여러 경제 분야를 동시에 바꾸는 모드는 **기능별로 분리해 참고**한다. 예: `Smart Economy Full`, `Grey's Soft Econ Adjustments`.
- 삭제·차단·버전 비호환·수정 폐기 여부는 최종 패키징 직전에 Steam 원문과 인게임에서 재확인한다.
