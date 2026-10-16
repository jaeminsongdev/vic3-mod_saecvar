# SAECVAR

여러 역사 시대의 대형 전략게임 토탈 컨버전 모드를 제작하는 **시리즈 브랜드**입니다.

## 시리즈

| 시리즈 | 기반 게임 | 대상 시대 | 상태 |
|---|---|---|---|
| **[Twilight of Antiquity](Twilight_of_Antiquity/README.md)** | Victoria 3 | **서기 600~720년 (120년)** | 문서·디렉터리 기반 구축, 코드 미구현 |

추가 시리즈가 시작되면 **저장소의 1차 디렉터리를 시리즈 이름으로** 생성합니다. 공통 루트는 README 및 최소한의 Git 설정만 관리하며, 서로 다른 모드의 소스·도구·문서가 섞이지 않도록 합니다.

## 저장소 구조

```text
vic3-mod_saecvar/
├── README.md
├── .gitignore
└── Twilight_of_Antiquity/
    ├── README.md      # 600~720년 모드 개요
    ├── docs/          # 18개 기획·개발 문서(주제별) + 구조 가이드
    ├── mod/           # Victoria 3가 읽는 모드 경로만
    ├── data/          # 역사·인구·경제 구조화 원본
    ├── assets/        # 그래픽 소스·라이선스 자료 (mod/gfx와 구분)
    ├── toolchain/     # 자체 Rust Analyzer + 편집·빌드 코어 + CLI/GUI
    └── tests/         # 정적·통합·실제 엔진 검사
```

**바로 볼 문서:** [시리즈 문서 목차](Twilight_of_Antiquity/docs/README.md) · [M0~M10 공식 개발 순서](Twilight_of_Antiquity/docs/planning/MASTER_ROADMAP.md) · [최적화 디렉터리 설계](Twilight_of_Antiquity/docs/architecture/REPOSITORY_LAYOUT.md)

**현재 상태:** 실제 Rust workspace·게임 실행용 모드 메타데이터·게임 콘텐츠는 생성하지 않았습니다. 실행/호환성 검증이 완료된 것으로 간주하지 않습니다.
