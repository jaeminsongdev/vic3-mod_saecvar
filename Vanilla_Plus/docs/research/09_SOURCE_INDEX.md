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
