# 11. SAECVAR Analyzer — 자체 정적 분석기 아키텍처

- 상태: **Analyzer 상세 설계 v0.1 / Toolchain 통합 기획 v0.3** (실제 코드 미구현)
- 결정된 방향: **Tiger 참고 전용**, 자체 엔진·자체 파서·자체 모드 적용 해석기 구현
- 권장 언어: **Rust** (프로젝트 설계안; 별도 코드 언어 확정은 아직 없음)
- 시작 위치 제안: `Twilight_of_Antiquity/tools/saecvar-toolchain/` 안의 독립 Analyzer 구성요소([16 통합 구조](16_TOOLCHAIN_ARCHITECTURE.md) 참고)
- 관련: [12 개발 단계](12_ANALYZER_ROADMAP.md) · [13 진단 계약](13_ANALYZER_RULES_REPORTS.md)

## 1. 목적·비목적

목적은 Victoria 3 게임 파일과 SAECVAR 모드의 정적 입력 데이터를 **게임 프로세스 실행 없이** 검사하여, 스크립트 오류·오브젝트 참조 오류·모드 적용 충돌·시대·경제 논리의 구조적 오류를 발견하는 것이다. Codex CLI가 기계가 읽는 결과를 받아 **수정→검증→재검증**을 반복하게 한다.

명시적 비목적:
- 게임 엔진 전체 재구현, 실제 경제/AI/군사 시뮬레이터 개발.
- 게임 로딩 성공·세이브 가능·지도 렌더링을 정적 결과만으로 보증.
- Tiger, Jomini 또는 타 분석기의 라이브러리·실행 파일을 필수 의존성으로 연결.
- 원본 게임·DLC 파일을 공개 저장소나 GitHub 호스팅 CI에 복제.
- 자동 수정 기능의 무조건 적용 또는 분석기 미지원 구문에 대한 무조건 성공 판정.

## 2. 계층형 데이터 파이프라인

```text
[입력]
  ├─ Victoria 3 원본 게임 파일 (로컬, read-only)
  ├─ 사용자가 설치한 DLC 및 기타 활성 모드 (로컬, read-only)
  ├─ SAECVAR 모드 파일 + 역사 데이터 (수정 대상)
  └─ 버전별 스키마·검증 규칙·모드 우선순위 설정
           │
           v
[Source Discovery & Versioned Workspace]
           │
           v
[Lexer → Lossless CST / Recoverable AST]
           │
           v
[Virtual Mod Overlay & Provenance Resolver]
           │
           v
[Symbol Index / Reference Graph / Scope & Type]
           │
           v
[Generic Rules + Historical/Economy Rules]
           │
           v
[Diagnostics: JSON → SARIF/HTML / CLI]
           │
           v
[Codex 수정 및 재검사]  (런타임 엔진 검증은 별도)
```

파싱→적용 모델→의미 분석의 계층을 유지하되, 실제 구현에서는 파일별 인덱싱과 증분 캐시를 사용한다.

## 3. Lexer / Parser

**직접 구현해야 할 최소 구문:** 중괄호 중첩, `key = value` 구조, 비교 연산자, 주석, 문자열(이스케이프 포함), 값 목록, 같은 키의 반복 등장, 파일별 순서, 공백·원본 위치, 오류 복구. 엔진 구문은 버전과 파일 종류에 따라 다르므로 문법 목록을 샘플과 함께 확대한다.

- **Lexical tokens:** 식별자·구분자·연산자·값·문자열·주석과 각 토큰의 byte offset, line, column.
- **Lossless CST:** 중복 키, 순서, 주석, 원본 영역을 보존. 추후 자동 수정/정확한 진단에 필요.
- **Semantic AST:** 검사 규칙에 필요한 구조만 정규화. 미확인 값은 버리지 않고 불투명 노드로 유지.
- **Error recovery:** 하나의 미종결 블록·문자열 때문에 나머지 파일을 전부 분석하지 못하는 상황을 완화.
- **Dialects:** 일반 스크립트, UI, localization 등은 처음부터 단일 문법으로 단정하지 않고 어댑터·지원 수준 분리.
- **Round-trip invariant:** 지원하는 파일을 파싱한 뒤 원본 범위/순서를 손실 없이 추적할 수 있어야 한다.

기원전/서기 연도는 데이터 레벨과 엔진 시간 처리 레벨을 별도로 취급한다. 텍스트로 `600.1.1`을 읽는 것과 런타임 달력의 정상 동작은 별개다.

## 4. Mod Resolution Engine (핵심 리스크)

**파일 경로 기준 오버레이**와 **게임 객체 정의 기준 적용**을 별도의 처리 단계로 구현한다.

1. 게임·DLC·활성 모드의 입력 루트와 실제 적용 순서를 기록.
2. 파일 종류·폴더 및 상위 경로별 로드·교체 규칙과 예외를 버전별 테이블로 선택.
3. 동일 경로 파일의 대체, 동일 객체명 중복, `INJECT:`/`REPLACE:`/조건부 variants 지원 여부를 구분.
4. 로컬라이제이션·GUI·이벤트·defines 등 특수 로드 규칙은 도메인 전용 어댑터로 분리.
5. 최종 객체마다 `winning_definition`, `sources`, `shadowed_sources`, `rule`, `rule_version` 등 **출처 추적(provenance)**을 제공.
6. 미지원 예외는 무리하게 확정하지 않고 `MODEL_PREDICTED` 또는 `UNSUPPORTED_RULE`로 표시.

**주의:** 위 연산이 모든 폴더/현재 게임 버전에서 동일하게 작동한다는 의미가 아니다. 실제 지원 여부·우선순위는 해당 버전의 문서/원본/실제 엔진 비교를 통해 확정한다.

## 5. Symbol / Semantic Layer

- `SymbolTable`: 국가·주·문화·종교·건물·상품·기술·법률·이벤트 등 분류와 정의 지점.
- `ReferenceGraph`: 정의→참조 대상, 도입/삭제/이름변경에 따른 영향 경로.
- `SchemaRegistry`: 게임 버전별 키·필드·타입·중첩 구조.
- `ScopeChecker`: 효과·트리거가 해당 컨텍스트에서 허용되는지 검사(확실하지 않으면 유보).
- `LocalizationIndex`: 키 정의·언어 파일·언어 정책에 따른 누락/중복 검사.
- `AnalyzerRuleEngine`: 일반 규칙과 SAECVAR 고대사 특화 규칙을 분리.

모델의 입력 근거가 충분한 규칙에만 고신뢰 오류를 출력하며, 역사적 해석·게임 플레이 최적화처럼 본질적으로 추정인 것은 경고로 분류한다.

## 6. Diagnostics / Reporting

공통 진단 구조:
- `rule_id`, `severity`, `verification_status`, `confidence`
- `file`, `range`(줄·열), `message`, `why`, `suggested_fix`
- `related_locations`, `source_provenance`, `engine_version`, `schema_version`
- `fingerprint`(중복·회귀 매칭), `suppression_reason`(선택), `runtime_evidence`(선택)

우선 안정적인 JSON 계약과 **안정된 오류 ID**를 구현하고 SARIF/HTML, Codex 자동화, GitHub 검사와 연결한다. 자동 수정의 안전성·승인 정책은 향후 별도로 결정한다.

## 7. 증분 캐시와 결정성

파서 출력·객체 적용 모델·심볼·규칙 결과를 **입력 파일 콘텐츠 해시 + 게임 버전 + 로드 순서 + 스키마/규칙 버전**으로 캐싱한다. 변경 파일만 재파싱하되 의존성 그래프를 통해 영향받는 소비자 정의를 재검사한다.

**일관성 조건:** 같은 입력 및 규칙에 대한 `check --changed` 결과는 `check --full`의 관련 진단 집합과 일치해야 한다. 모드 로드 순서 변경 시 overlay 캐시를 반드시 무효화한다.

## 8. 코드 배치 제안 (미생성 디렉터리)

```text
Twilight_of_Antiquity/
└── tools/
    └── saecvar-analyzer/              # 계획된 위치, 아직 없음
        ├── Cargo.toml
        ├── crates/
        │   ├── pdx-syntax/            # 원본 보존 Lexer/CST
        │   ├── pdx-parser/            # AST·오류 복구
        │   ├── pdx-workspace/         # 파일 탐색·버전·로컬 게임 입력
        │   ├── pdx-resolver/          # 모드 오버레이·출처 추적
        │   ├── pdx-semantics/         # 심볼·타입·Scope·참조
        │   ├── rule-engine/           # 일반·시대별 검사 규칙
        │   ├── diagnostics/           # JSON, SARIF, HTML
        │   └── analyzer-cli/          # 커맨드와 종료 코드
        ├── schemas/victoria3/         # 버전별 규칙
        ├── rules/saecvar/             # 600년 시나리오 규칙
        └── tests/
            ├── fixtures/
            ├── golden/
            ├── fuzz/
            └── integration/
```

추후 다른 SAECVAR 시리즈에서 재사용할 경우, 분석기 코어를 공통 저장소/패키지로 추출할지를 **별도 의사결정**한다. 현재는 '루트 1차 폴더=시리즈명' 원칙을 유지한다.

## 9. 라이선스·외부 참고 정책

- [Tiger](https://github.com/amtep/tiger)는 비교 가능한 기능·테스트 관점·실패 사례를 **참고만** 한다. Tiger 프로젝트는 **GPL-3.0-or-later**로 배포되므로 해당 코드를 복사하거나 개작하려면 라이선스 의무를 별도로 검토해야 한다.
- [Jomini](https://github.com/rakaly/jomini)는 PDX 포맷의 함정과 참고 구현을 연구할 수 있으나 **현재 설계는 이를 실행/링크 의존성으로 사용하지 않는다.**
- 기능 요구·외부 문서에서 영감을 얻는 것과 저작권 보호되는 코드·테스트 자료를 그대로 옮기는 것은 다르다. 참고한 문헌·설계 선택의 근거를 기록하고 코드 자체를 독립 구현한다.
- 원본 유료 게임 데이터는 로컬 검사 입력으로만 읽고, 공개 저장소/CI 산출물에 배포하지 않는다.

## 10. 모델 신뢰 경계

`STATIC_VERIFIED` = 지원된 규칙 기준에서 정적 검사됨(엔진 성공 아님)  
`MODEL_PREDICTED` = 정적 로드·분석 모델이 예상한 적용 결과  
`RUNTIME_REQUIRED` = 실제 게임 엔진을 실행해야 판단 가능  
`UNSUPPORTED_RULE` = 아직 분석 범위 밖의 파일·구문·규칙

특히 날짜·세이브·AI·전쟁·GUI는 엔진 테스트가 필요하다. [13 출력 계약](13_ANALYZER_RULES_REPORTS.md)에 단일 보고서 형식을 정의한다.

## v0.3: 공통 툴체인과의 결합

본 문서의 SAECVAR Analyzer는 [16 공통 Toolchain](16_TOOLCHAIN_ARCHITECTURE.md)의 **읽기·오버레이·검사 엔진**이다. 자체 Rust CLI를 독립 실행할 수 있어야 하지만, 별도의 완전히 독립적인 parser/규칙 엔진을 사람용 GUI가 재구현하지 않는다. 이전의 `tools/saecvar-analyzer/`는 미생성된 위치 **제안**이며, 이후 `tools/saecvar-toolchain/` workspace 안의 analyzer 패키지로 배치하는 방향으로 갱신한다. 타사 실행 도구에 의존하지 않는다는 결정은 그대로 유지한다.
