# Twilight of Antiquity — 문서 안내

기획 문서 18개를 **주제별 디렉터리**로 배치했다. 전체 프로젝트 시작점은 [시리즈 README](../README.md), 저장소 경계·미래 확장 원칙은 [디렉터리 아키텍처](architecture/REPOSITORY_LAYOUT.md), 가장 먼저 수행할 작업은 [최종 로드맵](planning/MASTER_ROADMAP.md)을 참고한다.

개발 착수 후 현재 상태는 [STATUS](planning/STATUS.md), 실제 구현·검증 결과는 [M0/G0 보고서](quality/M0_G0_REPORT.md), 실행 명령은 [runtime 안내](../tests/runtime/README.md)에서 관리한다. 아래 18개 원 기획 문서는 설계 기준으로 유지한다.

| 영역 | 목적 | 문서 |
|---|---|---|
| `product/` | 프로젝트 비전·기획 범위 | [01 프로젝트 헌장](product/PROJECT_CHARTER.md) · [02 게임 디자인 원칙](product/DESIGN_PILLARS.md) |
| `history/` | 서기 600~720 세계사·연대 | [03 600년 세계사](history/WORLD_600.md) · [04 600~720년 캠페인](history/CAMPAIGN_600_720.md) |
| `gameplay/` | 경제·사회·지도·전쟁 설계 | [05 고대 경제·기술](gameplay/ECONOMY_TECH.md) · [06 정치·사회·외교](gameplay/SOCIETY_DIPLOMACY.md) · [07 군사·지도·게임 데이터](gameplay/WAR_MAP_DATA.md) |
| `architecture/` | 툴체인·분석기·편집 프로토콜 | [11 자체 정적 분석기 아키텍처](architecture/ANALYZER_ARCHITECTURE.md) · [13 규칙·진단·CLI 계약](architecture/ANALYZER_RULES_REPORTS.md) · [16 공통 코어·편집 아키텍처](architecture/TOOLCHAIN_ARCHITECTURE.md) · [18 Codex·사람 공통 작업 절차](architecture/SHARED_WORKFLOWS.md) |
| `planning/` | 의사결정·개발 순서 | [08 로드맵 개요·품질 게이트](planning/ROADMAP_QA.md) · [09 결정 기록·미결 사항](planning/DECISIONS_OPEN.md) · [12 분석기 세부 로드맵](planning/ANALYZER_ROADMAP.md) · [17 최종 M0~M10 개발 순서](planning/MASTER_ROADMAP.md) |
| `reference/` | 기능 카탈로그·외부 레포 | [14 기능 96항목 카탈로그](reference/TOOLCHAIN_FUNCTION_CATALOG.md) · [15 참고 레포 18개·매핑](reference/REFERENCE_REPOSITORIES.md) |
| `quality/` | 검증·QA·근거 관리 | [10 검증 체크리스트](quality/VALIDATION.md) |

## 작업 시 읽는 순서

- **Codex 개발 착수**: [MASTER_ROADMAP](planning/MASTER_ROADMAP.md) → [REPOSITORY_LAYOUT](architecture/REPOSITORY_LAYOUT.md) → [TOOLCHAIN_ARCHITECTURE](architecture/TOOLCHAIN_ARCHITECTURE.md) → [REFERENCE_REPOSITORIES](reference/REFERENCE_REPOSITORIES.md)
- **정적 분석기**: [ANALYZER_ROADMAP](planning/ANALYZER_ROADMAP.md) → [ANALYZER_ARCHITECTURE](architecture/ANALYZER_ARCHITECTURE.md) → [ANALYZER_RULES_REPORTS](architecture/ANALYZER_RULES_REPORTS.md)
- **게임 콘텐츠·고증**: [WORLD_600](history/WORLD_600.md) → [ECONOMY_TECH](gameplay/ECONOMY_TECH.md) → [VALIDATION](quality/VALIDATION.md)
- **사람용 편집기**: [TOOLCHAIN_ARCHITECTURE](architecture/TOOLCHAIN_ARCHITECTURE.md) → [SHARED_WORKFLOWS](architecture/SHARED_WORKFLOWS.md) → [FUNCTION_CATALOG](reference/TOOLCHAIN_FUNCTION_CATALOG.md)

## 기존 문서의 이동

예전 `docs/01_*.md`에서 `docs/18_*.md`까지의 문서는 삭제된 것이 아니라 **같은 내용의 주제별 파일로 이동**했다. 현재 파일명이 달라진 점은 다음과 같다.

| 이전 파일 (docs/) | 새 파일 (docs/) |
|---|---|
| `01_PROJECT_CHARTER.md` | [`product/PROJECT_CHARTER.md`](product/PROJECT_CHARTER.md) |
| `02_DESIGN_PILLARS.md` | [`product/DESIGN_PILLARS.md`](product/DESIGN_PILLARS.md) |
| `03_WORLD_600.md` | [`history/WORLD_600.md`](history/WORLD_600.md) |
| `04_CAMPAIGN_600_720.md` | [`history/CAMPAIGN_600_720.md`](history/CAMPAIGN_600_720.md) |
| `05_ECONOMY_TECH.md` | [`gameplay/ECONOMY_TECH.md`](gameplay/ECONOMY_TECH.md) |
| `06_SOCIETY_DIPLOMACY.md` | [`gameplay/SOCIETY_DIPLOMACY.md`](gameplay/SOCIETY_DIPLOMACY.md) |
| `07_WAR_MAP_DATA.md` | [`gameplay/WAR_MAP_DATA.md`](gameplay/WAR_MAP_DATA.md) |
| `08_ROADMAP_QA.md` | [`planning/ROADMAP_QA.md`](planning/ROADMAP_QA.md) |
| `09_DECISIONS_OPEN.md` | [`planning/DECISIONS_OPEN.md`](planning/DECISIONS_OPEN.md) |
| `10_VALIDATION.md` | [`quality/VALIDATION.md`](quality/VALIDATION.md) |
| `11_ANALYZER_ARCHITECTURE.md` | [`architecture/ANALYZER_ARCHITECTURE.md`](architecture/ANALYZER_ARCHITECTURE.md) |
| `12_ANALYZER_ROADMAP.md` | [`planning/ANALYZER_ROADMAP.md`](planning/ANALYZER_ROADMAP.md) |
| `13_ANALYZER_RULES_REPORTS.md` | [`architecture/ANALYZER_RULES_REPORTS.md`](architecture/ANALYZER_RULES_REPORTS.md) |
| `14_TOOLCHAIN_FUNCTION_CATALOG.md` | [`reference/TOOLCHAIN_FUNCTION_CATALOG.md`](reference/TOOLCHAIN_FUNCTION_CATALOG.md) |
| `15_REFERENCE_REPOSITORIES.md` | [`reference/REFERENCE_REPOSITORIES.md`](reference/REFERENCE_REPOSITORIES.md) |
| `16_TOOLCHAIN_ARCHITECTURE.md` | [`architecture/TOOLCHAIN_ARCHITECTURE.md`](architecture/TOOLCHAIN_ARCHITECTURE.md) |
| `17_MASTER_ROADMAP.md` | [`planning/MASTER_ROADMAP.md`](planning/MASTER_ROADMAP.md) |
| `18_SHARED_WORKFLOWS.md` | [`architecture/SHARED_WORKFLOWS.md`](architecture/SHARED_WORKFLOWS.md) |

기존 문서의 상대 Markdown 링크는 신규 경로로 갱신했다. 새 작업은 옛 숫자형 파일 경로를 다시 생성하지 않는다.
