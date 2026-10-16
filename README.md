# SAECVAR

**SAECVAR**는 서로 다른 역사 시대를 다루는 대형 전략게임 토탈 컨버전 모드의 **공통 브랜드**입니다.

이 저장소는 여러 개의 독립적인 모드 시리즈를 수용합니다. **저장소 루트의 1차 디렉터리는 시리즈명**으로 구성하고, 각 시리즈의 기획 문서와 향후 실제 모드 파일은 해당 디렉터리 아래에서 독립적으로 관리합니다.

## 시리즈 목록

| 시리즈 | 대상 게임 | 캠페인 시대 | 현재 단계 |
|---|---|---|---|
| **[Twilight of Antiquity](Twilight_of_Antiquity/README.md)** | Victoria 3 | 서기 **600~720년** (120년) | 통합 툴체인 기획 v0.3 |

다른 시대의 모드 시리즈는 기획이 확정될 때 별도 1차 디렉터리로 추가합니다. 현재 다른 시리즈의 제작·시작연도·출시는 확정되지 않았습니다.

## 현재 저장소 구조

```text
vic3-mod_saecvar/
├── README.md                         # SAECVAR 공통 브랜드 및 시리즈 목차
└── Twilight_of_Antiquity/            # 시리즈별 독립 영역
    ├── README.md                     # 해당 모드의 기획 개요
    └── docs/                         # 해당 모드의 상세 기획·검증 문서 18개
        ├── 01_PROJECT_CHARTER.md
        ├── 02_DESIGN_PILLARS.md
        ├── 03_WORLD_600.md
        ├── 04_CAMPAIGN_600_720.md
        ├── 05_ECONOMY_TECH.md
        ├── 06_SOCIETY_DIPLOMACY.md
        ├── 07_WAR_MAP_DATA.md
        ├── 08_ROADMAP_QA.md
        ├── 09_DECISIONS_OPEN.md
        ├── 10_VALIDATION.md
        ├── 11_ANALYZER_ARCHITECTURE.md
        ├── 12_ANALYZER_ROADMAP.md
        ├── 13_ANALYZER_RULES_REPORTS.md
        ├── 14_TOOLCHAIN_FUNCTION_CATALOG.md
        ├── 15_REFERENCE_REPOSITORIES.md
        ├── 16_TOOLCHAIN_ARCHITECTURE.md
        ├── 17_MASTER_ROADMAP.md
        └── 18_SHARED_WORKFLOWS.md
```

## 디렉터리 운영 원칙

1. **1차 폴더 = 시리즈명**: 예를 들어 `Twilight_of_Antiquity/`와 같이 작품별로 최상위 폴더를 분리합니다.
2. **시리즈별 독립 문서**: 각 폴더 안의 `README.md`에서 해당 작품의 `docs/` 및 향후 구현 파일로 연결합니다.
3. **향후 코드·에셋도 해당 시리즈 아래**: 실제 모드 코드를 작성하면 같은 시리즈 디렉터리 안에 배치하되, 세부 구조는 사용하는 엔진·모드 로더를 확인한 뒤 설계합니다.
4. **공통 파일은 정말 공통인 경우에만 루트에 배치**: 작품별 설정·문서·파일을 최상위에 혼합하지 않습니다.
5. **빈 시리즈나 예정작 폴더는 만들지 않음**: 작업이 시작된 모드만 등록합니다.

첫 시리즈의 상세 결정 및 개발 검증 항목은 **[Twilight of Antiquity 문서](Twilight_of_Antiquity/README.md)**에서 확인할 수 있습니다.

## 첫 시리즈 개발 우선순위

현재 진행 방향은 **자체 정적 분석기의 최소 파서·적용 해석·진단을 먼저 구현하고 공통 안전 편집 API를 구축한 뒤, 작은 고대 경제 모드를 만들며 Toolchain을 확장**하는 것이다. Tiger는 기능 연구 참고자료일 뿐, 실행·라이브러리·검증 백엔드에 의존하지 않는다.

- [SAECVAR Analyzer 아키텍처](Twilight_of_Antiquity/docs/11_ANALYZER_ARCHITECTURE.md)
- [통합 개발 순서 및 마일스톤](Twilight_of_Antiquity/docs/08_ROADMAP_QA.md)
- [분석기 세부 구현 로드맵](Twilight_of_Antiquity/docs/12_ANALYZER_ROADMAP.md)
- [규칙·진단 형식 및 CLI 계약](Twilight_of_Antiquity/docs/13_ANALYZER_RULES_REPORTS.md)
- **[전체 개념 기능 96항목](Twilight_of_Antiquity/docs/14_TOOLCHAIN_FUNCTION_CATALOG.md)**
- **[기능별 참고 GitHub 저장소 18개](Twilight_of_Antiquity/docs/15_REFERENCE_REPOSITORIES.md)**
- **[공통 코어 + Codex/Human 설계](Twilight_of_Antiquity/docs/16_TOOLCHAIN_ARCHITECTURE.md)**
- **[최종 M0~M10 개발 순서](Twilight_of_Antiquity/docs/17_MASTER_ROADMAP.md)**
- [Codex와 사람의 공통 변경·검증 워크플로](Twilight_of_Antiquity/docs/18_SHARED_WORKFLOWS.md)

분석기를 포함한 공통 코어는 **향후** `Twilight_of_Antiquity/tools/saecvar-toolchain/` 아래에 구성하는 것을 권장한다. 아직 구현되지 않았으며, 다른 시리즈로의 범용화는 실제 재사용 요구가 생긴 뒤 결정한다.

> 현재 상태는 **설계 문서화**다. Rust 툴체인이나 GUI가 구현되었다는 뜻은 아니다. SAECVAR 브랜드의 '루트 1차 폴더 = 시리즈명' 원칙은 그대로 유지한다.
