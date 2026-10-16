# toolchain — SAECVAR 공통 모딩 엔진

M0 Rust workspace를 생성했습니다. `crates/workspace`는 원본/모드의 실제 경로, 중첩 여부와 최소 식별 파일을 읽기 전용으로 검사합니다. CLI의 M0 명령은 `inspect-paths <game-data-root> <mod-root>`입니다. OS 쓰기 권한을 변경하지 않습니다.

**M1은 사용자 요청으로 추가 개발 중단 상태입니다.** 이미 작성한 `crates/pdx-syntax`와 `parse <file.txt>` 명령은 미완료 초안으로 보존합니다. `.txt` 초기 구문 부분집합에서 토큰·원문·주석·중복 키·순서를 보존하고 CST/AST와 오류 복구를 제공하지만 inline math, typed value, 다른 방언 등의 전체 지원은 없습니다. 파서·JSON 출력이 엔진/의미 검증을 대신하지 않습니다.

`parse` 초안은 `saecvar.syntax.v0` JSON으로 byte offset과 1부터 시작하는 줄·Unicode scalar 열을 보고합니다. 초기 BOM은 열 계산에서 제외하며 열은 UTF-16 단위가 아닙니다. 종료 코드 0은 해당 부분집합에서 진단 없음, 1은 구문 또는 미지원 진단, 2는 입력·인코딩·방언·사용법 오류입니다. `.yml`/`.gui` 및 UTF-8이 아닌 입력은 지원하지 않습니다. 문자열은 단일 행 부분집합이고 escape를 해석하지 않은 원문으로 유지합니다. 중첩 한도 128 초과는 불투명 범위와 미지원 진단으로 보존합니다. M3의 완성된 Analyzer 보고서 계약은 아직 구현하지 않았습니다.

```powershell
cargo test --workspace -j 1
cargo run -j 1 -p saecvar-cli -- inspect-paths '<Victoria 3>/game' '<series>/mod'
```

`configs/game-target.json`에 버전·빌드·날짜와 원본 식별 해시를 고정합니다. 개인 설치 경로는 명령 인자와 Git 제외된 `.local/` manifest에서만 관리합니다. 엔진 실험 도구는 [tests/runtime](../tests/runtime/README.md)에 있습니다.

## 단계별 목표

1. M0~M3: PDX Parser(CST/AST), Overlay Resolver, 심볼·Scope, 정적 검사·JSON 진단
2. M4: 구조적 EditPlan, source-preserving 수정, semantic diff, atomic apply·rollback
3. M5~M6: 국가·POP·상품·건물 데이터 작성 API 및 검증 가능한 빌드
4. M7~M8: 같은 API를 사용하는 Human Workbench, 지도·그래프·GUI 편집
5. M9+: 증분·캐시·Codex 자동 재검사·로그·성능 분석

**향후 패키지:** `mod-overlay`, `semantics`, `diagnostics`, `edit-engine`, `data-engine`, `build-engine`, Human Workbench는 미구현입니다. `pdx-syntax`는 위에서 설명한 미완료 초안만 존재합니다.

**의존성:** Tiger/Jomini 등은 검증 규칙·구현 방식의 참고용으로만 사용하고, 독립 Analyzer를 외부 도구 실행이나 라이브러리 사용에 의존시키지 않습니다. 라이선스 준수 전제.

[공통 툴체인 아키텍처](../docs/architecture/TOOLCHAIN_ARCHITECTURE.md) · [분석기 아키텍처](../docs/architecture/ANALYZER_ARCHITECTURE.md) · [개발 순서](../docs/planning/MASTER_ROADMAP.md) · [저장소 구조](../docs/architecture/REPOSITORY_LAYOUT.md)
