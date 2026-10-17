# SAECVAR: Twilight of Antiquity

> Victoria 3 · 고대 말기/중세 초기 글로벌 역사 토탈 컨버전  
> **정규 캠페인: 서기 600~720년, 120년** · M0 환경 기반 구현 / G0 최소 날짜 실증 수용 / M1 초안 추가 개발 중단

600년의 실제 지구를 출발점으로 유럽·아시아·아프리카·아메리카·오세아니아 등 전 세계의 역사적 조건을 연구하고, 국가·인구·경제·제도·외교·군사 체제를 고대 시기에 맞게 대규모 개조합니다. **미래의 역사적 사건은 조건에 따라 분기하며 결과를 강제하지 않습니다.**

[SAECVAR 전 프로젝트 로드맵](../docs/PORTFOLIO_ROADMAP.md)에서 독립적인 Vanilla Plus 시리즈와 범위를 비교할 수 있습니다. 기존 Twilight 개발 상태와 M0~M10은 그대로 유지됩니다.

## 진입점

- **[현재 개발 상태](docs/planning/STATUS.md)** · **[M0/G0 결과와 재현 절차](docs/quality/M0_G0_REPORT.md)**
- **[전체 문서 목차](docs/README.md)** — 게임 기획, 개발 구조, 참고 프로젝트, 검증 규칙 18개 원문
- **[개발 로드맵](docs/planning/MASTER_ROADMAP.md)** — M0~M10 및 G0 날짜 호환성 조기 실증
- **[툴체인 공통 아키텍처](docs/architecture/TOOLCHAIN_ARCHITECTURE.md)** — Codex CLI/API와 사람용 GUI의 동일 편집·검사 엔진
- **[디렉터리 구조 규칙](docs/architecture/REPOSITORY_LAYOUT.md)** — 디렉터리 책임·파일 소유권·생성 산출물
- [참고 레포 매핑](docs/reference/REFERENCE_REPOSITORIES.md) · [기능 카탈로그](docs/reference/TOOLCHAIN_FUNCTION_CATALOG.md)
- [확정 및 미정 사항](docs/planning/DECISIONS_OPEN.md) · [검증 항목](docs/quality/VALIDATION.md)

## 실제 소스 저장 위치

| 위치 | 소유권 및 내용 |
|---|---|
| [`mod/`](mod/README.md) | 런처가 읽는 Victoria 3 호환 파일만. `common/`, `events/`, `localization/`, `map_data/`, `gfx/` 등은 실제 사용 시 생성 |
| [`data/`](data/README.md) | 600년 사료·국가·POP·게임 데이터의 버전 관리 가능한 원본 |
| [`assets/`](assets/README.md) | 독자적으로 제작·사용 허가된 편집용 이미지·문장·디자인 소스 |
| [`toolchain/`](toolchain/README.md) | Rust 공통 코어, 자체 PDX 분석기, 편집 API, Codex CLI, 향후 Human Workbench |
| [`tests/`](tests/README.md) | Fixture·회귀·데이터 정합성·엔진 실제 실행 테스트 |

## 핵심 결정

**자체 독립 분석기:** Tiger/Jomini 등은 설계·테스트 방식 참고용이고 핵심 파서/Resolver/Validator 실행 의존성은 두지 않습니다.

**공통 편집 엔진:** Codex와 사람은 동일한 `Inspect → EditPlan → Preview → Validate → Apply → Rollback` 프로토콜을 사용합니다.

**로드맵:** 게임 버전 확인·날짜 실증(G0) → 파서·오버레이·참조 검사 → 안전한 편집 API → **작은 고대 경제 수직 슬라이스** → 대규모 데이터·GUI·밸런싱.

**검증 경계:** 정적 분석 통과는 실제 게임 엔진의 경제·AI·전투·세이브·GUI 정상 동작을 보증하지 않습니다.

대상은 Victoria 3 1.13.11 / Steam build 24799966입니다. G0에서 600년 시작·날짜 진행·저장/로드, 별도 719년 말 시작 실험에서 720년 종료·계속 플레이를 사용자 관찰로 확인했습니다. 120년 연속 진행·전체 엔진/DLC/경제/AI/전쟁 검증은 아직 아닙니다. 자체 파서의 초기 코드는 존재하지만 M1은 완료하지 않았으며 사용자 요청으로 추가 개발을 중단했습니다.
