# SAECVAR

**SAECVAR**는 서로 다른 역사 시대를 다루는 대형 전략게임 토탈 컨버전 모드의 **공통 브랜드**입니다.

이 저장소는 여러 개의 독립적인 모드 시리즈를 수용합니다. **저장소 루트의 1차 디렉터리는 시리즈명**으로 구성하고, 각 시리즈의 기획 문서와 향후 실제 모드 파일은 해당 디렉터리 아래에서 독립적으로 관리합니다.

## 시리즈 목록

| 시리즈 | 대상 게임 | 캠페인 시대 | 현재 단계 |
|---|---|---|---|
| **[Twilight of Antiquity](Twilight_of_Antiquity/README.md)** | Victoria 3 | 서기 **600~720년** (120년) | 기획 기준선 v0.1 |

다른 시대의 모드 시리즈는 기획이 확정될 때 별도 1차 디렉터리로 추가합니다. 현재 다른 시리즈의 제작·시작연도·출시는 확정되지 않았습니다.

## 현재 저장소 구조

```text
vic3-mod_saecvar/
├── README.md                         # SAECVAR 공통 브랜드 및 시리즈 목차
└── Twilight_of_Antiquity/            # 시리즈별 독립 영역
    ├── README.md                     # 해당 모드의 기획 개요
    └── docs/                         # 해당 모드의 상세 기획·검증 문서 10개
        ├── 01_PROJECT_CHARTER.md
        ├── 02_DESIGN_PILLARS.md
        ├── 03_WORLD_600.md
        ├── 04_CAMPAIGN_600_720.md
        ├── 05_ECONOMY_TECH.md
        ├── 06_SOCIETY_DIPLOMACY.md
        ├── 07_WAR_MAP_DATA.md
        ├── 08_ROADMAP_QA.md
        ├── 09_DECISIONS_OPEN.md
        └── 10_VALIDATION.md
```

## 디렉터리 운영 원칙

1. **1차 폴더 = 시리즈명**: 예를 들어 `Twilight_of_Antiquity/`와 같이 작품별로 최상위 폴더를 분리합니다.
2. **시리즈별 독립 문서**: 각 폴더 안의 `README.md`에서 해당 작품의 `docs/` 및 향후 구현 파일로 연결합니다.
3. **향후 코드·에셋도 해당 시리즈 아래**: 실제 모드 코드를 작성하면 같은 시리즈 디렉터리 안에 배치하되, 세부 구조는 사용하는 엔진·모드 로더를 확인한 뒤 설계합니다.
4. **공통 파일은 정말 공통인 경우에만 루트에 배치**: 작품별 설정·문서·파일을 최상위에 혼합하지 않습니다.
5. **빈 시리즈나 예정작 폴더는 만들지 않음**: 작업이 시작된 모드만 등록합니다.

첫 시리즈의 상세 결정 및 개발 검증 항목은 **[Twilight of Antiquity 문서](Twilight_of_Antiquity/README.md)**에서 확인할 수 있습니다.
