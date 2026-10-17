# SAECVAR: Vanilla Plus — 독립 기능군 18개

[X01~X07 횡단 요구](CROSS_CUTTING_REQUIREMENTS.md) · [세부 기능 등록부 56개](../planning/REQUIREMENT_REGISTER.csv) · [기능별 외부 근거 인덱스](../planning/FEATURE_SOURCE_MATRIX.csv)

각 Fxx는 설계/구현 소유 단위다. 세부 `Fxx-Ryy`는 총 56개이며, 현재 모두 **PLANNED**다. 동일한 사용자 요구가 여러 기능에 걸칠 수 있으므로 [추적 매트릭스](../research/10_TRACEABILITY_MATRIX.md)로 중복을 관리한다.

| ID | P | 기능 설계 문서 | 세부 요구 | 의존 |
|---|---|---|---:|---|
| F01 | P0 | [건설·대기열 관리](F01_CONSTRUCTION.md) | 4 | F04/F06 |
| F02 | P0 | [생산방식(PM) 관리](F02_PRODUCTION_METHODS.md) | 5 | F03/F04/F05 |
| F03 | P0 | [기반시설 자동화](F03_INFRASTRUCTURE.md) | 3 | F01/F04 |
| F04 | P0 | [경제 통계·정보](F04_ECONOMY.md) | 4 | F06 |
| F05 | P0 | [경고·알림](F05_ALERTS.md) | 3 | F04/F06 |
| F06 | P0 | [UI·조작 편의](F06_UI.md) | 4 | F04 |
| F07 | P1 | [동화·개종·편입](F07_POP_INCORPORATION.md) | 3 | F15 |
| F08 | P1 | [악명·외교 균형](F08_INFAMY.md) | 2 | F12 |
| F09 | P1 | [칙령·행정](F09_DECREES.md) | 2 | F04/F06 |
| F10 | P1 | [무역·관세·조약](F10_TRADE.md) | 3 | F04/F06 |
| F11 | P1 | [민간투자·기업](F11_PRIVATE_INVESTMENT.md) | 3 | F01/F04 |
| F12 | P1 | [속국·외교 관리](F12_SUBJECT_DIPLOMACY.md) | 3 | F05/F08/F10 |
| F13 | P1 | [육군·해군·전쟁](F13_MILITARY.md) | 3 | F05/F06 |
| F14 | P1 | [정치·법률·기술](F14_POLITICS_TECH.md) | 3 | F05/F06 |
| F15 | P1 | [POP·생활수준 진단](F15_POP_WELLBEING.md) | 2 | F04/F07 |
| F16 | P2 | [뉴스·통계·역사 기록](F16_NEWS.md) | 3 | F05/F14 |
| F17 | P2 | [소폭 경제·산업·전쟁 밸런스](F17_BALANCE_LITE.md) | 3 | F07/F08/F09 |
| F18 | P2 | [성능·접근성·현지화](F18_PERFORMANCE_I18N.md) | 3 | F04/F05/F06 |

## 공통 설계 규칙

- 원본 바닐라 게임의 법률·기술·경제/군사 규칙과 정상 비용을 우회하지 않는다.
- 자동화는 **정보 표시 → 추천 → 승인/실행 → 제한된 조건부 자동화**로 확장한다.
- Balance Lite 수치는 정답이 아니라 원본과 AI 관찰전으로 검증할 제안값이다.
- [모듈 경계](../architecture/MODULE_BOUNDARIES.md) · [개발 로드맵](../planning/MASTER_ROADMAP.md) · [출처](../research/02_WORKSHOP_CATALOG.md)
