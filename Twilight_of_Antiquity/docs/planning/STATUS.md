# SAECVAR: Twilight of Antiquity — 현재 개발 상태

- 갱신: 2026-10-16 (KST)
- 작업: **M0 환경 기반 구현 / G0 최소 날짜 실증 수용 / M1 초안 추가 개발 중단**
- 대상: Victoria 3 **1.13.11 (Matcha)**, Steam build **24799966**
- 기준 소스: 시작 시 내려받은 `origin/main`의 `514c76d`. 사용자 요청으로 이번 변경을 main에 보존한다.

| 항목 | 현재 상태 | 근거 |
|---|---|---|
| M0 환경 기반 | IMPLEMENTED | Rust workspace, 경로 검사 CLI, 버전·해시 manifest, 합성 안전성 테스트, 실험 도구 |
| G0 런처 인식 | CONFIRMED | 런처 화면 관찰 + 런처 생성 content_load + 엔진 모드 목록 |
| G0 600년 표시·플레이 진입 | USER_CONFIRMED | 사용자가 실제 게임에서 확인 |
| G0 날짜 진행 | USER_CONFIRMED | 사용자 확인 |
| G0 저장·불러오기·재진행 | USER_CONFIRMED | 사용자 확인; 파일·로그 증거는 실험 보고서에 구분 기록 |
| G0 720년 종료·계속 플레이 | USER_CONFIRMED | 719.12.30 시작 → 720년 종료 화면 → 계속 플레이 정상이라는 사용자 관찰. 로그에서 경계 모드 단독 로드 확인 |
| 600~720 전체 연속 시뮬레이션 | NOT_RUN | 짧은 실행과 경계 실험으로 대체 완료 처리하지 않음 |
| DLC 요구사항·조합 검증 | UNRESOLVED | 설치 descriptor 17개 확인; 설치·활성·필수 여부는 별개 |
| M1 Parser | DRAFT / USER_STOPPED | Lexer/CST/AST·오류 복구·parse CLI의 초기 초안 존재. 사용자 요청으로 추가 개발 중단. M1 완료 아님 |
| M2 Overlay / M3 Analyzer 이후 | NOT_STARTED | 후속 개발 미착수 |

**G0 최소 날짜 가능성 실증: ACCEPTED_WITH_LIMITATIONS.** 600년 시작·진행·저장/로드와 720년 종료·계속 플레이를 사용자 관찰로 확인했다. 120년 연속 진행·전체 엔진 호환성·DLC 조합 및 경제/AI/전쟁 검증을 포함한 포괄적 Gate PASS가 아니다. 상세 근거는 [M0/G0 보고서](../quality/M0_G0_REPORT.md), 재현 명령은 [runtime 안내](../../tests/runtime/README.md)를 따른다.

## 현재 작업 경계

사용자 요청에 따라 이번 변경을 정리해 main에 보존한다. M1 초안은 완료로 승격하지 않고 함께 보존하며 추가 개발·게임 실행을 하지 않는다. 다음 개발 재개 시의 계획상 우선순위는 M1이지만 현재 수행 지시는 중단이다.

남은 위험은 초기 조약 생성 거부 등 시대 불일치, DLC 활성·요구사항, 120년 연속 안정성이다. 기존 600년 세이브와 테스트 증거는 로컬에 보존한다. 게임은 현재 꺼도 되며, 이후 실제 엔진 의존 기능을 검증할 때 사용자가 다시 실행한다.
