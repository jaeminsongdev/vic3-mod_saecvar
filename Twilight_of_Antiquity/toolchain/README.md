# toolchain — SAECVAR 공통 모딩 엔진 (개발 예정)

게임 원본/DLC와 SAECVAR 모드를 **읽기 전용 분석**, 구조적 수정, 정적 검증, 코드·데이터 생성, Codex CLI/API, 사람용 GUI에 연결하는 자체 Rust 작업공간을 개발할 위치입니다. **현재 Cargo.toml이나 Rust 코드는 생성하지 않았습니다.**

## 단계별 목표

1. M0~M3: PDX Parser(CST/AST), Overlay Resolver, 심볼·Scope, 정적 검사·JSON 진단
2. M4: 구조적 EditPlan, source-preserving 수정, semantic diff, atomic apply·rollback
3. M5~M6: 국가·POP·상품·건물 데이터 작성 API 및 검증 가능한 빌드
4. M7~M8: 같은 API를 사용하는 Human Workbench, 지도·그래프·GUI 편집
5. M9+: 증분·캐시·Codex 자동 재검사·로그·성능 분석

**논리적 패키지(아직 미생성):** `crates/` 아래 `pdx-syntax`, `workspace`, `mod-overlay`, `semantics`, `diagnostics`, `edit-engine`, `data-engine`, `build-engine` 등. `apps/` 아래 CLI와 Human Workbench.

**의존성:** Tiger/Jomini 등은 검증 규칙·구현 방식의 참고용으로만 사용하고, 독립 Analyzer를 외부 도구 실행이나 라이브러리 사용에 의존시키지 않습니다. 라이선스 준수 전제.

[공통 툴체인 아키텍처](../docs/architecture/TOOLCHAIN_ARCHITECTURE.md) · [분석기 아키텍처](../docs/architecture/ANALYZER_ARCHITECTURE.md) · [개발 순서](../docs/planning/MASTER_ROADMAP.md) · [저장소 구조](../docs/architecture/REPOSITORY_LAYOUT.md)
