# 18. Codex·사람 공통 모딩 작업 프로토콜

- 상태: **설계 초안 v0.3**. CLI/API 및 GUI는 현재 미구현.
- 관련: [14 기능](../reference/TOOLCHAIN_FUNCTION_CATALOG.md) · [16 구조](TOOLCHAIN_ARCHITECTURE.md) · [17 로드맵](../planning/MASTER_ROADMAP.md)

## 1. 공통 작업 명령의 의미

| 작업 | 공통 처리 계약 | Codex UI | 사람 UI |
|---|---|---|---|
| Inspect | 현재 최종 정의, 원본·모드 출처와 버전 조회 | 심볼 쿼리/API | 검색·파일·객체 탐색 |
| Search | 파일·정의·근거·의존성 검색 | 구조화 결과 | 필터·검색 패널 |
| Create/Update/Delete/Rename | typed edit operation 생성(즉시 저장 아님) | CLI/API 파라미터 | 폼·표·지도·코드 |
| Preview | 원본/의미 Diff·영향 파일·위험 예측 | JSON·텍스트 Diff | 색상·그래프·차이 화면 |
| Validate | 구문·참조·적용 모델·시대 규칙 | 진단 JSON | 오류 패널 |
| Apply | staging 파일·해시 대조·승인 정책 후 적용 | 명시 실행 | 적용 버튼·검토 |
| Rollback | 해당 트랜잭션의 이전 원본 상태 복원 | 복구 명령 | 실행 취소 |
| Build | 원본 데이터로 재현 가능한 모드 생성 | 빌드 명령 | 빌드·패키지 UI |
| Runtime Evidence | 실제 게임 세이브/로그/GUI 결과 첨부 | 파일·로그 읽기 | 화면·증거 표시 |

**예제 명칭은 계약 개념**이며 실제 현재 사용할 수 있는 명령을 나타내지 않는다.

## 2. 작업 단계 — 하나의 EditPlan

```text
Request (Codex or Human)
  → Inspect + Resolve + Provenance
  → EditPlan {target, operation, expected_revision, scope, evidence}
  → Preview SemanticDiff + FileDiff + Impact Graph
  → Validate (Static/Model/Unsupported/Runtime Needed)
  → Stage Writes (never to vanilla or DLC)
  → Apply (only if revision and policy match)
  → Re-index + Re-validate + Build
  → Human-readable + machine-readable report
```

'검사 통과'로 자동 적용을 강제하지 않고, 작업 수준/위험 정도에 따라 승인 정책을 둔다. 파서의 미지원 구문이 대상 파일에 존재하면 소스 덮어쓰기 대신 수동 확인을 요구한다.

## 3. 사례 A: 고구려의 600년 철 생산방식 편집

**Codex:** `get building`으로 정의·연결된 철·도구·기술·POP을 찾고, 철 도구 생산량을 변경하는 EditPlan을 생성한다. 검사기는 존재하지 않는 상품 ID, 연결되지 않은 기술, 영향을 받는 다른 생산방식을 찾아 JSON 보고서로 제공한다.

**사람:** GUI 국가·지역 편집기에서 고구려를 선택하고 같은 생산방식의 표나 그래프 값을 변경한다. 자동으로 **같은 EditPlan API**가 생성된다. 변경 전후 생산량/상품 입력과 파일 위치를 확인한 뒤 적용한다.

**공통 엔진:** 권위 원본 확인 → CST 범위 수정 또는 원본 데이터 생성 → 오버레이·심볼 검사 → 다중 파일 변경 staging → 일치하는 revision에만 적용. 실제 시장가격·군사·AI 반응은 별도의 엔진 검증으로 남는다.

## 4. 사례 B: 서기 600년 정치지도 수정

- 인간은 지도에서 기존 province를 선택해 국가/주 소속 변경을 계획.
- Codex는 동일 province ID와 정치체 ID로 구조화 편집 요청 생성.
- 공통 엔진은 중복 소속·도시/항구 마커·변경 영향 및 생성 산출물을 검사.
- 해당 시대에 정치적 직접 지배가 불명확하다면 사료 신뢰도를 표시.
- 게임 로더가 실제 지도를 제대로 읽는지, 군대 이동·해상 항로가 정상인지 **엔진 실행 검증 대상**으로 남긴다.

B02/B03 지도 편집은 R07/R08의 '기존 province/주 소속' 편집 기능을 우선 참고하며, 완전히 새 지형을 그리는 도구와 동일시하지 않는다.

## 5. 사례 C: 이벤트 체인 제작

- 요청: '수나라의 고구려 침공을 무조건 612년에 발생시키지 않고 정치·경제·외교 조건에 따라 발생'.
- 공통 엔진이 트리거·국가 존재 조건·관련 Scope·다른 이벤트 참조를 인덱싱.
- Codex가 선택지/결과 코드를 작성하거나 사람이 이벤트 그래프에서 분기 연결.
- 정적 조건 충돌·누락된 로컬라이제이션·이벤트 미참조 후보를 검사.
- 실제 발생률·AI 대응·전쟁 결과는 게임 진행이 필요하다고 보고.

## 6. 다중 사용자/에이전트 충돌

원본 revision이 `r42`인 객체를 Codex A와 사람 B가 동시에 수정할 때 A가 먼저 적용하면, B의 `expected_revision=r42`는 더 이상 맞지 않는다. 시스템은 B의 적용을 막고 새로운 `r43`을 기준으로 semantic diff와 병합 가능성을 다시 제시한다.

실제 구현에서는 파일 및 **정의/데이터 원본 단위**로 revision과 소유권을 관리한다. 복수 파일 트랜잭션의 중간 실패는 rollback 가능해야 한다. 자동 merge는 의미 보존이 증명되지 않으면 금지한다.

## 7. 증거 분류와 실행 정책

- `STATIC_VERIFIED`: 현재 지원하는 정적 규칙만 검증.
- `MODEL_PREDICTED`: 모드 적용 모델에 따른 예측.
- `RUNTIME_REQUIRED`: 실제 엔진 확인 필요.
- `UNSUPPORTED_RULE`: 지원 범위 밖.

오류 심각도는 별도 필드다. **`STATIC_VERIFIED`가 곧 플레이 가능한 상태는 아니다.** 게임을 실행하지 않는 작업이 기본이지만, 600년 시작과 경제 루프 같은 변경 후에는 최소 게임 엔진 확인을 해야 한다.

## 8. 사용자 인터페이스 MVP에 포함할 것

- 공통 객체 검색, 원본/모드 최종 정의 확인, 파일·문서 링크.
- 편집 폼/테이블/텍스트의 어떤 변경도 EditPlan에 도달.
- 오류·경고·미지원 구분, 실제 적용 전 변경 영향 미리보기.
- 승인·되돌리기·과거 변경 목록.
- 게임 로딩·GUI 확인이 필요한 미완료 항목 별도 큐.

## 9. 테스트 계획

- Codex와 GUI가 **같은 객체 수정 요청**에서 동일한 semantic diff/PDX 산출물을 만드는지 테스트.
- generated 데이터 수정 차단/승인된 역수입 절차.
- 동시 수정 충돌·부분 파일 저장 실패·롤백.
- 파서가 오류를 복구했지만 안전한 쓰기가 불가능한 파일에 대한 쓰기 차단.
- 원본 게임 설치가 없는 공개 fixture 모드 vs 설치된 게임의 로컬 전체 분석.
- 두 차례 연속 검토와 게임 내 확인을 서로 다른 완료 기준으로 관리.
