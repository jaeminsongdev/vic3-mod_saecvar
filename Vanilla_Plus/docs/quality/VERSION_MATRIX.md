# Vanilla Plus — 게임 버전 및 호환성 실측 계획

> 기준 2026-10-17 · 본 시리즈는 **아직 어떤 게임빌드에서도 실제 모드 로딩/기능 검증을 수행하지 않음**.

| 게임 채널 | 2026-10-17 기준 근거 | Vanilla Plus 실기 검증 | 필수 조치 |
|---|---|---|---|
| 1.13 정식 | 기존 연구의 원본 기준. Twilight의 1.13.11 설치 실험은 **다른 프로젝트 증거** | NOT_RUN | 설치된 정확한 빌드·DLC/GUI/API 조사 |
| 1.14 공개 베타 | [2026-10-07 공식 개발일지 #191 관련 공지](https://www.reddit.com/r/victoria3/comments/1wzxb1b/): 베타 주요 변경을 1.15에 통합 중. 악명·전쟁 지지도·전쟁 목표·해군·외교 AI 주의 | NOT_RUN | 베타 결과를 정식 패치 검증으로 전용하지 않음 |
| 1.15 정식 예정 | [2026-10-08 개발일지 #192](https://www.reddit.com/r/victoria3/comments/1x0rrdl/): State & Revolution **2026-11-05**로 연기. 1.15는 같은 시기에 공개 예정이나 일정 변경 가능 | NOT_RUN | 실제 출시·빌드 확인 뒤 GUI/defines/PM/관세/전쟁 diff 및 전 기능 회귀 |

## 고위험 재검증 항목

| 기능 ID | 위험 요인 | 릴리스 전 조치 |
|---|---|---|
| F08,F13,F17 | 악명·해군·전쟁 지지도 및 AI의 베타→정식 변경 | 예전 베타용 밸런스 패치 중복 여부 검증 |
| F02,F10,F11 | 생산방식·무역·AI 산업판단 | 원본/다른 모드와 계산·정책 중복 검증 |
| F04,F05,F06,F14 | GUI·이벤트·텍스트/현지화 구조 변경 | 수정되는 GUI 파일/키별 baseline diff |
| 그 외 전 기능 | 게임버전·DLC·세이브·멀티플레이 | 직접 사용한 경로마다 버전·증거 고정 |

## 증거 규칙

각 실험은 `game_version, game_build, dlc_profile, source_hash, mod_commit, feature_id, test_id, observed_result, evidence`를 함께 관리한다. 성공하지 않은 항목은 `NOT_RUN/PENDING`에서 상향하지 않는다. [QA 계획](QA_PLAN.md), [56개 요구 등록부](../planning/REQUIREMENT_REGISTER.csv).
