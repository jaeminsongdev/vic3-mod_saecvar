# SAECVAR Vanilla Plus — 저장소 전반 문서 감사 보고서

> 검토일: **2026-10-17 (KST)** · 기준 리비전: `e496117109085a87a192dd155c7c20ad251efd2f` → `8d22f2ae16ccee4f259138fcefd3bd83c81ac53a` → `5d88523c5e4a4cf539fd9e971bb14fc560bcad5b`  
> 작업 범위: GitHub `main`의 SAECVAR 브랜드 문서/로드맵, Twilight의 상태 정확성, 신규 Vanilla Plus 요구/근거/QA. **Victoria 3 엔진 실행·정적 파서 실행·실제 모드 구현은 이번 검수 대상에 포함되지 않음.**

## 1. 발견 후 수정된 문제

| ID | 최초 문제 | 수정/결정 | 근거 문서 |
|---|---|---|---|
| AUD-01 | F18에서 프로젝트 전 기능의 `횡단 요구사항`을 F18 독점 요구로 잘못 내포 | 정책 X01~X07로 독립화. 모든 F01~F18에 공통 적용 | [X01~X07](../features/CROSS_CUTTING_REQUIREMENTS.md), [F18](../features/F18_PERFORMANCE_I18N.md) |
| AUD-02 | 56개 Fxx-Ryy의 실현성/구현/실행검증 단위 추적 누락 | 56개 고유 ID, 계획 마일스톤 및 보류 상태 등록 | [요구 등록부](../planning/REQUIREMENT_REGISTER.csv) |
| AUD-03 | 수요·모드 출처와 기능군 간 지속 추적 불명확 | 111개 Workshop 사례 및 16개 수요를 F01~F18에 연결하는 **범주 단위** 참조표 | [출처 매핑](../planning/FEATURE_SOURCE_MATRIX.csv) |
| AUD-04 | 일부 기능을 대표 QA 10개 수준으로만 검토, 항목별 테스트 계획이 없음 | 56개 인수 시나리오 + 7개 횡단 정책 테스트 생성. 전부 `NOT_RUN` | [인수 테스트](ACCEPTANCE_TEST_CASES.csv), [공통 QA](CROSS_CUTTING_TEST_CASES.csv) |
| AUD-05 | 1.14 베타와 미래 정식 버전 호환성·패치 차이를 기능별로 관리할 표 부재 | 1.13/1.14/1.15 대상별 별도 시험 규칙 및 위험 매트릭스 추가 | [패치 검증표](VERSION_MATRIX.md) |
| AUD-06 | 연구 초기의 독립 `vic3-qol/` 경로·미반영 상태가 현재 저장소 구조와 혼동될 수 있음 | 초기 제안으로 명시하고 `Vanilla_Plus/`를 현재 기준으로 정정; 결합 열람본 갱신 | [연구 개요](../research/README.md), [역사적 구조안](../research/06_ARCHITECTURE_COMPATIBILITY.md), [합본](../research/ALL_IN_ONE.md) |
| AUD-07 | Twilight 일부 옛 문서가 'M0~M10 모두 미구현·코드 없음'으로 오해될 표현 유지 | M0 환경 구현/G0 최소 날짜 실증 수용, M1 초안 사용자 요청 중단, M2 이후 미착수 상태에 맞춰 정정 | [Twilight 결정](../../../Twilight_of_Antiquity/docs/planning/DECISIONS_OPEN.md), [검증 계획](../../../Twilight_of_Antiquity/docs/quality/VALIDATION.md), [STATUS](../../../Twilight_of_Antiquity/docs/planning/STATUS.md) |

## 2. 구조·수량 검증

| 검사 | 발견/검증 결과 |
|---|---|
| 변경 전 파일·문서 | 97개 파일 / Markdown 70개 |
| 첫 두 수정 커밋 이후 | 103개 파일 / Markdown 72개 |
| Workshop 목록 | **111건**, 고유 Workshop ID 111개 |
| 커뮤니티 수요 | **16건**, 고유 D ID 16개 |
| 기능군 | **18개 F01~F18**, 누락 없음 |
| 세부 수용 기준 | **56개**, 각 Fxx-Ryy 중복/누락 없음 |
| 공통 정책 | **7개 X01~X07** |
| 기능별 테스트 예정 | **56개**, 각 요구와 1:1 대응 |
| 공통 정책 테스트 예정 | **7개**, 실제 테스트는 아직 0건 |
| 소스 연결 관계 | 111개 Steam ID와 16개 수요의 기능군 매핑이 원본 CSV와 교차표 간 일치 |
| 변경 범위 | Vanilla Plus 문서 + Twilight의 상태 관련 Markdown 2개. **기존 Rust 코드/게임 모드 파일/테스트 코드 0개 변경** |

참고: 다수 Workshop 모드가 하나 이상의 기능군에 속하므로 범주별 연결 건수의 합계는 고유 ID 111보다 크다. 새 등록부의 `QA-Fxx-Ryy`는 **시험 계획 번호**이고 실제 테스트 성공이 아니다.

## 3. 연속 무수정 검토 기록

### 검토 A (수정 반영 후 첫 회차)

- A1: 요구/테스트 56대56, 정책 7개, 기능군 18개 ID 정합, 잘못된 F18 횡단 분류 제거 확인 → 문제 0개.
- A2: Twilight 상태 정정, 조사 문서 경로 정정, `ALL_IN_ONE`과 번호 06 원문 동기화, 새 버전 문서 연결 → 문제 0개.
- A3: 신규/수정 핵심 7개 문서 상대링크 **49개**, 깨짐 0.
- A4: 기타 수정 8개 문서 상대링크 **62개**, 깨짐 0.

### 검토 B (코드·문서 추가 수정 없이 별도 알고리즘으로 재검증)

- B1: 원본 `catalog.csv / requirements.csv / user_demands.csv`와 새 `FEATURE_SOURCE_MATRIX.csv / REQUIREMENT_REGISTER.csv / ACCEPTANCE_TEST_CASES.csv` 간 원문·건수·ID 역방향 매칭 → 누락/불일치 0.
- B2: 루트·브랜드 로드맵·두 시리즈 STATUS·QA·버전 문서 주요 상대링크 **51개**, 깨짐 0; M1 중단 상태와 독립 프로젝트 경계 유지.
- **A와 B는 수정이 필요하지 않은 2회 연속 검토**. 이후 추가한 본 감사 문서는 감사 결과 기록이며 게임 기능이나 밸런스 내용을 수정하지 않음.

## 4. 최신 공식 변경 관련 확인

- 2026-10-07 Paradox [Dev Diary #191](https://steamcommunity.com/app/529340/announcements/): 1.14 공개 베타의 검증된 변경들을 1.15에 통합 중이라고 설명.
- 2026-10-08 Paradox Dev Diary #192: State & Revolution의 출시 예정일을 **2026-11-05**로 연기. 1.15 실제 안정 빌드가 확정되기 전에는 현재 모드 호환이라고 표기하지 않음.
- 게임 업데이트 일정은 계획이며 추후 변경될 수 있다.

## 5. 범위 밖·남은 위험

1. 스팀 창작마당 등록물 111개의 현재 게시 여부·지원 버전·소스 라이선스를 모두 직접 설치 검증한 것은 아니다.
2. Fxx 기능들의 Victoria 3 스크립트/GUI 엔진 지원을 실제 설치 파일로 테스트하지 않았다. `PENDING/NOT_RUN` 보존.
3. 1.15 공개 후 GUI/정치·해군·악명·무역 등 변경사항과 기존 제안이 충돌하는지 재검토해야 한다.
4. 정적 검사 및 문서의 두 차례 무수정 검토는 **게임 엔진 실행 테스트와 다르다**.
5. Twilight G0 실험은 Twilight만의 제한된 날짜 검증이며 Vanilla Plus의 런타임 검증으로 이전하지 않는다.

## 6. 개발 재개 시 진입점

[Vanilla Plus Q0~Q8](../planning/MASTER_ROADMAP.md) → [공통 규약](../features/CROSS_CUTTING_REQUIREMENTS.md) → [56개 등록부](../planning/REQUIREMENT_REGISTER.csv) → [게임 패치 검증표](VERSION_MATRIX.md) → [QA 계획](QA_PLAN.md).

**결론:** 요청 범위의 문서·ID·로드맵 정합성 보완과 2회 연속 무수정 검토 완료. 구현/인게임 검증은 후속 단계로 남는다.
