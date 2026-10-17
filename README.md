# SAECVAR

Victoria 3를 위한 독립 모드 프로젝트 브랜드. 기존 토탈 컨버전과 Vanilla+ 편의성/Balance Lite 프로젝트를 **하나의 저장소에서 독립 관리**합니다.

| 시리즈 | 범위 | 실제 상태 | 로드맵 |
|---|---|---|---|
| [Twilight of Antiquity](Twilight_of_Antiquity/README.md) | 600~720년 고대 토탈 컨버전 | M0 구현, G0 최소 날짜 실증, M1 초안 추가 개발 중단 | [M0~M10](Twilight_of_Antiquity/docs/planning/MASTER_ROADMAP.md) |
| [Vanilla Plus (가칭)](Vanilla_Plus/README.md) | 편의성·반복 자동화·소폭 밸런스 | 조사 문서와 기능 설계만 반영 | [Q0~Q8](Vanilla_Plus/docs/planning/MASTER_ROADMAP.md) |

**[SAECVAR 통합 포트폴리오 로드맵](docs/PORTFOLIO_ROADMAP.md)** — 두 시리즈의 관계·공통 도구의 추출 조건·우선순위.

```text
vic3-mod_saecvar/
├── README.md
├── docs/PORTFOLIO_ROADMAP.md
├── Twilight_of_Antiquity/         # 기존 고대 토탈 컨버전: mod/data/assets/toolchain/tests/docs
└── Vanilla_Plus/                  # 신규 Vanilla+ 편의성: docs/features(F01~F18), docs/research, mod/tests(예정)
```

**원칙:** 1차 디렉터리는 시리즈 단위. 원본 게임/DLC·타사 Steam 창작마당 코드는 무단 복제·배포하지 않습니다. 공통 도구는 검증된 재사용 수요가 발생할 때만 추출하고 Twilight의 중단된 M1을 QoL 작업 선행 조건으로 삼지 않습니다. 정적 검사는 실제 게임 검증을 대체하지 않습니다.
