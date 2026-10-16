# 저장소 디렉터리 아키텍처 — SAECVAR v0.4

- 상태: **현재 GitHub에 생성하는 기본 폴더 + 미래 구현을 위한 하위 구조 계획**
- 기준: 최상위는 작품별 시리즈 폴더. 해당 시리즈의 소스·도구·문서·게임 데이터는 서로 구분.
- 최초 게임: Victoria 3, 서기 600~720년 토탈 컨버전.
- 연관: [전체 문서](../README.md), [공통 아키텍처](TOOLCHAIN_ARCHITECTURE.md), [최종 개발 순서](../planning/MASTER_ROADMAP.md).

## 1. 현재 생성할 실제 디렉터리

```text
SAECVAR repository/
├── README.md                         # 브랜드/시리즈 목차
├── .gitignore                        # 빌드·캐시·로컬 원본 게임 데이터 제외
└── Twilight_of_Antiquity/            # 첫 시리즈; 새 시리즈도 이 레벨에 생성
    ├── README.md
    ├── docs/
    │   ├── README.md                 # 문서 탐색·기존 경로 매핑
    │   ├── product/                  # 정체성, 범위, 게임 디자인 원칙
    │   ├── history/                  # 600년 세계사, 600~720 캠페인
    │   ├── gameplay/                 # 경제·정치·군사·지도 시스템
    │   ├── architecture/             # 공통 코어, 분석기, 편집 규칙, 본 문서
    │   ├── planning/                 # 공식 로드맵, 결정, Analyzer 세부 단계
    │   ├── reference/                # 기능 카탈로그와 참고 레포 18개
    │   └── quality/                  # 검증·QA·사료 신뢰도
    ├── mod/                           # 실제 Vic3 모드 경로; 개발 README 제외하고 배포
    ├── data/                          # 버전 관리되는 원본 역사/정형 데이터
    ├── assets/                        # 원본 그래픽 자산/라이선스; 게임용 gfx와 분리
    ├── toolchain/                     # 미래 Rust workspace + CLI/API + Human UI
    └── tests/                         # 독립 fixtures/통합/실제 엔진 실증
```

현재는 **기획·구조 골격**이므로 아래의 미래 소스 하위 폴더와 Cargo manifest를 아직 만들지 않는다. Git은 빈 폴더를 추적하지 않으므로 다섯 작업 디렉터리는 목적을 설명하는 README만 우선 둔다.

## 2. 런타임과 개발 도구의 엄격한 경계

### `mod/` — Victoria 3 모드 데이터 루트

앞으로 실제 게임 플레이가 가능해지면 다음 파일들이 생긴다.

```text
mod/
├── .metadata/metadata.json           # 런처 메타데이터; 실제 게임 버전 검증 후 생성
├── common/                           # 국가·경제·법률·기술·역사 등 게임 원본 경로를 미러링
├── events/
├── localization/
├── map_data/                         # 변경이 있을 때만
├── gfx/                              # 변환된 게임용 그래픽이 있을 때만
└── gui/                              # 게임 UI가 필요할 때만
```

**원칙:** 게임이 읽는 콘텐츠만 `mod/`에 포함한다. `toolchain`, `data`, `tests`, `assets`는 게임 런처/Steam 배포 패키지에 복사하지 않는다. `mod/README.md`도 개발 편의를 위해서만 보존하고 릴리스 패키지에는 제외한다. 게임 경로 미러링 원칙은 [Paradox 공식 모딩 개발일지](https://www.paradoxinteractive.com/games/victoria-3/news/victoria-3-dev-diary-60-modding)를 참고한다.

### `data/` — 원본의 소유권

미래의 데이터 유형(기획, 아직 구체 파일형식 미확정):
- `historical/`: 근거가 있는 600년 국가·지역·인구·문화·종교 데이터
- `sources/`: 서지, 사료 신뢰도, 연구용 주석
- `schemas/`: 자체 데이터 계약, 시대·ID 제약
- `balance/`: 개발자가 조정하는 숫자·밸런스 실험 입력

**한 데이터 정의의 편집 원본은 하나여야 한다.** 구조화 데이터로 생성한 PDX는 Generated 영역으로 마킹하고 재생성 때 임의 수동 변경을 덮어쓰지 않도록 한다. 수동 관리 PDX는 그대로 `mod/`에 두고 별도 DB에서 동시 쓰지 않는다.

### `assets/` — 그래픽 제작 원본

편집 가능한 SVG·디자인 소스, 원본 이미지, 개별 저작권/라이선스 근거를 별도 보관한다. 게임 렌더용 DDS/아이콘 등은 필요한 경우 빌드 도구가 `mod/gfx/` 등에 출력한다. 제3자 자산은 사용 허가를 확인하지 않았다면 공개 저장소에 추가하지 않는다.

### `toolchain/` — 구현할 공통 Rust workspace

M0~M4에서 다음의 **논리 패키지**를 필요에 따라 점진적으로 생성한다.

```text
toolchain/
├── Cargo.toml                        # 실제 첫 crate 생성 시 Rust 가상 workspace 구성
├── Cargo.lock                        # 버전 통일
├── crates/                           # flat crates/*, 필요 시 분할
│   ├── workspace/
│   ├── pdx-syntax/
│   ├── mod-overlay/
│   ├── semantics/
│   ├── diagnostics/
│   ├── edit-engine/
│   ├── data-engine/
│   └── build-engine/
├── apps/
│   ├── cli/                          # Codex/사용자 CLI
│   └── human-workbench/              # GUI (M7 이후)
├── configs/                          # 게임 버전별 스키마와 규칙 (예정)
└── fixtures/                         # 검증 사례의 공통 입력 (예정)
```

초기에 독립 프로세스/서비스로 모든 역할을 쪼개지 않는다. 라이브러리와 CLI를 하나의 Rust workspace로 시작하고 `apps/human-workbench/`는 코드가 필요할 때 생성한다. Cargo는 workspace 단위 `Cargo.lock`과 공통 target을 사용한다 ([Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)). 자세한 구조는 [Analyzer](ANALYZER_ARCHITECTURE.md)와 [Toolchain](TOOLCHAIN_ARCHITECTURE.md)을 참조한다.

### `tests/` — 게임 실행 여부별 분리

- `fixtures/`: 자유롭게 배포할 수 있는 합성 PDX/역사 데이터 정상·오류 입력
- `unit/`: 파서·참조·편집 범위·트랜잭션 회귀
- `integration/`: 파일 세트·정형 데이터→PDX 생성·기본 경제 정합성
- `runtime/`: 실제 Victoria 3에서 시작/끝/세이브·경제·AI·전쟁 확인
- `benchmarks/`: 정적 검사 속도와 실제 게임 진행 시간/성능 결과

외부 레포 및 원본 게임/DLC 파일을 CI 공개 아티팩트에 복제하지 않는다. 공개 Actions에는 합성 fixture만 사용한다.

## 3. 생성 산출물 및 Git 무시

**버전 관리:** 문서, 원본 데이터, 수동 관리 PDX, 편집 원본 에셋, 실제로 작성한 Rust 소스/테스트/로컬라이제이션.

**생성·비공개:** `target/`, `build/`, `dist/`, `.cache/`, `.local/`, runtime 테스트 산출물, 로그·실제 사용자 프로필/세이브. `.gitignore`로 제외한다. 실측 버전·결과·재현 경로의 요약은 사람이 쓴 문서로 추적할 수 있다.

**원본/모드 경로:** Windows/WSL 로컬 게임 설치 경로·Steam 라이브러리 위치는 사용자별 설정으로 읽고 Git에 커밋하지 않는다.

## 4. 다른 시리즈 추가 규칙

두 번째 SAECVAR 시리즈는 새 `<Series_Name>/` 형식으로 시작하고, 그 시리즈의 `mod/`, `data/`, `docs/`를 독립 배치한다.

공유 기능이 실제로 발생하면 별도 공통 툴체인 저장소/패키지로 추출하거나 의존성을 관리하는 방식을 **나중에** 선택한다. 현재 '루트 1차 폴더는 시리즈명' 원칙을 위반하면서 공통 도구 폴더를 미리 만들지 않는다.

## 5. 이동 규칙·검증 게이트

1. 원본 18개 문서를 주제별로 **내용 보존 이주**, 내부 상대 링크 재작성, [문서 목차](../README.md)에 구 경로 매핑.
2. 기존 `README.md`는 시리즈·로드맵·자료 검색 시작점으로 갱신.
3. 원본 게임/DLC는 읽기 전용이며 개발 도구가 임의 수정 금지.
4. 자동생성 파일과 수동 관리 파일은 소유권·Diff·rollback 경계를 설정.
5. 기술 검증(600년 날짜·모드 로딩)과 정적 검증은 서로 별도 게이트.
6. 모든 문서 링크·구조·모드 로더 경로를 **두 번 검토**하고 main 브랜치에 커밋.

**주의:** 저장소 폴더가 생성되었다고 Rust workspace와 실제 로드 가능한 Victoria 3 모드가 구현된 것은 아니다.
