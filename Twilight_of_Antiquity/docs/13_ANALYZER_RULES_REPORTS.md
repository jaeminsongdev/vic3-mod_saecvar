# 13. SAECVAR Analyzer — 검증 규칙·진단·CLI 계약

- 상태: 설계안 v0.1; **아래의 CLI·출력 형식은 아직 구현되지 않음**
- 관련: [11 아키텍처](11_ANALYZER_ARCHITECTURE.md) · [12 구현 로드맵](12_ANALYZER_ROADMAP.md)

## 1. 규칙 카탈로그 (초기 후보)

| 분류 | 규칙 ID 예시 | 검사 내용 | 결과 속성 |
|---|---|---|---|
| 구문 | `SYN-001` | 닫히지 않은 블록/문자열 | 정적 ERROR |
| 구문 | `SYN-002` | 미지원 연산자·파일 방언 | UNSUPPORTED_RULE |
| 로드·적용 | `MOD-001` | 파일 경로 덮어쓰기 충돌 | MODEL_PREDICTED |
| 로드·적용 | `MOD-002` | 객체 교체 의도가 정적 최종 정의에 반영되지 않음 | MODEL_PREDICTED |
| 참조 | `REF-001` | 정의되지 않은 상품·건물·기술 ID | STATIC_VERIFIED |
| 참조 | `REF-002` | 중복/차단된 심볼 정의 | STATIC_VERIFIED / MODEL_PREDICTED |
| 타입/Scope | `SEM-001` | 효과/트리거 컨텍스트·Scope 불일치 | STATIC_VERIFIED 또는 UNSUPPORTED_RULE |
| 로컬라이제이션 | `LOC-001` | 기대 언어의 로컬라이제이션 키 누락 | STATIC_VERIFIED |
| 경제 | `ECO-001` | 생산방식이 존재하지 않는 상품을 참조 | STATIC_VERIFIED |
| 경제 | `ECO-002` | 필수 투입 상품의 생산 경로가 보이지 않음 | MODEL_PREDICTED / WARNING |
| 기술 | `TECH-001` | 해금 트리의 순환 또는 도달 불가능 후보 | MODEL_PREDICTED |
| 역사 | `HIST-001` | 600년 시작 정치체가 출처 데이터와 충돌 | 출처 신뢰도 기반 WARNING |
| 역사 | `HIST-002` | 특정 연도에 국가를 무조건 삭제할 가능성 | MODEL_PREDICTED |
| 군사/지도 | `MAP-001` | 주·국가 참조 및 시작 소유권 누락 | STATIC_VERIFIED |
| 버전 | `VER-001` | 검사 엔진·스키마 대상 게임 버전 불일치 | WARNING/UNSUPPORTED_RULE |

위 ID는 **설계 예시**이며 최종 규칙 ID를 확정한 것이 아니다. 특정 역사·게임성 규칙의 실패를 파서 문법 오류와 같은 확실도로 분류하지 않는다.

## 2. 심각도와 검증 상태는 별개

**Severity**: `ERROR` (지원 범위 내 확정 오류), `WARNING` (잠재 문제), `INFO` (설명), `HINT` (선택적 개선).

**Verification status**:
- `STATIC_VERIFIED`: 이 규칙의 입력과 정적 조건이 검증된 상태. 엔진 실행 성공을 뜻하지 않는다.
- `MODEL_PREDICTED`: 오버레이·조건·의존성 모델에 근거한 예측.
- `RUNTIME_REQUIRED`: 게임 엔진 실행 없이는 확인 불가능.
- `UNSUPPORTED_RULE`: 분석기 미지원 문법·버전·검사 종류.

**Confidence**: `high / medium / low`. 정보가 부족한 부분을 억지로 `ERROR/high`로 분류하지 않는다.

## 3. 구조화된 진단 형식 제안

```json
{
  "schema_version": "0.1-draft",
  "tool": "saecvar-analyzer",
  "game": {
    "id": "victoria3",
    "version": "unverified",
    "load_order_fingerprint": "example-only"
  },
  "ruleset_version": "draft",
  "summary": {
    "errors": 1,
    "warnings": 0,
    "unsupported": 0
  },
  "diagnostics": [
    {
      "rule_id": "REF-001",
      "severity": "ERROR",
      "verification_status": "STATIC_VERIFIED",
      "confidence": "high",
      "file": "common/buildings/saecvar_ironworks.txt",
      "range": {
        "start_line": 42,
        "start_column": 4,
        "end_line": 42,
        "end_column": 24
      },
      "message": "참조하는 상품 ID가 없습니다.",
      "symbol": "saecvar_wrought_iron",
      "why": "해당 적용 대상의 최종 상품 정의를 찾지 못했습니다.",
      "suggested_fix": "상품 정의 또는 참조 ID를 확인하세요.",
      "related_locations": [],
      "source_provenance": [],
      "fingerprint": "example-ref-001"
    }
  ]
}
```

이 JSON은 **가상 설계 예제**이며 실제 검사 결과·정식 API가 아니다. 사용자에게 공개하는 보고서에는 가정·게임 버전·모델 한계를 명시한다.

## 4. 분석 대상 범위와 실패 정책

- 설정 가능한 분석 루트: 원본 게임, DLC, 활성 모드, 분석 대상 시리즈 모드.
- 기본 공개 CI: 공개 가능한 synthetic fixtures와 샘플 규칙만 사용.
- 로컬 전체 검증: 사용자 설치 게임 파일을 **read-only**로 읽고 원본 텍스트는 리포트에 불필요하게 복사하지 않는다.
- 지원된 규칙에서 `ERROR` 발견 시 기본 CI 실패; `WARNING`은 설정에 따라 실패 여부 선택.
- `UNSUPPORTED_RULE`은 성공으로 취급하지 않고 검사범위 요약에 표시.
- 파일 한 개의 구문 실패가 다른 독립 파일 검사까지 중단하지 않도록 설계.

## 5. 예정 CLI 인터페이스

```sh
# 아래 명령은 구현 전 인터페이스 초안임
saecvar-analyzer check --full --format json
saecvar-analyzer check --changed --format json
saecvar-analyzer check --full --format sarif
saecvar-analyzer check --full --format html
saecvar-analyzer explain REF-001
```

`--changed`는 변경 파일만 파싱하되 영향을 받는 참조·적용 정의까지 다시 검증한다. 전체 검사와 동등한 관련 진단을 반환해야 한다. `explain`은 규칙의 의미, 심각도, 검증 가정과 수정 후보를 제공한다.

## 6. Codex 사용 워크플로

1. Codex가 모드 스크립트 또는 역사 데이터를 변경.
2. 정적 CLI 실행 → 결과 JSON을 저장.
3. `ERROR`인 규칙의 `file/range/why/related_locations`를 우선 분석.
4. 해당 문제의 영향 경로를 확인하여 **관련 파일만** 수정.
5. 동일 명령을 다시 실행하고 오류 해소·새 회귀가 없는지 확인.
6. 엔진 필요 항목은 자동으로 '성공' 처리하지 않고 **별도 실행 검증 목록**에 남김.

코덱스의 무제한 자동 패치와 스크립트 의미를 바꿀 수 있는 대량 자동 수정을 초기 기능으로 넣지 않는다.

## 7. 검증 스위트·관측치

- Parser: 골든 입력/오류 복구, 주석/중복 키·순서 보존, fuzz.
- Overlay: 동일 경로·다중 객체·INJECT/REPLACE·모드 순서, 실제 엔진과 대조된 fixtures.
- Semantic: 정의 누락·Scope 위반·게임 버전 차이에 대한 참/거짓 양성 사례.
- History: 사료 A/B/C/U 등급별 경고 신뢰성.
- Incremental: 수정 전/후 full 대비 affected diagnostics 결과 동일성.
- Performance: 첫 실행·캐시 히트·메모리 점유 등 **실측 후 목표 설정**.
- Versioning: 게임 패치 또는 스키마 변경 시 이전 캐시 무효화·검증 가능.

## 8. 결과 재현에 필요한 최소 메타데이터

모든 보고서는 분석기 자체 버전, 게임 버전, 규칙 버전, 활성 모드 목록 및 순서의 fingerprint, 콘텐츠 해시, 검사한 폴더와 제외 항목, 미지원 규칙을 기록해야 한다. 경로·설치 정보가 공개 산출물에 누출되지 않도록 사용자의 로컬 경로는 선택적으로 가린다.

## v0.3: 공통 EditPlan과 CLI 연결

분석기의 `check`와 `explain`은 [16 Toolchain](16_TOOLCHAIN_ARCHITECTURE.md)의 일관된 `Inspect → EditPlan → Preview → Validate → Apply → Rollback` 절차 중 **읽기·정적 검증 단계**를 담당한다. Codex/API와 GUI는 같은 Diagnostic 객체를 사용한다. 파일을 수정하는 `edit`/`apply`/`rollback` 인터페이스는 분석기 자체가 아니라 공통 편집 엔진에 위치하며, 세부 트랜잭션과 충돌 해석은 [18 워크플로](18_SHARED_WORKFLOWS.md)에 정의한다.
