# 16. SAECVAR Toolchain — 공통 엔진과 Codex/Human 통합 아키텍처

- 상태: **v0.3 설계**, 아직 구현되지 않음
- 범위: 정적 분석기 단품에서 **편집·데이터·검증·빌드가 공유되는 툴체인**으로 확장
- 원칙: 기존 `Twilight_of_Antiquity/` 1차 폴더를 유지; 타 시리즈의 파일은 최상위 분리
- 관련: [14 기능](../reference/TOOLCHAIN_FUNCTION_CATALOG.md) · [17 순서](../planning/MASTER_ROADMAP.md) · [18 워크플로](SHARED_WORKFLOWS.md)

## 1. 단일 엔진, 두 클라이언트

```text
                +------------------------------+
                |  Codex CLI/API (Agent Bridge)|
                +---------------+--------------+
                                |
                       Typed Operations
                                |
  +-----------------------------v-----------------------------+
  | Shared Modding Core                                        |
  | Workspace | Parser | Overlay | Symbol/Scope | Validator    |
  | EditPlan | Transaction | Data Engine | Build | Knowledge  |
  | Versioned Schema | Provenance | Diagnostics | Diff         |
  +-----------------------------+-----------------------------+
                                |
                   same operation protocol
                                |
                +---------------v--------------+
                | Human Workbench (GUI)        |
                | Code / Map / Form / Graph    |
                | Preview / Approve / Errors   |
                +------------------------------+
```

도식의 위·아래는 **동등한 클라이언트**다. GUI가 Codex의 결과를 복사하거나 Codex가 사람의 UI를 스크린 조작할 필요가 없다. CLI와 GUI가 동일한 EditPlan, 검증과 트랜잭션 엔진을 사용한다.

## 2. 저장 계층: '하나의 진실'의 정확한 뜻

**모든 데이터를 하나의 JSON 파일로 모으지 않는다.** 데이터 유형별 권위 있는 원본을 **정확히 하나** 정한다.

| 데이터 | 권위 있는 원본 | 생성/가상 표현 |
|---|---|---|
| 시리즈 메타데이터·설정 | 프로젝트 manifest·버전·경로 설정 | 런처/배포 메타데이터 |
| 스크립트·이벤트·GUI | 수동 관리 PDX 파일 | 파서 IR 및 GUI 편집 뷰 |
| 일부 대량 정형 국가·POP·지역 자료 | data/ 정형 데이터·출처 DB | generated PDX 파일 |
| 상품·건물·기술 | 해당 데이터셋별 원본 정책(선택 가능) | 구조적 API가 만드는 변경 계획 |
| 역사 사료와 불확실성 | 별도 출처·추정 레지스트리 | 경고·근거 링크·타임라인 |
| 바닐라·DLC | 설치된 게임 파일(immutable/read-only) | 가상 오버레이 모델 |
| 최종 설치용 모드 | 결정적 빌드 산출물 | 게임 로더가 읽는 package |

**금지:** 생성 산출물을 사람이 조용히 수정하고 다음 빌드에서 덮어써 잃어버리는 것. generated 파일에 직접 수정 시 차단/임포트/원본 소유권 변경 중 명시적 절차를 요구한다.

## 3. 데이터 처리 순서

1. **Workspace:** 게임 빌드, DLC, 모드 playset, 실제 파일과 각 게임 폴더의 입력 형식 감지.
2. **Parser:** 텍스트 구문과 주석·순서·중복 키·위치를 보존.
3. **Overlay Resolver:** 원본+DLC+활성 모드의 적용 순서와 최종 정의를 provenance와 함께 해석.
4. **Typed Index:** 정의, 참조, 스키마, Scope, 로컬라이제이션, 의존성 그래프.
5. **Edit Engine:** CLI나 GUI 요청을 동일한 typed EditPlan으로 구성, 예상 변경·영향·충돌 제공.
6. **Validation:** 안전하게 검사할 수 있는 규칙만 확정하고 미지원/엔진 필요는 분리.
7. **Transaction/Build:** staging에서 검증하고 소스 revision 충돌 검사 후 atomic apply/rollback; PDX 생성, 빌드·리포트.
8. **Presentation:** CLI JSON/SARIF, 사람용 폼·지도·코드·그래프, 추후 게임 GUI 비주얼 편집.

## 4. 핵심 추상 데이터 객체(초안)

- `WorkspaceSnapshot`: 게임 버전, 모드 경로·로드 순서, source hashes, active DLC.
- `SourceNode`: 원본 파일·문자 위치·AST/CST 범위.
- `ResolvedDefinition`: 최종 정의와 shadowed/overridden 정의, 적용 규칙, provenance.
- `SymbolRef`: 타입/Scope와 참조 정체성, 원본·의존성 그래프.
- `EditPlan`: 작업 목록, 대상 파일·리비전·검증 조건, 예상 Diff, 권위 원본.
- `Transaction`: stage → preview → validate → apply/abort → rollback.
- `Diagnostic`: 규칙 ID, severity, verification_status, confidence, 증거.
- `BuildManifest`: 게임 빌드, 툴체인 커밋, 콘텐츠 해시, 산출물 목록.
- `HistoricalEvidence`: 사료·연대·지역·범위·신뢰도 A/B/C/U.

실제 Rust 타입명/저장 형식은 구현 전 확정되지 않았다. **API 의미 계약을 먼저 설계**하고 코드를 나중에 작성한다.

## 5. Codex와 Human의 접근 방식

| 단계 | Codex | Human |
|---|---|---|
| 찾기 | 쿼리 API로 심볼·근거·사용처 조회 | 검색창·국가/경제/지도 탐색 |
| 변경 설계 | Typed EditPlan 생성 | 폼·지도·코드에서 같은 EditPlan 생성 |
| 영향/미리보기 | JSON semantic diff·진단·provenance | 시각 Diff·그래프·오류 패널 |
| 적용 | 권한 있는 CLI apply | GUI 명시 적용·승인 |
| 검증 | check → 보고서 → 제한된 재수정 | 같은 진단을 파일/객체로 표시 |
| 런타임 확인 | 별도 게임 실행 증거 요청·수집 | 인게임 GUI·동작 확인 |

## 6. 독립 분석기와 툴체인의 관계

기존 [11 Analyzer 아키텍처](ANALYZER_ARCHITECTURE.md)와 [12 Analyzer 로드맵](../planning/ANALYZER_ROADMAP.md)은 **Shared Core 중 읽기/검증 계층의 설계**로 유지한다.

- Analyzer는 독립적으로 CLI 실행 가능해야 한다.
- Toolchain의 편집 엔진과 사람용 GUI는 Analyzer의 API/진단을 사용한다.
- Tiger/Jomini/pdx-parser-go/Toolkit의 실행·라이브러리를 필수로 연결하지 않는다.
- 기능별 구현을 과도하게 쪼개 **독립 프로세스/마이크로서비스**로 시작할 필요는 없다. 초기에는 하나의 Rust workspace에서 라이브러리 모듈·CLI를 우선한다.
- 개발 초기 인게임 확인을 완전히 없애지는 못한다. 고위험 시작 날짜와 실제 경제·전쟁·AI는 최소 엔진 테스트가 필요하다.

## 7. 현재 저장소 및 미래 논리 모듈 경계

현재 저장소에는 `Twilight_of_Antiquity/{docs,mod,data,assets,toolchain,tests}`를 실질적인 최상위 작업 영역으로 구분한다. 문서는 `docs/{product,history,gameplay,architecture,planning,reference,quality}`로 분류한다. **Rust workspace와 세부 crates/apps는 구현 단계에서만 생성**한다.

```text
Twilight_of_Antiquity/
├── mod/                             # Vic3 모드의 게임 상대 경로만
├── data/                            # 역사·정형 데이터의 authoritative source
├── assets/                          # 편집용 원본 그래픽/라이선스
├── tests/                           # 정적·통합·런타임 검증
└── toolchain/
    ├── crates/                      # 실제 코딩이 시작되면
    │   ├── workspace/
    │   ├── pdx-syntax/
    │   ├── mod-overlay/
    │   ├── semantics/
    │   ├── diagnostics/
    │   ├── edit-engine/
    │   ├── data-engine/
    │   └── build-engine/
    └── apps/
        ├── cli/
        └── human-workbench/
```

이전 `tools/saecvar-analyzer/` 및 `tools/saecvar-toolchain/`은 실제 만들어진 적 없는 **옛 제안 경로**다. 현재 구조의 **유일한 도구 소스 위치 제안은 `toolchain/`**이다. 상세 내용은 [REPOSITORY_LAYOUT](REPOSITORY_LAYOUT.md)을 참조한다.

## 8. 안전성·동시성·검증 경계

- 게임/DLC 설치 파일은 읽기 전용; 실제 게임파일 직접 편집 금지.
- AI/GUI가 동시에 동일 객체를 수정할 경우 expected revision/hash가 맞지 않으면 적용 차단 후 재분석.
- 편집 엔진은 가능한 한 AST/CST 범위에만 변경을 국한하고 주석·공백·중복 키를 보존.
- 다중 파일 변경은 stage→preview→validate→atomic apply/rollback으로 처리.
- 파일 외부 수동 변경 감지 후 즉시 index/overlay 갱신. 같은 파일의 generated/manual 소유권 충돌 금지.
- unsupported schema, inferred history, engine-runtime-only checks를 '정상'으로 둔갑시키지 않음.
- 공개 CI에는 게임 원본 데이터가 아닌 synthetic fixtures만. 로컬 전수 분석은 사용자 설치 경로를 읽음.

## 9. '툴체인 완성'의 최소 정의

GUI 화면 수나 분석기 규칙 개수로 평가하지 않는다. **상품/건물 1개를 Codex와 사람 모두 동일한 API 경로로 조회·수정하고, Diff/참조 검사 후 안전하게 반영하며 게임 파일로 내보내는 능력**이 MVP 완성 기준이다. 사람용 완전한 고급 지도/GUI 편집기는 그 다음 단계다.
