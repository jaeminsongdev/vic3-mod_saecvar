# SAECVAR: Vanilla Plus 조사 자료 전체 합본

---

<!-- 01_PROJECT_CHARTER.md -->

# 01. 프로젝트 헌장 및 조사 범위

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

## 목적

Victoria 3의 시대·국가·경제 체계를 유지하면서 클릭 횟수와 정보 탐색 부담을 줄이고, 반복 작업 자동화·소규모 밸런스 개선을 선택적으로 제공한다. 일부 기능은 새로운 외부 시스템을 창작하는 것보다 기존 게임 규칙을 정확히 보여주고 조작 가능하게 하는 데 집중한다.

## 포함/제외 기준

| 범주 | 포함 | 제외 또는 별도 프로젝트 |
| --- | --- | --- |
| UI/UX | 정보 표시·정렬·검색·일괄 조작·알림·단축키 | 본편 화면을 통째로 대체하는 과도한 변경은 기본 제외 |
| 자동화 | 건설/PM/기반시설/외교 등 반복 작업 조건부 수행 | 플레이어 승리 보장·게임 규칙 무시 |
| 밸런스 | 동화·개종·악명 감쇠·편입·칙령의 제한된 보정 | 전면적 경제 생산식·인구·전쟁 개편 |
| 성능 | 불필요한 반복 계산·GUI 비용 제거, 계측 | POP 문화를 강제 일치시키는 식의 게임성 훼손 |
| 현지화 | 한국어·수치 단위·글꼴·접근성 | 국가 콘텐츠·역사 전면 개편 |

## 우선 원칙

1. **예측 가능성**: 자동화 이전에 조건·대상·결과·비용을 보여준다.
2. **대칭성**: 밸런스 기본값은 플레이어만 특혜를 받지 않게 설계한다. AI 적용 가능성은 기술 검토 과제로 남긴다.
3. **비침습성**: 가능한 경우 기존 파일 교체보다 추가 정의를 사용한다. UI override가 불가피하면 충돌 목록을 명시한다.
4. **기본 비활성·명시적 선택**: 게임 규칙이 바뀌는 모듈은 기본값을 보수적으로 잡거나 비활성화한다.
5. **게임 버전 분리**: 1.13과 1.14 베타의 동작 가정은 공유하지 않는다. 1.15 릴리스 시 별도 재검토한다.
6. **성능의 계측 가능성**: 모드 적용 전후 후반 세이브의 로딩·틱 처리 시간을 비교한다.
7. **증거 수준 명시**: 실제 모드 설명, 사용자 요청, 개발 제안, 구현 검증을 구분한다.

## 범위 밖으로 유지할 연구 주제

- 지도를 교체하는 610년 기반의 별도 대형 모드, 새로운 세계관·국가 체계.
- 생산식 전체 교체, PM 단계별 업그레이드의 대규모 엔진 수준 구현.
- 건물·POP 구조를 광범위하게 재설계하는 고용 반감 모드류.
- 전 게임 상태를 되감는 '무제한 Ctrl+Z'. 스크립트로 통제 가능한 일부 설정 복원만 검토한다.
- 게임 엔진 코드 수준 FPS·AI 연산 변경. 모드 스크립트의 가능한 범위만 검토한다.

## 성과 지표(제안)

- 건설/PM/경제 진단 시 반복 클릭 수 및 완료 시간 감소.
- 자동화 적용 후 기반시설 부족·전력 부족·예산 적자 발생 횟수.
- UI 오류, 키보드 충돌, GUI 동일 파일 덮어쓰기 건수.
- 게임 5년·20년 진행 후 이탈·동화·악명·산업 비중의 원본 대비 변화.
- 플레이어/AI 별 지표 편차, 세이브 로드·업데이트 호환 회귀 통과율.

---

<!-- 02_WORKSHOP_CATALOG.md -->

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

---

<!-- 03_FUNCTIONAL_REQUIREMENTS.md -->

# 03. 기능 요구사항 명세서

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

기능 ID `F01`~`F18`은 모드 이름이 아니라 독립적인 요구사항 범주다. P0/P1/P2는 출시 순서를 나타내며 기능의 실현 가능성을 보장하지 않는다.

## 우선순위 요약

| ID | 기능군 | 우선순위 | 구현 목표 |
| --- | --- | --- | --- |
| F01 | 건설·대기열 관리 | P0 | 다중 주/다중 건물 선택, 최소 인프라 1단계 일괄 건설, 단계별 예약, 대기열 우선순위·일괄 이동/삭제·안전한 축소 |
| F02 | 생산방식(PM) 관리 | P0 | 수익성/가격·노동력·자원 공급을 비교하여 자동 PM 추천·선택적 적용; 신규 획득 영토 PM 통일; 전기 공급 후 전환 |
| F03 | 기반시설 자동화 | P0 | 철도·전력·조세 역량·항구·시장접근·행정 역량 부족 경보와 조건부 자동 확장 |
| F04 | 경제 통계·정보 | P0 | 시장 수급·가격 추세·기업·무역 상대·투자풀·국유자산·GDP/POP 비교·민간 소유지분 |
| F05 | 경고·알림 | P0 | 지휘관 미배치, 연구 완료, 미제정 법률, 전력 부족, 공급망 중단, 건설 중지, 국제사건 중요도 필터 |
| F06 | UI·조작 편의 | P0 | 상단바·고밀도 목록·검색 정렬·기본 단축키·Enter 확인·지원되는 조작 한정 Undo·접근성 |
| F07 | 동화·개종·편입 | P1 | 동화/개종 속도 조정, 소규모 POP 처리, 문화적 거리·인구·식민지 상태별 편입 효과 |
| F08 | 악명·외교 균형 | P1 | 기본 악명 감쇠·악명 수준별 추가 감쇠·문화 연관성에 따른 정복 악명 조정 |
| F09 | 칙령·행정 | P1 | 주 인구/보유 주 수에 비례하는 칙령 권위비용, 수익성 높은 소비세 표시, 칙령 대상 분석 |
| F10 | 무역·관세·조약 | P1 | 조약 예상 순이익·운송비·우선 교역 상대·다중 관세 변경·자동화·만료 조약 접근 |
| F11 | 민간투자·기업 | P1 | 민간투자 업종/주/수익성 필터, 과잉투자 억제, 기존 건물 확장 우선, 안전한 국영건물 축소 |
| F12 | 속국·외교 관리 | P1 | 관계개선 자동화·속국 법률/생산방식/조공·외교행동 가용 알림·조약 기간 설정 |
| F13 | 육군·해군·전쟁 | P1 | 지휘관 자동 모집, 부대 표시, 수송 부족 경고, 전선·전쟁목표 가시화, 해군 명령 간소화 |
| F14 | 정치·법률·기술 | P1 | 의회/선거 가시화·법률 알림·연구 완료 이력·쿠데타 위험 툴팁 |
| F15 | POP·생활수준 진단 | P1 | 급진파·실업·자격·문화 수용·상품 소비를 주/계층/직업별로 분해 표시 |
| F16 | 뉴스·통계·역사 기록 | P2 | 사건·연구·정복 연혁, 세계 GDP/POP, 업적 진행, 중요 국가 알림 |
| F17 | 소폭 경제·산업·전쟁 밸런스 | P2 | 자동차/철도 수요 보정, 운하, 건물 철거비, 주별 세제(옵션), 병참/전쟁 피로도 미세 조정 |
| F18 | 성능·접근성·현지화 | P2 | 불필요한 상시 검사 최소화, 글꼴/단위/지도/스크롤, 한국어 로컬라이즈 |

## F01. 건설·대기열 관리 — P0

**목적:** 다중 주/다중 건물 선택, 최소 인프라 1단계 일괄 건설, 단계별 예약, 대기열 우선순위·일괄 이동/삭제·안전한 축소.

**완료 기준 및 제약:** 기존 비용·건설력·법률을 우회하지 않음; 같은 주·건물을 중복 예약하지 않음.

**구현 전 확인:** UI 제한·이벤트·효과·건설 트리거 확인.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 다중 주 건설 | 공사 대상을 여러 주로 묶고 단계 수를 지정 | 필요한 정상 건설비·대기열 유지 |
| 기본 기반시설 일괄 배치 | 철도·발전소·행정청·항구의 최소 필요치만 예약 | 이미 충분한 지역 중복 방지 |
| 대기열 일괄 관리 | 다중 선택·상하 이동·삭제·페이지 이동 | 의도하지 않은 전체 삭제 방지 |
| 안전한 구조조정 | 불필요 시설과 국영 건물의 축소·이전 추천 | 철거·이전 비용 또는 제한 준수 |

## F02. 생산방식(PM) 관리 — P0

**목적:** 수익성/가격·노동력·자원 공급을 비교하여 자동 PM 추천·선택적 적용; 신규 획득 영토 PM 통일; 전기 공급 후 전환.

**완료 기준 및 제약:** 적용 전후 상품 수급 및 고용을 확인; 특정 산업/주 제외; 사용자 선택 우선.

**구현 전 확인:** PM 선택 스크립트·동적 계산의 노출 범위 확인.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 정복지 PM 자동 통일 | 새 영토의 자국 다수 생산방식 적용 | 기술 미해금/공급 부족이면 스킵 |
| 수익성 기반 PM 추천 | 입출력 가격·임금·현지 고용 고려 | 변경 전후 비교 가능 |
| 전기화 연계 | 발전소 수요·공급 확보 후 전기 PM 허용 | 예상 부족 시 적용 보류 |
| 선택형 자동화 | 주·산업·PM 그룹별 on/off | 사용자 수동 결정 우선 |
| 경고 세분화 | 가격 충격·노동력 부족·경고 생략 구분 | 경고 완전 끄기도 독립 옵션 |

## F03. 기반시설 자동화 — P0

**목적:** 철도·전력·조세 역량·항구·시장접근·행정 역량 부족 경보와 조건부 자동 확장.

**완료 기준 및 제약:** 수요 기준 초과 건설 금지; 자원·건설력·예산·예약 단계 상한.

**구현 전 확인:** 주별 수요·공급 지표 접근성.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 철도 자동확장 | 기반시설 부족량 감지·정상 예약 | 중복 예약·적자 누적 상한 |
| 발전소 자동확장 | 지역 전기 소비 목표 충족 | 전기 수요 없는 주에 자동 신설 금지 |
| 행정청·항구·무역시설 | 조세·항만·시장접근 부족을 각각 감시 | 정책별 허용·비용·산업 유형 확인 |

## F04. 경제 통계·정보 — P0

**목적:** 시장 수급·가격 추세·기업·무역 상대·투자풀·국유자산·GDP/POP 비교·민간 소유지분.

**완료 기준 및 제약:** 원본 숫자와 일치; 값의 집계 시점·범위 명시; 성능 저하 억제.

**구현 전 확인:** GUI 데이터 바인딩 및 제공 지표 확인.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 시장 스냅샷 | 상품 수요/공급·가격·전주 대비 변화 | 실제 조회값만 보여주기 |
| 국유자산·기업 | 국유 자산 평가, 투자풀, 기업별 정렬 | 국유자산 계산 누락 범위 명시 |
| 국가/주/세계 통계 | GDP·인구·생활수준·실업·재정 비교 | 통계 범위를 도구 설명에 표기 |
| 조약·교역 데이터 | 국가별 거래 상대·순이익·운송비 추정 | 예측 수치와 정산 수치 분리 |

## F05. 경고·알림 — P0

**목적:** 지휘관 미배치, 연구 완료, 미제정 법률, 전력 부족, 공급망 중단, 건설 중지, 국제사건 중요도 필터.

**완료 기준 및 제약:** 경고 사용자 설정; 경고 폭주 방지; 유사 알림 병합.

**구현 전 확인:** 게임 알림 훅·조건 노출 검토.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 적용 가능한 행동 알림 | 지휘관 미배치, 연구, 법률, 세력권 행동 | 행동불가 시 경고 숨김 |
| 경제 위험 알림 | 전기·기반시설·품귀·투입재 위험 | 경고 빈도 제한·병합 |
| 글로벌 뉴스 필터 | 중요국·자국 연관성·사건 유형 선택 | 알림 폭주 방지 |

## F06. UI·조작 편의 — P0

**목적:** 상단바·고밀도 목록·검색 정렬·기본 단축키·Enter 확인·지원되는 조작 한정 Undo·접근성.

**완료 기준 및 제약:** 모드 충돌 탐지; 버튼 중복·클릭 영역·색각·키 충돌 점검.

**구현 전 확인:** GUI 파일 충돌과 CMF 연동 검토.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 한국어 고밀도 UI | 긴 문자열·만/억 단위·툴팁·가독성 | 텍스트 겹침 없음 |
| 일괄 버튼과 필터 | 정렬·검색·핀 고정·빠른 건설 | 기존 버튼/권한과 충돌 없음 |
| 키보드 접근성 | Enter 확인, 스크롤 속도·단축키 | 충돌하는 단축키 사용자 설정 |
| 한정 실행 취소 | 지원 가능한 설정 변경만 이전 값 복원 | 시간 진행/AI 변경 상태까지 되돌린다고 약속하지 않음 |

## F07. 동화·개종·편입 — P1

**목적:** 동화/개종 속도 조정, 소규모 POP 처리, 문화적 거리·인구·식민지 상태별 편입 효과.

**완료 기준 및 제약:** 차별·수용·문화적 유사성 무시 금지; 과도한 인구 일색화 억제.

**구현 전 확인:** 수치 수정 가능한 영역 vs 하드코딩 구분.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 조건부 동화·개종 | 수용/차별/주 편입 조건을 존중한 미세 보정 | 원본 문화 구성 붕괴 방지 |
| 주 편입 시간 조정 | 문화 유사성·인구·식민지 여부 보정 | 정치 비용·기본 한계 유지 |
| 소규모 POP 처리 | 연산량 감소 가능성 연구 | 실제 엔진 메커니즘 확인 후 시행 |

## F08. 악명·외교 균형 — P1

**목적:** 기본 악명 감쇠·악명 수준별 추가 감쇠·문화 연관성에 따른 정복 악명 조정.

**완료 기준 및 제약:** 감쇠 상한, AI와 플레이어 동일 적용, 정복 도미노 방지.

**구현 전 확인:** 공식 패치 변경과 중복 검토.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 악명 감쇠 프로필 | 고정 배율 + 조건부 감쇠·상한 | 대국 공격 연쇄 유발 여부 체크 |
| 영토 악명 발생량 | 관련 문화 인구 비율 등 추가 변수 | 핵심 밸런스 테스트 |

## F09. 칙령·행정 — P1

**목적:** 주 인구/보유 주 수에 비례하는 칙령 권위비용, 수익성 높은 소비세 표시, 칙령 대상 분석.

**완료 기준 및 제약:** 최대 할인 상한·의미 있는 권위 비용 유지.

**구현 전 확인:** 칙령 비용 계산 파일 충돌 점검.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 대국 칙령 비용 | 주 수·인구 기반 권위 부담 미세 보정 | 할인 상한 설정 |
| 칙령 수익 정보 | 칙령 적용 대상·권위 100당 세수 | 원본 데이터 일치 |

## F10. 무역·관세·조약 — P1

**목적:** 조약 예상 순이익·운송비·우선 교역 상대·다중 관세 변경·자동화·만료 조약 접근.

**완료 기준 및 제약:** 시장 규칙·법률 제한 무조건 존중, 수익 예측과 실현값 구분.

**구현 전 확인:** 1.9 이후 무역 개편 반영, 접근성 확인.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 조약 손익 계산 | 상품·운송비·기회비용·장기 계약 표시 | 예측과 실제 분리 |
| 관세 일괄/자동화 | 상품 그룹별 일괄 적용·조건부 자동 조절 | 법률 제한 우회 금지 |
| 만료 계약 처리 | 만료 조약 필터·취소/갱신 빠른 접근 | 실행 전 확인 |

## F11. 민간투자·기업 — P1

**목적:** 민간투자 업종/주/수익성 필터, 과잉투자 억제, 기존 건물 확장 우선, 안전한 국영건물 축소.

**완료 기준 및 제약:** AI 투자 판단과 플레이어 대칭성·민간투자 원칙 보존.

**구현 전 확인:** 민간 경제 AI의 스크립트 제어 범위 검증.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 민간투자 필터 | 산업/주/수익성·실업 기준 | 민간경제 핵심 제약 보존 |
| 과잉 건설 방지 | 적자·시설 중복·원료 공급 부족 시 제한 | AI 투자의 종합 결과 확인 |
| 국영시설 안전 축소 | 유휴·적자 지속 시 후보 제시 | 자동 실행 기본 꺼짐 |

## F12. 속국·외교 관리 — P1

**목적:** 관계개선 자동화·속국 법률/생산방식/조공·외교행동 가용 알림·조약 기간 설정.

**완료 기준 및 제약:** 자유 열망·관계·권한 제한 존중; 외교 자원 비용 존재.

**구현 전 확인:** 관계·속국 효과 scope 확인.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 관계개선 자동화 | 외교관계 수준/영향력 여유 시 수행 | 관계 적대·권한 제약 반영 |
| 속국 관리 | 법률·조공·생산방식·해방시 초기법 선택 | 종주국 권한·대응 비용 |
| 외교 경고 | 세력권 행동 가능 및 속국 이탈 위험 | 자국 관련 항목 우선 |

## F13. 육군·해군·전쟁 — P1

**목적:** 지휘관 자동 모집, 부대 표시, 수송 부족 경고, 전선·전쟁목표 가시화, 해군 명령 간소화.

**완료 기준 및 제약:** 1.14 베타/1.15 변경 반영; 전쟁 지원 효과 과도 완화 금지.

**구현 전 확인:** 전쟁 GUI·유닛 업그레이드 한계 검토.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 군대 관리 | 지휘관 모집·아이콘·편제 경고 | 부대 편제 변경 정상 제한 |
| 수송·전선 정보 | 해군 수송 부족·전선 병력·조직력·전쟁목표 | 패치별 전쟁 UI 변경 추적 |
| 선박·병종 변경 | 선박 일괄 이전·허용 업그레이드 정리 | 성능치·비용·기술 제한 보존 |

## F14. 정치·법률·기술 — P1

**목적:** 의회/선거 가시화·법률 알림·연구 완료 이력·쿠데타 위험 툴팁.

**완료 기준 및 제약:** 중복 법률 입법은 기본 제외; 투명성 중심.

**구현 전 확인:** 정치/법률 이벤트 및 UI API.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 정치정보 | 의회 구성·선거·쿠데타 위험 | 감춰진 데이터 공개의 공정성 검토 |
| 연구기록 | 최근 완료 기술 표시·알림 이력 | 오래된 기록 중복 방지 |
| 정치행동 | 법안 미제정 알림·개별 보조 기능 | 동시법 제정은 기본 비활성 |

## F15. POP·생활수준 진단 — P1

**목적:** 급진파·실업·자격·문화 수용·상품 소비를 주/계층/직업별로 분해 표시.

**완료 기준 및 제약:** 원인과 단순 상관관계 구분, 정보 집계 지연 표시.

**구현 전 확인:** POP 집계 성능·계산 가능성 확인.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 생활수준 원인 | 실제 소비재 가격·직업·주·문화별 급진파 분석 | 복잡한 인과관계 단정 금지 |
| 인구 표 | 실업·교육·자격·문화 수용·소유지분 | 데이터 접근 가능 범위만 표시 |

## F16. 뉴스·통계·역사 기록 — P2

**목적:** 사건·연구·정복 연혁, 세계 GDP/POP, 업적 진행, 중요 국가 알림.

**완료 기준 및 제약:** 오래된 이벤트 중복 기록 방지; 국가 관련성 필터.

**구현 전 확인:** 세이브 내 지속 데이터·이벤트 훅 검토.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 국가 역사 | 게임 시작 이래 사건·전쟁·제도 기록 | 저장 규칙·세이브 호환 검증 |
| 국제뉴스 | 관계 높은 국가만 중요도 필터로 표시 | 알림 중복 및 역사적 부정확성 방지 |
| 성과 통계 | 세계 GDP·POP 순위·도전과제 상태 | 업적 달성 판정은 바닐라와 분리 |

## F17. 소폭 경제·산업·전쟁 밸런스 — P2

**목적:** 자동차/철도 수요 보정, 운하, 건물 철거비, 주별 세제(옵션), 병참/전쟁 피로도 미세 조정.

**완료 기준 및 제약:** 시스템 개편급 변경은 별도 프로젝트; 원본 경제 루프 보존.

**구현 전 확인:** 가격·운송 수요·전쟁 효과의 하드코딩 여부.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 운송 수요 | 자동차 도입 후 철도 서비스 급락 완화 | 산업별 수지균형 확인 |
| 국지 경제 규칙 | 운하·철거비·이전 등 선택형 규칙 | 기본 비활성화 검토 |
| 전쟁 피로/수송 | 전쟁 지원·병참 지표만 약하게 보정 | 전쟁이 무한하거나 즉시 종료되지 않게 테스트 |

## F18. 성능·접근성·현지화 — P2

**목적:** 불필요한 상시 검사 최소화, 글꼴/단위/지도/스크롤, 한국어 로컬라이즈.

**완료 기준 및 제약:** 문화·종교 강제변경 및 AI 무력화 등 게임성 파괴형 최적화 금지.

**구현 전 확인:** 오프라인 점검 + 실제 로드·프레임 측정 구분.

| 세부 요구사항 | 기능 정의 | 인수 조건 |
| --- | --- | --- |
| 한국어 현지화 | 고정 문자열/숫자·단위·폰트·줄바꿈 | 모든 대표 패널에서 테스트 |
| 정적 점검 | 키 충돌·GUI 파일 override·참조 누락·미번역 점검 | 실행 없이 모든 런타임 오류 검증 불가 고지 |
| 성능 측정 | 월말 tick·장기 후반 세이브 비교 | 차이는 동일 버전·동일 시드에서 측정 |

## 횡단 요구사항

- **사용자 제어:** 자동화 허용 범위는 국가·주·건물 유형·업종별 제외 목록과 최대 예약/실행 주기를 가진다. 게임 내에서 실제 제공할 수 있는 설정 UI 방식은 별도로 검증한다.
- **원본 제약:** 법률·기술·예산·시장접근·외교 자원·건설력 등의 제한을 우회하지 않는다.
- **비대칭 방지:** 수치 보정은 양 진영/AI에 일관되게 적용하는 것을 원칙으로 하되, 게임 스크립트로 가능한지 검증한다.
- **정보 표시 원칙:** 계산된 예측값과 게임에서 읽은 실제값을 별도 레이블로 표시한다.
- **오류 격리:** 한 모듈의 결함이 다른 모듈 UI·밸런스에 연쇄 영향을 미치지 않게 의존도를 제한한다.
- **호환성:** 저장 데이터, 로컬라이제이션 키 충돌, GUI 전체 파일 대체, 도전과제·멀티플레이 호환성을 개별 검증한다.
- **후반 성능:** 매일 세계 모든 주를 검사하는 방식보다 주기적 검사·플레이어 선택 조건·메모이제이션 가능성을 우선 검토한다.

---

<!-- 04_USER_DEMAND_GAP_ANALYSIS.md -->

# 04. 플레이어 수요 및 기존 모드의 빈틈

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

## 수요 근거 해석

여기서 `상/중`은 본 조사에서 확인한 반복 노출·사용자 반응·실질적 조작 부담을 바탕으로 한 **정성적 개발 우선순위**다. 동일 표본의 설문 점수나 전체 플레이어 수요 비중을 뜻하지 않는다. 링크가 없는 요구는 이전 조사에서 수집된 사용자 요구를 재정리한 항목으로, 직접 검증된 원문 인용과 구분한다.

| ID | 요구 | 우선순위 | 문제 상황 | 연계 | 해결 상태/차별화 |
| --- | --- | --- | --- | --- | --- |
| D01 | 정복·병합 주의 PM 표준화 | 상 | 반복 클릭이 심하고 공급·고용 충격 위험도 존재 | F02,F05 | 기존 모드 존재; 정책·조건·예외 개선 |
| D02 | 전기화와 발전소 자동 연결 | 상 | 전기 생산시설과 PM를 주별로 다시 맞춤 | F02,F03 | 기존 독립 모드들을 연결하는 기능 |
| D03 | PM 수익성 판단·보호 잠금 | 상 | 산업별 자동화가 경제에 손실을 줄 수 있음 | F02 | 이미 자동 PM 모드 존재; 잠금·목표·안전장치 차별화 |
| D04 | 대량 건설과 대기열 관리 | 상 | 후반 국가 운영에서 수십·수백 번 반복 클릭 | F01 | 기존 일괄 건설 모드 존재; 여러 조건을 연결 |
| D05 | 상품별 교역 상대·시장 가시화 | 상 | 주요 수입국·수출국이 국가 전략에 결정적 | F04,F10 | Trade Partners 높은 직접 추천; 조약 수익 통합 개선 |
| D06 | 민간투자 자동확장 신뢰성 | 중상 | 민간투자가 원치 않는 시설을 만들거나 경제를 악화 | F01,F11 | 투자 금지·추천·예상 손익을 별도로 제공 |
| D07 | 직접 조작보다 예측·제안 우선 | 중상 | 무조건 PM를 최신으로 바꾸면 투입재 부족 | F02,F05 | 추천 → 예약 → 실행의 단계별 정책 |
| D08 | 군대·해군 지휘/수송 알림 | 중 | 편제 오류와 부족한 해군 수송 상태를 놓침 | F05,F13 | No Commander Alert, Better War Info 등으로 부분 구현 |
| D09 | 국제 사건 필터·중요도 | 중 | 전 세계 알림이 과도하거나 반대로 중요 사건을 놓침 | F05,F16 | 선택적 국제뉴스·하이라이트 통합 |
| D10 | 급진파·생활수준 원인 추적 | 중 | 급진파 수치만 보이고 원인이 여러 UI에 분산 | F04,F15 | 인과 단정하지 않는 다차원 진단 |
| D11 | 조약 운송비·예상 순이익 | 중 | 계약 체결 시 숨겨진 비용으로 실현 손익이 변함 | F10 | Smart Treaties의 예측/실현 구분 참고 |
| D12 | 연구 완료·법률·세력권 행동 알림 | 중 | 완료 팝업 누락, 행동 가능 여부 탐색 부담 | F05,F14 | 이미 개별 알림 모드 존재; 통합 필터 개선 |
| D13 | 긴 목록·작은 글자·한국어 단위 | 중 | 과도한 스크롤/정보 밀도/만·억 현지화 불일치 | F06,F18 | 한국어 QA와 접근성 토글 |
| D14 | 완전 실행 취소 대신 제한된 Undo | 중 | 실수로 세율·관세·소비세 등 값을 변경 | F06 | 지원되는 상태만 저장·이전 값 복원 |
| D15 | 악명·동화·편입의 완만한 개선 | 조건부 | 제약이 너무 느리지만 과도한 버프는 전략을 소거 | F07,F08,F09 | 옵션 프로필로 제공; 사용자 수요 정량 근거는 부족 |
| D16 | 성능 저하 없는 후반 대국 운영 | 중 | 후반 월말/주말 연산 지연 | F01,F02,F18 | 자동화 알고리즘이 새 연산 부담을 만들지 않게 설계 |

## 직접 확인할 수 있는 수요 자료

| 요구 ID | 원문/근거 URL |
| --- | --- |
| D01 | [Reddit/원문](https://www.reddit.com/r/victoria3/comments/1ra08jv/the_most_critical_mods_that_should_be_rolled_into/) |
| D02 | [Reddit/원문](https://www.reddit.com/r/victoria3/comments/1svrsfg/at_what_point_do_you_guys_just_turn_on_automatic/) |
| D03 | [Reddit/원문](https://www.reddit.com/r/victoria3/comments/1cns3ig) |
| D04 | [Reddit/원문](https://www.reddit.com/r/victoria3/comments/yzdt23) |
| D05 | [Reddit/원문](https://www.reddit.com/r/victoria3/comments/1waf40o/what_are_must_have_mods_for_victoria_3/) |
| D06 | [Reddit/원문](https://www.reddit.com/r/victoria3/comments/1svrsfg/at_what_point_do_you_guys_just_turn_on_automatic/) |
| D07 | [Reddit/원문](https://www.reddit.com/r/victoria3/comments/1t8g3zx) |

## 중복 개발을 피해야 할 기능

- GORA UI+는 이미 시장·건설·외교·정치 등 광범위한 UI를 묶는다. 그대로 다시 만드는 대신 한국어·충돌 방지·특정 자동화 연결에 차별화한다.
- OGAS와 Smart Economy Full은 건설·PM 전역 자동화 참고 사례다. 본 프로젝트는 안전한 기본값과 설명 가능한 의사결정을 목표로 한다.
- `Production Method Auto Conversion (Glitch Fixed)`가 존재하므로 정복지 PM 정리를 '미구현 수요'로 단정하면 안 된다. 기술·공급·고용 손실 검증과 UI 통합이 차별점이다.
- 1.9 전후 무역 규칙이 변화했으므로 구형 자동 무역로 모드를 기능 근거로 직접 재사용하지 않는다.
- `Latest Research Indicator`, `Decree State Insights`, `No Commander Alert` 등은 이미 구현 사례가 있으므로 개선 목표를 명확히 설정한다.

## 구현되지 않았거나 검증이 필요한 차별화 아이디어

1. 생산방식과 **기반시설 수요를 함께 계산**하여 안전할 때만 적용하는 경제 운영 도우미.
2. 대량 건설 전 **예상 비용·전력/철도 부족·수익성 영향**을 한 번에 제시하는 사전 검증.
3. 급진파 발생 변화를 집계하고 **그럴듯한 기여 요인**을 표시하는 분석(확정 인과라고 표기 금지).
4. 모든 경고에 대해 **중요도·빈도·지역·국가 관련성**을 공통 설정으로 제공.
5. 한국어 표시·만/억 수치 형식·글자 길이 충돌을 통합 처리.
6. 사용자 선택을 존중하는 자동화 일시정지·산업 잠금·조건 미충족 시 사유 노출.
7. 1.13/1.14/1.15의 차이를 테스트하는 버전별 기술·스크립트 호환 계층.

## 사용자 규모 추정 금지

Steam 구독 수, Reddit 업보트, 게시물 댓글은 서로 다른 편향된 지표다. 수요 순위를 수치화하려면 동일 기간·동일 범위의 표본 설문이 필요하다. 모드 인기와 실제 이용률·만족도·안정성을 동일시하지 않는다.

---

<!-- 05_BALANCE_POLICY.md -->

# 05. Balance Lite 정책 및 실험 계획

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

## 핵심 원칙

표의 배율은 사용자가 제시하거나 실험으로 검증한 값이 아닌 **프로토타입의 출발점**이다. 국가·문화·AI·경제에 미치는 장기 영향이 클 수 있으므로 실제 적용할 때 고정 수치라고 가정하지 않는다. 가능하다면 원본의 차별·수용·법률 조건을 보존하고 수정 범위를 속도·비용으로 좁힌다.

## 권장 프리셋 초기 가설

| 항목 | 보수적 | 표준 | 적극적 | 설계 주의 |
| --- | --- | --- | --- | --- |
| 동화 속도 | ×1.10 | ×1.20 | ×1.35 | 차별/수용/편입 조건을 건드리지 않고 속도만 조정하는 가설 |
| 개종 속도 | ×1.10 | ×1.20 | ×1.35 | 법률·수용·문화 조건 유지 |
| 악명 기본 감쇠 | ×1.10 | ×1.20 | ×1.35 | 원본 감쇠량에만 곱하는 기준안 |
| 주 편입 소요 시간 | ×0.95 | ×0.90 | ×0.80 | 기간 감소안; 원본 병목과 영향 확인 |
| 칙령 비용 | 원본 유지 | 규모별 소폭 할인 | 상한 내 추가 할인 | 권위 비용·효과 동시에 영향 점검 |
| PM 자동 전환 | 권고만 | 조건부 단계적 적용 | 적극 적용 | 적극 전환도 기술/공급/고용 조건은 유지 |
| 민간투자 통제 | 경고만 | 과잉투자 제한 | 업종별 통제 | 계획경제화 방지와 AI 형평성 검토 |

### 악명 감쇠의 동적 대안(설계 개념; 실행 스크립트 아님)

- 기본 감쇠: `원본 감쇠 × 기본 배율`.
- 선택적 부가 보정: `악명이 기준치 T 이상일 때만 감소율을 완만하게 증가`.
- 최대 보너스와 전체 감쇠 절대 상한을 둔다. `현재 악명→0` 일괄 제거는 포함하지 않는다.
- AI와 플레이어 적용 여부가 정확히 동일해야 하며, 특정 외교 목표 비용 변화와 중복 계산 여부를 검증한다.
- 악명 상승 원인(정복 문화 구성)을 반영하는 **발생량 조정**과 감쇠량 조정은 별도의 토글로 관리한다.

### 동화·편입 지침

- `동화 속도 상승`과 `동화가 불가능한 대상을 새로 허용`하는 것은 다른 기능이다. 후자를 포함하지 않는 것을 기본으로 한다.
- 편입 소요 시간 보정은 문화·인구·식민지 조건으로 미세 차등화하되 제도 및 행정비용 구조는 유지한다.
- 작은 POP 집단 통합/병합은 문화·종교 구성이 급격히 훼손될 수 있으므로 성능 모드의 필수 기능으로 묶지 않는다.

### 칙령·전쟁 밸런스 지침

- 칙령 권위 할인에 상한을 두고 작은 국가의 상대적 이점이 소멸하지 않게 한다.
- 해군 수송량·전쟁 지지도 조정은 1.14 베타/1.15 예정 변경과 중첩 가능성이 있어 뒤로 미룬다.
- 운하·자동차/철도 등 특정 산업 보정은 기술별 수요·시장 가격 변동을 대조한 뒤에만 적용한다.

## 검증 시나리오

| 테스트 | 플레이 세팅 | 검사 지표 | 성공 조건(상대적) |
| --- | --- | --- | --- |
| A01 | 소국·농업국 1836 시작, 5~20년 | GDP, 편입 속도, 세수, 급진파, 악명 | 기본 모드 대비 극단적 버프/버그 없음 |
| A02 | 대국·산업화 국가, 5~20년 | 건설 수요, PM 오류, 부족 전력·철도, 병목 | 자동화가 경제위기 유발 시 정지 |
| A03 | 다문화 다종교 국가 | 동화·개종·문화 구성 비중 | 차별/수용 조건 무력화 없음 |
| A04 | 정복으로 영토 대량 획득 | 정복지 PM, 악명, 통합 기간 | 비용·보급·법률 제한 준수 |
| A05 | AI 관찰전 플레이 | 플레이어/AI 경제·정복 편차 | 한쪽 진영만 강화되지 않음 |
| A06 | 게임 후반 저장파일 | 월별 연산, 메모리, UI 지연 | 원본 대비 측정 가능·반복 재현 |

## 데이터 기록 규격(제안)

매 테스트에 `game_version`, `mod_version`, `enabled_modules`, `difficulty`, `scenario`, `seed_if_available`, `country`, `year`, `sim_seconds`, `GDP`, `SoL`, `radicals`, `infamy`, `construction_queue`, `unfulfilled_infrastructure`, `unfulfilled_electricity`, `error_count`를 기록한다. 모든 지표를 게임 API에서 직접 읽어낼 수 있다는 뜻은 아니다. 구현 가능한 필드만 추출하고 불가능한 항목은 수동 로그로 대체한다.

## 채택 기준

- 기본 게임 메커니즘을 악용하여 제약을 우회하는 설정은 제거한다.
- 같은 프리셋으로 서로 다른 플레이에서 큰 불안정성이 나타나면 배율보다 적용 조건을 먼저 조정한다.
- 게임 버전이 바뀌면 테스트 기록을 새 버전에 그대로 전용하지 않는다.

---

<!-- 06_ARCHITECTURE_COMPATIBILITY.md -->

# 06. 구조·호환성·검증 파이프라인 설계

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

## 모듈 패키징 (제안)

배포 단위는 다섯 개로 논리적으로 나눈다. 이것은 아직 구현 검증 전의 설계안이며, 게임에서 완전하게 별도 활성/비활성이 가능한지와 패키징 제한을 확인해야 한다.

| 패키지 | 포함 | 의존 관계 | 호환 위험 |
| --- | --- | --- | --- |
| vic3_qol_core | 공통 스크립트·설정키·이벤트·현지화·버전 식별 | 독립 | 공통 키/이벤트 충돌 |
| vic3_qol_ui | 시장·상단바·건설·알림·한국어 UI | core | GUI 전체 파일 교체, CMF/타 UI 모드 충돌 |
| vic3_qol_automation | PM·기반시설·건설·외교 정책 | core | 게임 로직 변경, 스크립트 연산, 경제 효과 |
| vic3_qol_balance_lite | 동화·악명·편입·칙령·운송 소폭 보정 | core | vanilla 스크립트 값 override, 패치 취약 |
| vic3_qol_compatches | 버전별/모드별 호환 패치 | 해당 모듈들 | 업데이트에 따라 파일 동기화 |

## 저장소 구조 예시 (복사 전 확인)

> **역사적 초기 제안:** 아래 독립 저장소 `vic3-qol/` 그림은 대화 중 개발 시나리오였으며, 현재 GitHub의 실제 폴더가 아니다. 채택한 기준은 [SAECVAR 루트 로드맵](../../../docs/PORTFOLIO_ROADMAP.md), [현재 Vanilla Plus 문서](../README.md), [Q0~Q8 공식 로드맵](../planning/MASTER_ROADMAP.md)이다. 논리 모듈 5개를 다섯 개의 별도 Steam 패키지로 출시한다는 결정도 아직 없다.

```text
vic3-qol/
├─ README.md
├─ docs/
│  ├─ 00-research/
│  ├─ 01-spec/
│  ├─ 02-balance/
│  ├─ 03-architecture/
│  └─ 04-qa/
├─ research/
│  ├─ workshop_catalog.csv
│  ├─ community_requests.csv
│  ├─ game_version_matrix.csv
│  └─ external_mod_override_matrix.csv
├─ packages/
│  ├─ vic3_qol_core/
│  │  ├─ .metadata/
│  │  ├─ common/
│  │  ├─ events/
│  │  └─ localization/
│  ├─ vic3_qol_ui/
│  │  ├─ .metadata/
│  │  ├─ gui/
│  │  └─ localization/
│  ├─ vic3_qol_automation/
│  │  ├─ .metadata/
│  │  └─ common/
│  ├─ vic3_qol_balance_lite/
│  │  ├─ .metadata/
│  │  └─ common/
│  └─ vic3_qol_compatches/
├─ tools/
│  ├─ validator/             # 직접 제작할 파서·검증기
│  ├─ conflict_scanner/      # 덮어쓴 경로·정의 키·충돌 감지
│  ├─ vanilla_diff/          # 패치 후 원본 대비 diff
│  ├─ loc_audit/             # 번역·키 충돌·BOM·줄바꿈
│  └─ report/                # JSON·Markdown 검사 보고서
├─ tests/
│  ├─ static/
│  ├─ integration/
│  ├─ save_compat/
│  ├─ performance/
│  └─ ui_visual/
└─ scripts/
   ├─ audit.py
   └─ package.py
```

정확한 하위 디렉터리·구현 파일명은 **현재 설치된 Victoria 3 본편의 경로를 먼저 분석한 뒤** 결정한다. 위의 `common/`, `gui/`, `localization/` 등은 컨벤션을 설명하기 위한 상위 구조 예시다. 기존 600~720년 Twilight 시리즈와 섞지 않고 `Vanilla_Plus/` 독립 폴더에서 구현한다.

## 기능 구현 가능성 분류

| 클래스 | 예시 | 판정 방법 |
| --- | --- | --- |
| UI·로컬라이제이션 중심 | 툴팁, 시장 패널, 알림, 상단바 | GUI/로컬라이제이션 스키마 확인·정적 분석·실행 UI 시험 |
| 일반 스크립트 조작 | 칙령 수치, 일부 조건부 건설·외교 효과 | 트리거·효과 docs와 현재 본편 구현 확인 |
| 경제 데이터 재계산 | 수익성 예측, 주별 상품 수급, 최적 건설 | 스크립트 노출 값과 성능 타당성 탐색 |
| 원본 엔진 제한 위험 | 동화 로직 일부, POP 계산, 전역 되돌리기, AI 내부 판단 | 하드코딩 여부 확인. 불가능하면 대체 방안 또는 기능 제외 |
| 외부 도구 전용 | 정적 파서·중복 키·버전 diff·자동 리포트 | 오프라인 분석으로 구현 가능하나 런타임 보증 불가 |

## 검증 체계: 게임 실행 전 / 실행 후 분리

**A. 오프라인 정적 검사 (게임을 켜지 않고 자동화할 수 있는 것)**

1. Vanilla/모드 파일의 구조, 괄호 균형, 인코딩, 중복 정의, 미존재 참조 및 잠재적 scope 불일치 검사.
2. 패치별 본편 파일 SHA/diff와 override 파일 목록 생성.
3. 로컬라이제이션 키 존재 여부·중복·한국어 fallback 분석.
4. 보유한 예시 저장파일/테스트 fixture 기반 설정 값의 정적 비교.
5. 로그가 **이미 존재한다면** 파싱 및 오류 유형별 집계.

**B. 게임 초기화 또는 로딩이 필요한 검증(디버그 모드 포함)**

- 실제 엔진 파서 통과 여부, 게임 실행 중 `error.log`와 관련 로그, 이벤트 실행·효과 적용·save 호환.
- `-debug_mode`에서는 에러로그 확인이 가능하다. 외부 정적 분석기만으로 게임 런타임의 모든 경고를 판정할 수는 없다.
- GUI 시각적 중첩·폰트·클릭 이벤트, 그래픽 회귀는 실제 게임을 한 번 이상 실행하여 검증한다.
- 기존 사용자 선호대로 **매 코딩마다 게임 실행을 강제하지 않고**, 릴리스 단계의 필수 인게임 확인을 배치할 수 있다.

**C. 실행 조건이 필요한 성능 검증**

- 동일 게임/패치·동일 세이브·동일 모듈 조건에서 FPS·월말 tick·세이브 로드 시간 비교.
- 후반 성능 관련 주장을 정적 코드 개수나 외부 추정만으로 확정하지 않는다.

## 외부 모드/본편과 충돌 검사

- 중복되는 `gui` 경로, 스크립트 정의 키, 생산방식 정의, 시장 패널, 법률/칙령 modifier, 로컬라이제이션 키를 추적한다.
- 특히 GORA UI+와 Community Mod Framework(CMF), Parliament 2.0 및 더 많은 생산방식 열을 추가하는 모드의 관계를 시험한다.
- `GORA UI+` 페이지에는 CMF 지원, 1.14 지원, 로드 순서 및 일부 생산방식 화면 충돌 보고가 존재한다. 모드 설명의 호환 선언과 댓글의 결함 보고를 모두 반영한다.
- OGAS는 기본 생산식만 계산한다고 설명한다. 다른 생산식 수정 모드와 함께 쓰면 수익성 판단이 부정확할 수 있다.
- 최신 본편으로 이미 포함된 기능은 설치된 버전 기준에서 확인한 후 중복 패치를 제거한다.

## 최소 품질 기준

- 별도의 고유 키 네임스페이스 (`qol_` 등) 사용, 전역 객체 이름 중복 방지.
- 한 모듈이 바닐라 전체 파일을 교체해야 한다면 승인 문서와 patch re-diff 테스트를 반드시 추가.
- 자동화 동작 사유와 실패 사유를 기록할 수 있도록 설계(노출 가능한 범위).
- 선택된 설정이 세이브에 유지되는지, 비활성 모듈을 제거해도 세이브가 안정적인지 시험.
- 모든 모드가 Ironman/업적/멀티플레이와 호환된다고 미리 주장하지 않는다.

---

<!-- 07_ROADMAP_QA.md -->

# 07. 로드맵, 우선순위 및 검증 체크리스트

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

## 권장 개발 순서

| 단계 | 주요 작업 | 완료 조건 |
| --- | --- | --- |
| 0. 기준판 고정 | 게임 및 주요 DLC 버전 매트릭스, 대조 저장파일 수집, 가용 트리거·효과·GUI 스키마 조사 | 모든 P0 항목에 실현가능성 판정 / 블로커 등록 |
| 1. 개발·검증 도구 | 정적 파서, 로컬라이제이션 검사, 충돌 탐지, 원본 파일 diff, 로그 수집 | 정적 케이스 재현/자동 보고, 성공·실패 분류 |
| 2. UI 기본 기능 | F04 시장·통계 읽기 전용, F05 경고, F06 상단·정렬·검색 | 기존 UI와 충돌 없이 정보표시; 한국어 테스트 |
| 3. 건설·기반시설 | F01 일괄/최소 건설, F03 부족량 계산·예약 | 불필요 건설·중복 대기열 없이 비용 제약 준수 |
| 4. PM 반자동→자동 | F02 PM 추천·안전 조건·정복지 통일·전기화 연동 | PM 경제적 손실·단계적 전환 경고/일시정지 |
| 5. P1 관리 기능 | F07-15 중 실현 가능·UI 충돌 낮은 순서로 확장 | 자동화와 UI 기능별 회귀 통과 |
| 6. Balance Lite | F07/F08/F09/F17 선택형 프리셋 설계·AI 관찰 | 프리셋 수치 실험 로그와 대칭성 확인 |
| 7. P2 확장·출시 | F16/F18 한국어 뉴스·현지화, 성능 계측, 사용자 문서 | 기능·모듈별 릴리스 노트와 호환성 표 |

## 세부 선후 의존성

```text
게임 버전/스크립트 API 조사
           ↓
정적 검증기 및 버전별 diff
           ↓
읽기 전용 경제/UI 기능(F04-F06)
           ↓
건설 계획·부족량 진단(F01/F03)
           ↓
생산방식 제안 → 안전 적용(F02)
           ↓
소폭 밸런스와 외교/전쟁/정치 확장(F07-F15/F17)
           ↓
현지화·성능·멀티플레이·세이브 검증(F16/F18)
```

기능을 먼저 자동 실행하지 않는다. `정보 표시 → 추천 → 클릭 실행 → 조건부 자동화` 순서로 배포하면 실패 원인을 좁히기 쉽다.

## P0 상세 작업 순서

| 순서 | 작업 | 설명 | 의존 |
| --- | --- | --- | --- |
| 1 | 시장·주·건설 데이터 읽기 | 현재 게임 데이터가 어디서 노출되는지 확인 | 버전/스키마 |
| 2 | 일괄 건설 UI | 설정/버튼·선택 주·건물·예약 단계를 표시 | F06 |
| 3 | 대기열 안전 장치 | 중복·비용·한도·사용자 취소 처리 | F01 |
| 4 | 철도·발전소 부족 경고 | 지표 없을 때는 추정/불확실 표기 | F03,F04 |
| 5 | PM 제안기 | 가격·실업·예상 이익과 예외 사항 표기 | F02,F04 |
| 6 | 정복지 PM 정책 | 획득 이벤트 발생 시 적용 대상/적용 여부 설명 | F02 |
| 7 | 경고 공통 필터 | 소리/팝업/정보창 등 가용 채널별 필터 | F05,F06 |
| 8 | 한국어·디자인 충돌 QA | 긴 국가명/상품명/만·억 수치/화면 배율 | F06,F18 |

## 릴리스 게이트

**G0 — 연구 통과:** 게임 버전, DLC 기준, 스크립트 가용성, 하드코딩 제약 확인. 요구사항마다 `가능/조건부/불가` 표기.

**G1 — 정적 검사 통과:** 파싱·괄호·참조·인코딩·로컬라이제이션·충돌 검사 오류 0건(허용된 경고는 문서화).

**G2 — 게임 로딩 통과:** 실제 게임에서 모드가 로드되고 치명적 에러·잘못된 의도치 않은 override가 없는지 검증.

**G3 — 기능 테스트 통과:** UI 버튼/계산값/건설/PM/외교/밸런스 기능별 시나리오와 수동 확인.

**G4 — 회귀/호환 통과:** 핵심 DLC·게임 버전·세이브·다른 UI 모드와의 조합 테스트. 멀티플레이는 실제 테스트 후 호환 여부 표기.

**G5 — 성능·릴리스 통과:** 동일한 후반 세이브를 기준으로 성능 측정, 릴리스 노트/기능/문제점 공개.

## 핵심 인수 테스트 예시

| 테스트 ID | 검증 시나리오 | 기대 결과 |
| --- | --- | --- |
| T001 | 이미 철도가 충분한 3개 주에 철도 자동 건설 | 중복 예약 0 |
| T002 | 전기가 부족한 주에 전기사용 PM 자동화 | 조건 미충족 사유 노출, 적용 보류 |
| T003 | 전쟁 직후 새 주 획득, 낮은 PM 보유 | 기술·공급·고용 위험 평가 후 정책대로 전환 |
| T004 | 1회에 건설 5단계 예약, 건설력이 부족 | 정상 건설 큐로 예약, 비용·순위 무시하지 않음 |
| T005 | 관세 조정 불가 법률에서 일괄 관세 버튼 | 우회 없이 거부 또는 비활성 |
| T006 | 악명 0/중간/높음, AI와 플레이어 | 감쇠 보정의 상한·대칭성 |
| T007 | 자국 문화 소수 주 편입 프리셋 | 기존 문화·행정 제약 유지 |
| T008 | 한국어 + UI 배율 + 긴 국가명 | 글자 겹침·영어 fallback 누락 최소화 |
| T009 | GORA UI+와 동시 사용 | 충돌 목록 또는 명시적 compatch 제공 |
| T010 | 기본 기능만 설치하고 10년 진행 | 중대 로그 에러·성능 급락 없음 |
| T011 | 기본 저장파일에 Balance Lite 제거 | 지원 정책에 따라 안전하거나 제한 경고 |
| T012 | 게임 패치 후 GUI/스크립트 바닐라 diff | 기존 override의 변경·삭제를 자동 보고 |

## 리스크 우선순위

| 위험 | 심각도 | 대응 |
| --- | --- | --- |
| 게임 버전 변경으로 전쟁/PM UI 스키마 변경 | 상 | 기능별 버전 잠금·바닐라 diff·릴리스 이전 재검증 |
| GUI 전체 파일 override 충돌 | 상 | 변경 파일 최소화, compatch, CMF 호환 시험 |
| 자동 PM로 원자재 가격 급등·실업 | 상 | 추천 모드 우선·산업 잠금·점진적 실행·긴급 정지 |
| 복잡한 매월 검사로 후반 성능 저하 | 상 | 주기/대상 축소, 실측 프로파일링 |
| 악명·동화 조정의 대국/AI 불균형 | 상 | 관찰전과 시나리오 A/B 테스트 |
| 구형 모드 기능의 본편 흡수 | 중 | 게임 버전 매트릭스 및 제거 로직 |
| 미검증 Steam 링크·언어·제작 중단 | 중 | 정식 릴리스 전 개별 모드 원문 점검 |

## 최종 검수 정의

- `문서 검수`: 기능별 ID가 로드맵·카탈로그·인수 테스트와 연결되는지 확인.
- `구현 검수`: 오류 보고서를 기계적으로 생성하고, 파서 경고의 오탐·미탐을 샘플 검증.
- `게임 검수`: 정적 단계 성공과 별도로 로딩·GUI·게임 규칙·세이브를 확인.
- `릴리스 검수`: 결함·호환 불가·설정 기본값을 숨기지 않고 명시.

**주의:** 두 차례 `추가 기능 유형 없음`은 문헌 분류의 수렴 조건이지, 실제 모드가 동작한다는 품질 인증이 아니다.

---

<!-- 08_RESEARCH_AUDIT.md -->

# 08. 조사 방법·누락 검토 기록·한계

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

## 자료 조사 방법

이번 문서는 이전 대화에서 지속적으로 수집한 Workshop 모드와 기능 검토를 **정리·정규화**한 산출물이다. 기능군 수렴은 '연속 두 차례 검토에서 신규 독립 기능군이 더 나타나지 않는다'는 사용자의 종료 기준에 맞추어 정리한다.

| 대상 | 판단 방식 |
|---|---|
| 창작마당 실제 모드 | 제목·ID·설명·분류·버전 기재·수정 댓글 확인 |
| 커뮤니티 요구 | Reddit/모드 댓글에서 반복 불편·미구현 요구 확인 |
| 버전 변경 | Steam/Paradox 개발일지와 모드 제작자 안내 비교 |
| 개발 가능성 | 스크립트·GUI 데이터 접근성, 본편 구조 및 에러로그 검토 |
| 수렴 기준 | 기존 18개 기능군에 속하지 않는 독립 기능군이 추가되는지 검사 |

## 1차 누락 검토: 기능 카테고리 기준

| 검토 영역 | 매핑 | 대조 범위 | 판정 |
| --- | --- | --- | --- |
| UI/정보 | F04,F05,F06,F14,F15,F16,F18 | 대표 UI + 복합 UI + 미세 툴팁 대조 | 기존 범주 내 신규 사례만 존재 |
| 자동화 | F01,F02,F03,F10,F11,F12,F13 | OGAS·PM·민간건설·자동확장·자동외교 대조 | 기존 범주 내 신규 사례만 존재 |
| 밸런스 | F07,F08,F09,F13,F17 | 인구·악명·칙령·전쟁·교통·기업 규칙 대조 | 기존 범주 내 신규 사례만 존재 |
| 성능/현지화 | F06,F18 | 성능 모드 위험 + 한국어·단위·키보드 대조 | 기존 범주 내 신규 사례만 존재 |

## 2차 누락 검토: 요구사항·모드 중복과 경계 기준

| 검토 항목 | 확인 내용 | 판정 |
| --- | --- | --- |
| 카탈로그 링크 | 제목/ID/기능군/주의 표시 대조 | 후속판과 원본 간 중복은 의도적으로 보존 |
| 수요-기능 매핑 | D01-D16 → F01-F18 대조 | 기존 구현 vs 신규 연결 기능 구분 |
| 설계-검증 매핑 | 요구사항 → 릴리스 게이트 → QA 테스트 | 실현 미확인 기술은 블로커로 관리 |
| 범위 관리 | 전체 오버홀·성능 게임성 훼손·엔진 변경 분리 | 기본 프로젝트에 포함하지 않음 |

## 정량적 완성도(문서화 산출물)

| 항목 | 수량/상태 |
| --- | --- |
| Workshop ID를 확보한 사례 | 111개 |
| ID 미확보·구형 참고·기능 제안 | 32개 |
| 통합 기능군 | 18개 (F01-F18) |
| 명시 세부 요구사항 | 56개 |
| 커뮤니티 수요 항목 | 16개 (D01-D16) |
| 문서화 과정 다시 열람한 대표 Workshop 페이지 | 8개 |

## 발견했으나 새로운 기능군으로 세지 않은 세부 기능

- 자동차/철도 수요 보정 → F17.
- 건설 대기열 30개 일괄 삭제·10페이지 점프 → F01.
- 부분 Ctrl+Z·Enter 단축키 → F06.
- 조약별 손익·관세 일괄·만료 처리 → F10.
- 정복지 PM 자동 통일 후속판 → F02.
- 선박/육군 병종 업그레이드 → F13.
- 연구 완료 이력·국유자산 표시 → F14/F04.
- 팝 규모별 주 편입 기간 → F07.
- 한국식 만/억 단위 → F18.

## 한계와 보류 상태

1. Steam Workshop 전체의 모든 등록 항목을 페이지 단위로 훑거나 모드 파일을 내려받아 설치 검증한 **진정한 완전 전수조사**를 했다고 주장하지 않는다.
2. 앞서 수행했다는 두 차례 추가 검토의 결론(새로운 독립 기능군 0)은 **해당 조사 표본의 분류 수렴**을 나타낼 뿐이다. 이번 문서화 단계에서 전체 Steam 목록을 새로 끝까지 크롤링하지 않았다.
3. Steam 링크의 대다수는 이전 대화의 결과를 보존한 것으로 **개별 최신 호환성 재검증 대기**이다.
4. 기능 구현 가능성은 개별 게임 스크립트 API와 버전별 본편 파일 분석 전까지 **설계 가설**이며, 필수 출시 기능이라고 약속할 수 없다.
5. 정확한 구독 수·업데이트 시각·모드 수명은 자주 변하므로 정적인 문서에는 대부분 기재하지 않는다.
6. 고용·동화·AI·전쟁 내부 계산의 일부는 엔진에 하드코딩됐을 수 있다. 스크립트로 접근 불가능하면 규모를 줄이거나 제외한다.

## 재검토를 시작해야 할 조건

- 1.15 정식 출시 또는 새 게임 버전의 경제·전쟁·UI 구조 변경.
- 신규 인기 편의성 모드가 실제로 새로운 기능군을 제안한 경우.
- P0 기능 개발 도중 스크립트 접근이 불가능하거나 큰 성능 부하가 발견된 경우.
- Workshop 기존 모드가 업데이트 중단·삭제돼 참고 구현과 UX가 바뀐 경우.

## 완료 보고 정의

**문서화 관점**에서는 현재 조사 결과를 빠짐없이 18개 기능군으로 정리했으며, 1·2차 범주 누락 점검에서 새로운 독립 기능군은 추가되지 않았다. 다만 **실제 모딩 구현 및 개별 Steam 최신 호환성 전수 검증**은 이후 작업이다.

---

<!-- 09_SOURCE_INDEX.md -->

# 09. 근거 문헌 및 핵심 링크

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

## 공식 문서·패치 변경

- [Paradox: Dev Diary #60 – Modding](https://www.paradoxinteractive.com/games/victoria-3/news/victoria-3-dev-diary-60-modding) — 모드 생성, 경로 대응, `-debug_mode`, `error.log` 설명.
- [Steam: Victoria 3 공식 발표·개발일지](https://steamcommunity.com/app/529340/announcements/) — 게임 버전별 변경 사항의 우선 확인처.
- [Victoria 3 Dev Diary #191 재게시/논의 (2026-10-07)](https://www.reddit.com/r/victoria3/comments/1wzxb1b/victoria_3_dev_diary_191_ai_and_war_support_open/) — 1.14 공개 베타 변경의 1.15 통합 예정 사실. Reddit은 원문 링크 경유 자료이므로 원저자의 공식 페이지 확인 우선.

## 이번 문서화에서 다시 열람한 핵심 Workshop 설명

| 모드 | 설계상 중요점 | 원문 |
| --- | --- | --- |
| GORA UI+ | 시장·건설·외교·의회·국유자산·연구 UI 통합, 1.14 지원 표기 | [Workshop #3724865565](https://steamcommunity.com/sharedfiles/filedetails/?id=3724865565) |
| The OGAS – Your Build Advisor | 수익성 기반 생산방식·노동 균형·건설 자동화; 기본 레시피 한계 | [Workshop #3092955121](https://steamcommunity.com/sharedfiles/filedetails/?id=3092955121) |
| Scaling Infamy Decay | 현재 악명에 따라 감쇠 속도가 달라짐 | [Workshop #3308595632](https://steamcommunity.com/sharedfiles/filedetails/?id=3308595632) |
| Better Decrees 1.13 | 칙령 효과·비용·종류 조정 | [Workshop #2895956555](https://steamcommunity.com/sharedfiles/filedetails/?id=2895956555) |
| Enter Key | 다수 확인·수락 버튼에 Enter 키 사용 | [Workshop #3776818197](https://steamcommunity.com/sharedfiles/filedetails/?id=3776818197) |
| Production Method Auto Conversion (Glitch Fixed) | 정복·교환·주 이전 시 자국의 다수 생산방식에 통일 | [Workshop #3007279569](https://steamcommunity.com/sharedfiles/filedetails/?id=3007279569) |
| Smart Economy Full | 경제 자동 건설·PM·관세·민간 투자·기술 이전 통합 | [Workshop #3786321132](https://steamcommunity.com/sharedfiles/filedetails/?id=3786321132) |
| by天草 제작 모드 모음집 | 다수 소형 QoL·규칙 변경 모드의 집합; 상호 호환성 주의 | [Workshop #3773905598](https://steamcommunity.com/sharedfiles/filedetails/?id=3773905598) |

## 직접 근거 링크가 있는 사용자 수요

| 요구 | 출처 |
| --- | --- |
| 정복·병합 주의 PM 표준화 | [Reddit discussion](https://www.reddit.com/r/victoria3/comments/1ra08jv/the_most_critical_mods_that_should_be_rolled_into/) |
| 전기화와 발전소 자동 연결 | [Reddit discussion](https://www.reddit.com/r/victoria3/comments/1svrsfg/at_what_point_do_you_guys_just_turn_on_automatic/) |
| PM 수익성 판단·보호 잠금 | [Reddit discussion](https://www.reddit.com/r/victoria3/comments/1cns3ig) |
| 대량 건설과 대기열 관리 | [Reddit discussion](https://www.reddit.com/r/victoria3/comments/yzdt23) |
| 상품별 교역 상대·시장 가시화 | [Reddit discussion](https://www.reddit.com/r/victoria3/comments/1waf40o/what_are_must_have_mods_for_victoria_3/) |
| 민간투자 자동확장 신뢰성 | [Reddit discussion](https://www.reddit.com/r/victoria3/comments/1svrsfg/at_what_point_do_you_guys_just_turn_on_automatic/) |
| 직접 조작보다 예측·제안 우선 | [Reddit discussion](https://www.reddit.com/r/victoria3/comments/1t8g3zx) |

## 참고 가능한 비공식 모딩 기술 자료

- [Victoria 3 Paradox Scripting Language 소개 (비공식 정리)](https://github.com/maxzxc0110/Victoria3/blob/main/Victoria_3_Wiki/knowledge_base/09_Misc/Introduction_to_the_Paradox_Scripting_Language.md) — 하드코딩·스크립트/GUI 차이·로그에 대한 이해. 실제 API는 현재 게임 파일 기준 재확인 필요.
- [Paradox 공식 개발일지 #60](https://www.paradoxinteractive.com/games/victoria-3/news/victoria-3-dev-diary-60-modding) — 정적 검사와 엔진 실제 로딩 검사를 구분하는 기초 근거.
- [Community Mod Framework (GORA UI+ 설명을 통한 연결)](https://steamcommunity.com/sharedfiles/filedetails/?id=3724865565) — 공존·로드 순서·UI 충돌 분석의 시작점.

## 출처 신뢰도 등급

| 등급 | 의미 | 사용 방식 |
|---|---|---|
| A | 2026-10-17 페이지 직접 재열람 | 설명 내용 확인, 게임 내 작동은 미검증 |
| B | 앞선 대화 조사에서 개별 ID/링크 확보 | 참고목록 유지, 배포 전 재확인 |
| C | 이름만 확보·커뮤니티 제안 | 기능 요구에만 사용, 실제 모드 존재 단정 금지 |
| D | 설계 가설/실험 배율 | 권장안이 아니라 검증 대기 기준 |

**추가 주의:** Steam 페이지 템플릿의 '삭제/호환되지 않음' 자동 안내 문구는 실제 설명이 노출되는 페이지에서도 보일 수 있다. 모드별 상태를 판정할 때는 제작자 설명·최신 변경 내역·런처/인게임 테스트를 함께 본다.

---

<!-- 10_TRACEABILITY_MATRIX.md -->

# 10. 요구사항 추적성 및 사례 매핑 표

> 작성일: 2026-10-17 · 게임: Victoria 3 · 목적: 원본 세계관과 핵심 게임성을 유지하는 편의성 및 소폭 밸런스 개선 프로젝트
> 문서 상태: 조사 기반 설계안. 현재 게임 내 동작 및 모든 링크의 호환성을 보증하지 않음.

이 문서는 무엇을 참고하여 어떤 기능을 구현하고 어떻게 검증할지 연결한다. 각 기능별로 대표 Workshop 사례 4개를 표시했으며, 해당 모드를 그대로 복제하거나 기능이 완전히 동일하다고 단정하지 않는다. 전체 사례는 `02_WORKSHOP_CATALOG.md`와 `catalog.csv`에 있다.

| 기능 ID | 범주 | 우선순위 | 대표 사례(최대 4건) | 관련 수요 | 대표 인수조건 |
| --- | --- | --- | --- | --- | --- |
| F01 | 건설·대기열 관리 | P0 | [The OGAS – Your Build Advisor](https://steamcommunity.com/sharedfiles/filedetails/?id=3092955121)<br>[Smart Economy Full](https://steamcommunity.com/sharedfiles/filedetails/?id=3786321132)<br>[Advanced Construction Queue](https://steamcommunity.com/sharedfiles/filedetails/?id=3636736389)<br>[Supply Chain Semi-Auto Builder](https://steamcommunity.com/sharedfiles/filedetails/?id=3357811223) | D04, D06, D16 | 필요한 정상 건설비·대기열 유지 |
| F02 | 생산방식(PM) 관리 | P0 | [The OGAS – Your Build Advisor](https://steamcommunity.com/sharedfiles/filedetails/?id=3092955121)<br>[Production Method Auto Conversion (Glitch Fixed)](https://steamcommunity.com/sharedfiles/filedetails/?id=3007279569)<br>[Smart Economy Full](https://steamcommunity.com/sharedfiles/filedetails/?id=3786321132)<br>[Auto-Apply PMs](https://steamcommunity.com/sharedfiles/filedetails/?id=3353797125) | D01, D02, D03, D07, D16 | 기술 미해금/공급 부족이면 스킵 |
| F03 | 기반시설 자동화 | P0 | [The OGAS – Your Build Advisor](https://steamcommunity.com/sharedfiles/filedetails/?id=3092955121)<br>[Supply Chain Semi-Auto Builder](https://steamcommunity.com/sharedfiles/filedetails/?id=3357811223)<br>[Auto Expand for Ports, Trade Centers and Power Plants](https://steamcommunity.com/sharedfiles/filedetails/?id=3775129439)<br>[QOL – Baseline construction](https://steamcommunity.com/sharedfiles/filedetails/?id=3592539197) | D02 | 중복 예약·적자 누적 상한 |
| F04 | 경제 통계·정보 | P0 | [GORA UI+](https://steamcommunity.com/sharedfiles/filedetails/?id=3724865565)<br>[Advanced Market Panel](https://steamcommunity.com/sharedfiles/filedetails/?id=3676318398)<br>[Improved Company List](https://steamcommunity.com/sharedfiles/filedetails/?id=3527702083)<br>[Trade Partners](https://steamcommunity.com/sharedfiles/filedetails/?id=3733651973) | D05, D10 | 실제 조회값만 보여주기 |
| F05 | 경고·알림 | P0 | [No Commander Alert](https://steamcommunity.com/sharedfiles/filedetails/?id=3753749567)<br>[Detailed Character Notifications](https://steamcommunity.com/sharedfiles/filedetails/?id=3633799211)<br>[Idle Law Alert](https://steamcommunity.com/sharedfiles/filedetails/?id=3753665894)<br>[Power Bloc Leader Action Alerts](https://steamcommunity.com/sharedfiles/filedetails/?id=3779275113) | D01, D07, D08, D09, D12 | 행동불가 시 경고 숨김 |
| F06 | UI·조작 편의 | P0 | [GORA UI+](https://steamcommunity.com/sharedfiles/filedetails/?id=3724865565)<br>[Enter Key](https://steamcommunity.com/sharedfiles/filedetails/?id=3776818197)<br>[Voice's Enhanced Topbar](https://steamcommunity.com/sharedfiles/filedetails/?id=3713208305)<br>[Advanced Construction Queue](https://steamcommunity.com/sharedfiles/filedetails/?id=3636736389) | D13, D14 | 텍스트 겹침 없음 |
| F07 | 동화·개종·편입 | P1 | [Colonial Administration Incorporation Fix](https://steamcommunity.com/sharedfiles/filedetails/?id=3741619775)<br>[Grey's Soft Pop Adjustments](https://steamcommunity.com/sharedfiles/filedetails/?id=3336914177)<br>[Merciful Incorporation](https://steamcommunity.com/sharedfiles/filedetails/?id=3613304558)<br>[Population-Based Incorporation Speed](https://steamcommunity.com/sharedfiles/filedetails/?id=3782096132) | D15 | 원본 문화 구성 붕괴 방지 |
| F08 | 악명·외교 균형 | P1 | [Scaling Infamy Decay](https://steamcommunity.com/sharedfiles/filedetails/?id=3308595632)<br>[MW Infamy Decay](https://steamcommunity.com/sharedfiles/filedetails/?id=3121842534)<br>[Configurable Infamy 1.13](https://steamcommunity.com/sharedfiles/filedetails/?id=3736349401)<br>[Recognition Expanded](https://steamcommunity.com/sharedfiles/filedetails/?id=3504963278) | D15 | 대국 공격 연쇄 유발 여부 체크 |
| F09 | 칙령·행정 | P1 | [Better Decrees 1.13](https://steamcommunity.com/sharedfiles/filedetails/?id=2895956555)<br>[Decree State Insights](https://steamcommunity.com/sharedfiles/filedetails/?id=3760606574)<br>[Consumption Tax Efficiency](https://steamcommunity.com/sharedfiles/filedetails/?id=3588231593)<br>[Scaling Decree Cost (Revived)](https://steamcommunity.com/sharedfiles/filedetails/?id=3642608053) | D15 | 할인 상한 설정 |
| F10 | 무역·관세·조약 | P1 | [Smart Economy Full](https://steamcommunity.com/sharedfiles/filedetails/?id=3786321132)<br>[Advanced Market Panel](https://steamcommunity.com/sharedfiles/filedetails/?id=3676318398)<br>[Trade Partners](https://steamcommunity.com/sharedfiles/filedetails/?id=3733651973)<br>[Smart Treaties](https://steamcommunity.com/sharedfiles/filedetails/?id=3814823352) | D05, D11 | 예측과 실제 분리 |
| F11 | 민간투자·기업 | P1 | [Smart Economy Full](https://steamcommunity.com/sharedfiles/filedetails/?id=3786321132)<br>[Controllable Private Construction](https://steamcommunity.com/sharedfiles/filedetails/?id=3278770055)<br>[Grey's Smarter Private Construction](https://steamcommunity.com/sharedfiles/filedetails/?id=3115017647)<br>[Kuromi's AI](https://steamcommunity.com/sharedfiles/filedetails/?id=3227982912) | D06 | 민간경제 핵심 제약 보존 |
| F12 | 속국·외교 관리 | P1 | [Auto-Diplomacy And Advanced Subject Management](https://steamcommunity.com/sharedfiles/filedetails/?id=3432933121)<br>[Choose Laws For Released Subjects](https://steamcommunity.com/sharedfiles/filedetails/?id=3467896823)<br>[Subject Production Method Control](https://steamcommunity.com/sharedfiles/filedetails/?id=3772712271)<br>[Power Bloc Leader Action Alerts](https://steamcommunity.com/sharedfiles/filedetails/?id=3779275113) | 간접 수요 | 관계 적대·권한 제약 반영 |
| F13 | 육군·해군·전쟁 | P1 | [War Goal & Treaty UI](https://steamcommunity.com/sharedfiles/filedetails/?id=3722701706)<br>[No Commander Alert](https://steamcommunity.com/sharedfiles/filedetails/?id=3753749567)<br>[More Icons for Formations](https://steamcommunity.com/sharedfiles/filedetails/?id=3625787964)<br>[Auto-Recruit & One-Click Recruit Commanders](https://steamcommunity.com/sharedfiles/filedetails/?id=3279201462) | D08 | 부대 편제 변경 정상 제한 |
| F14 | 정치·법률·기술 | P1 | [GORA UI+](https://steamcommunity.com/sharedfiles/filedetails/?id=3724865565)<br>[Choose Laws For Released Subjects](https://steamcommunity.com/sharedfiles/filedetails/?id=3467896823)<br>[Detailed Character Notifications](https://steamcommunity.com/sharedfiles/filedetails/?id=3633799211)<br>[Parliament 2.0](https://steamcommunity.com/sharedfiles/filedetails/?id=3687767007) | D12 | 감춰진 데이터 공개의 공정성 검토 |
| F15 | POP·생활수준 진단 | P1 | [Visible Pop Needs](https://steamcommunity.com/sharedfiles/filedetails/?id=2898482972)<br>[Visible Pop Qualifications](https://steamcommunity.com/sharedfiles/filedetails/?id=3807209131)<br>[Visible Pop Needs 1.13.x](https://steamcommunity.com/sharedfiles/filedetails/?id=3751688101)<br>[Intuitive Acceptance Icons](https://steamcommunity.com/sharedfiles/filedetails/?id=3422044570) | D10 | 복잡한 인과관계 단정 금지 |
| F16 | 뉴스·통계·역사 기록 | P2 | [Global Stats (GDP and POP)](https://steamcommunity.com/sharedfiles/filedetails/?id=3312478537)<br>[Headlines](https://steamcommunity.com/sharedfiles/filedetails/?id=3142463417)<br>[National History](https://steamcommunity.com/sharedfiles/filedetails/?id=3772525185)<br>[Achievement Guide](https://steamcommunity.com/sharedfiles/filedetails/?id=3815841501) | D09 | 저장 규칙·세이브 호환 검증 |
| F17 | 소폭 경제·산업·전쟁 밸런스 | P2 | [Building relocation](https://steamcommunity.com/sharedfiles/filedetails/?id=3791071271)<br>[State Taxation Adjustment](https://steamcommunity.com/sharedfiles/filedetails/?id=3773963536)<br>[Automobiles Won't Destroy Your Railways Anymore](https://steamcommunity.com/sharedfiles/filedetails/?id=3282418847)<br>[Remove Unwanted Buildings](https://steamcommunity.com/sharedfiles/filedetails/?id=3284950938) | 간접 수요 | 산업별 수지균형 확인 |
| F18 | 성능·접근성·현지화 | P2 | [Enter Key](https://steamcommunity.com/sharedfiles/filedetails/?id=3776818197)<br>[Grey's Soft Pop Adjustments](https://steamcommunity.com/sharedfiles/filedetails/?id=3336914177)<br>[Adjustable UI Opacity](https://steamcommunity.com/sharedfiles/filedetails/?id=3774563295)<br>[Adjust Middle Mouse Wheel Scrolling Speed](https://steamcommunity.com/sharedfiles/filedetails/?id=3774236562) | D13, D16 | 모든 대표 패널에서 테스트 |

## 추적 규칙

1. 기능이 구현됐다면 해당 기능 ID 아래 구체 테스트 ID와 작동 영상을 연결한다.
2. 기존 모드에서 오류나 호환 문제를 발견하면 개별 ID로 이슈를 기록하되 원본 코드를 무단 복사하는 것은 피한다.
3. Workshop ID가 바뀐 후속 모드가 나오면 새 행을 만들고 구형 모드는 기록으로 남긴다.
4. 게임 업데이트로 본편에 반영된 기능은 개발 우선순위에서 내려야 한다.
5. 설계 가설·이전 조사 링크·재열람 증거의 신뢰도를 동일하게 취급하지 않는다.

## 실제 구현 시 추가할 필수 열

`implementation_status`, `engine_feasibility`, `source_files`, `overridden_vanilla_files`, `test_cases`, `target_game_version`, `owner`, `blockers`, `compat_patches`, `last_verified_at`.
