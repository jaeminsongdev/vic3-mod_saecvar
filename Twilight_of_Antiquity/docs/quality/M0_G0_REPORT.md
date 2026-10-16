# M0/G0 개발 및 엔진 실험 기록

- 날짜: 2026-10-16 (KST)
- 판정: **M0 환경 기반 IMPLEMENTED / G0 최소 날짜 가능성 ACCEPTED_WITH_LIMITATIONS**
- 목적: 600~720 달력의 최소 엔진 가능성 확인. 고대 국가·경제나 자체 Parser 구현은 이번 범위 밖이다.
- 공개 기록에는 설치 원본 전문·실제 세이브·사용자 경로·원시 로그를 포함하지 않는다.

## 1. 고정한 환경과 소유권

Victoria 3 `1.13.11 (Matcha)`, Steam app `529340`, build `24799966`을 로컬 설치의 `launcher/launcher-settings.json`, `appmanifest_529340.acf`, 런타임 `system.log`로 교차 확인했다. 실행 로그의 exe revision은 `15aa89ae429b6bcff8d195f2a650f98ba95cd64c`이다. 이것은 현재 설치본 식별이며 최신 배포 버전이라는 주장은 아니다.

Rust `1.96.0`, Cargo `1.96.0`, Windows 11 환경이다. 게임 원본/DLC는 읽기 전용 입력, `mod/`는 작성 소스, `.local/g0/`는 공개하지 않는 실행 증거로 관리한다. 버전·원본 파일 SHA-256은 [game-target.json](../../toolchain/configs/game-target.json)에 있다. 설치 DLC descriptor는 **17개**다. 설치·보유·실제 활성·필수 DLC 여부는 별개이며 요구사항 확정은 보류한다.

원본 게임·DLC를 저장소나 fixture에 복사하지 않았다. `compat` 실험을 생성할 때만 지정된 설치 파일의 파생본을 Git 제외된 실험 프로필에 만든다. 실제 사용자 세이브와 다른 모드는 보존한다.

## 2. 구현 항목과 자동 검증

| 항목 | 실제 구현 | 검증 범위 |
|---|---|---|
| Rust workspace | `crates/workspace`, `apps/cli`, Cargo.lock | 원본/모드 경로 분리와 식별 파일 검사 |
| 최소 모드 | `.metadata/metadata.json`, `common/defines/zz_saecvar_g0_dates.txt` | 600.1.1~720.1.1 정의, UTF-8 BOM |
| 환경 manifest | 게임 버전·빌드·원본 해시·소유권·fixture 정책 | 개인 경로는 로컬만 사용 |
| 실험 도구 | `tests/runtime/g0.py` | 별도 프로필, 입력 해시, 기존 실행 거부, 로그 스냅샷 |
| 합성 테스트 | Rust 경로 테스트 4개, Python 안전성 테스트 5개 | 중첩·경로 우회·원본 불변·입력 수 변경·기존 증거 보존·실제 로드 모드 해시 기록 |

실행한 Rust 경로 테스트 4개와 Python 안전성 테스트 5개는 통과했다. 실제 설치 경로와 작성 모드에 `inspect-paths`를 실행해 경로 분리를 확인했다. Cargo 검사 일부에서 incremental 디렉터리 마무리 권한 경고가 있었으나 종료 코드는 0이었고 테스트·Clippy는 통과했다. 캐시 경고가 없었다고 표현하지 않는다. 엔진 성공과 별개인 개발 도구 검사다.

## 3. 엔진 실험과 근거

| ID | 변경·실행 | 관찰 | 판정 |
|---|---|---|---|
| `date-01` | 날짜 모드, 격리 프로필 | content_load에 문자열 배열을 써서 `Missing path for dlc/mod`; 모드 로딩 불인정 | CONFIGURATION_FAILED |
| `date-02` | `enabledMods: [{path: ...}]`로 수정한 격리 실행 | debug.log의 Mod 목록에서 SAECVAR 확인. Steam backend 초기화 문제와 최초 BOM 경고 기록. 플레이 진입을 확인하지 못한 채 해당 인스턴스 종료 | PARTIAL; 날짜 동작 판정에 사용하지 않음 |
| `user-date-01` | 사용자가 런처에 등록해 일반 프로필로 실행. 모드는 날짜 변경 하나 | 런처 화면에서 이름 확인, 런처 생성 content_load와 엔진 Mod 목록 확인. 사용자 확인: 600년 표시·플레이 진입, 날짜 진행, 수동 저장·불러오기·재진행 모두 정상 | 해당 항목 USER_CONFIRMED |
| `compat` | 법률/이벤트 보정 실험 생성 기능 | 실험적 보정이며 엔진에서 실행하지 않음 | NOT_RUN |
| `boundary-01` | 격리 프로필의 719.12.30 시작 실험 실행 시도 | 초기화 화면에서 응답 없음 및 UI 전환 실패. 영어 기본 설정의 별도 프로필 실행을 중단하고 사용자 런처 실행으로 전환 | INITIALIZATION_INCOMPLETE; 날짜 불가능 판정에 사용하지 않음 |
| `user-boundary-01` | 한국어 설정의 기존 런처에서 `SAECVAR G0 - 720년 종료 검증`만 활성화 | 사용자 확인: 719.12.30 시작, 720년 진입 시 종료 화면 표시, 계속 플레이 가능. content_load·엔진 로그·실제 로드 날짜 파일로 조건 확인 | 해당 항목 USER_CONFIRMED |

`user-date-01`의 화면·플레이·저장/불러오기 판단은 **사용자 관찰**이다. Codex가 게임 화면을 직접 조작해 재현한 결과로 표현하지 않는다. Codex는 content_load와 엔진 로그를 읽고 `.local/g0/user-date-01/evidence/<UTC timestamp>/`에 스냅샷·해시 receipt를 보존했다. 격리 실행의 증거도 `date-01`, `date-02` 아래 보존했다. 원본 게임 파일 해시 불변 확인과 로그 수집은 사용자 게임플레이 관찰과 별개다.

`user-boundary-01`의 실행 로그는 23:07:49 시작 및 1.13.11 revision을 식별했고, content_load와 debug.log에서 종료 검증 모드만 로드된 것을 확인했다. 실제 모드의 날짜 정의는 `719.12.30`~`720.1.1`이었다. 종료 화면과 계속 플레이 성공은 사용자 관찰이며 Codex의 독립 화면 검증이 아니다. 로그 스냅샷, 실제 로드 파일 해시, 사용자 확인 기록은 `.local/g0/user-boundary-01/`에 보존했다. 기존 600년 소스 모드는 변경하지 않았다.

600년 세이브 `saecvar_g0_600_smoke.v3`의 크기는 **10,184,385바이트**, SHA-256은 `7174c63085d7bf460ae850639f98bd493c74f9c885ad0b8a83114ed096661f7e`였다. 원본을 유지하고 `.local/g0/user-date-01/artifacts/`에 동일 해시 사본을 보존했다. ZIP 내 meta는 바이너리여서 날짜를 텍스트로 해석하지 않았다. 저장·로드 성공은 사용자 관찰이 근거다.

G0의 최소 날짜 가능성은 제한 사항을 남긴 채 수용한다. **120년 전체 연속 진행, 전체 엔진 호환성, DLC 조합, 경제·AI·전쟁의 올바른 동작은 미검증**이다. 분리된 시작·저장/로드·종료 실험을 120년 장기 진행 성공으로 합산하지 않는다.

## 4. 시대 불일치와 실제 오류

### 로컬 원본에서 식별한 후보

- `common/laws/00_governance_principles.txt`: `game_date > 1836.1.1` **6곳** — 96, 115, 131, 1093, 1112, 1128행. 전달된 과거 사례의 3곳을 현재 버전에 그대로 적용하지 않는다.
- `events/french_pretenders.txt`: `year < 1883`, `year >= 1883` — 1534, 1580행. `compat`에서는 1836→600의 차이 1236년을 적용해 임계연도를 647로 옮긴다. 최종 고대 콘텐츠 설계가 아니라 상대 시점 보존 실험이다.
- `common/`, `events/` 텍스트에서 날짜·연도 관련 비교 후보 **2,513건 / 216파일**을 찾았다. 문자열 기반 제한 검색으로, 전체 날짜 문법·archive·DLC 특수 경로·의미 분석을 포괄하지 않는다. 후보 수는 오류 수가 아니다.

### 사용자 실행에서 실제 기록된 문제

2026-10-16 22:54:45 부근의 초기화 로그에 다음이 있었다.

| 문제 | 그 시점의 건수 | 근거·의미 |
|---|---:|---|
| 미래 발효일의 초기 조약 생성 거부 | 43 | `create_treaty`가 미래의 `entered_into_force_date`를 거부. 원본 `common/history/treaties/00_historical_treaties.txt:4` 등. 예: 원본 첫 조약의 발효일은 1815.10.26이며 600년 기준 미래다. |
| 통치자와 이해집단 지도자 조건 불일치 | 17 | `country.cpp:12779`의 초기화 오류. 날짜 변경 인과는 baseline 비교 전 UNRESOLVED. |
| GUI localization 진단 | 3 | `custom_tooltip.gui`, `map_markers.gui`, `military_formation_panel.gui`. baseline 비교 전 모드 회귀로 단정하지 않음. |

해당 스냅샷에서 error.log는 **192줄**이었다. 오류 하나가 여러 줄을 차지하므로 192개 오류라고 세지 않는다. 사용자 세션의 이 스냅샷에는 store backend 오류가 없었으나, 이것만으로 17개 DLC가 모두 실제 활성화됐다고 판단하지 않는다.

초기 조약 오류는 600년 날짜만 바꾸면 1836년 세계의 시간 관련 초기화가 그대로 유지되지 않는다는 실제 증거다. 법률 6곳과 이벤트 2곳 보정만으로 이 문제가 해결되지 않는다. 현재 단순 날짜 모드와 실험적 compat 패치는 모두 최종 고대 세계 구현이 아니다. 기존 게임 파일을 일괄 치환하거나 출처 없이 고대 역사 사실로 바꾸지 않는다.

## 5. 검토와 남은 경계

M0/G0 정리에서는 (1) 입력·실험 조건·원본 보존·합성 실패 사례와 (2) 사용자 관찰·로그 근거·미검증 항목·문서 연결을 별도로 자체 검토한다. 검증 명령과 커밋 대상의 공개 가능 여부를 확인한 뒤 main에 보존한다. 독립 외부 리뷰를 받은 것으로 표현하지 않는다.

추가 엔진 실증이 필요할 때 동일 DLC 조건의 baseline/날짜/compat 비교로 초기화 오류를 분류하고 120년 연속 안정성을 검사해야 한다. 미래 조약 문제는 아직 해결하지 않았다. 현재는 추가 게임 실행을 요청하지 않는다.

M1 초기 초안은 G0 기록 후 작성됐으나 사용자가 현재 진행할 필요가 없다고 지시하여 **추가 개발 중단** 상태로 보존한다. Lexer/CST/AST·오류 복구와 `parse` CLI, 합성 테스트 11개가 존재한다. 읽기 전용 원본 검사 4파일 중 3파일은 구문 진단이 없었고 defines는 inline math의 대괄호 26곳을 `UNSUPPORTED_RULE`로 보고했다. 이는 전수 검사·M1 완료·엔진 성공이 아니다. 결과는 `.local/m1/first-corpus/`에 보존했다.

실행 명령은 [runtime 안내](../../tests/runtime/README.md), 공식 순서는 [로드맵](../planning/MASTER_ROADMAP.md)을 따른다.

## 6. 조사 출처와 적용 한계

- [Paradox 공식 Dev Diary #60 — Modding](https://www.paradoxinteractive.com/games/victoria-3/news/victoria-3-dev-diary-60-modding), 2022-09-28. 모드의 `.metadata/metadata.json`, 게임 상대 경로 미러링, 런처 활성화 및 오류 로그 확인 절차의 공식 근거다. 현재 버전의 세부 동작은 설치 파일과 실행 로그로 재확인했다.
- [시작·종료일 변경 충돌 사례의 작성자 보고](https://www.reddit.com/r/vic3modding/comments/1rig7fc/editing_defines_startend_date_causes_crashes/). 작성자가 법률 3곳·프랑스 이벤트 2곳 수정으로 자신의 충돌을 해결했다고 기록했다. 이는 해당 사용자의 관찰이며 SAECVAR나 1.13.11의 해결 보장이 아니다. 현재 로컬 검사에서는 법률 6곳과 추가 초기 조약 오류가 확인됐다.

웹 출처의 수량·해결책을 현재 엔진 검증의 대체 근거로 쓰지 않았다. 자동 무GUI 엔진 검증 기능은 확인하지 않았다.
