# tests — 자체 검사·회귀 및 엔진 검증

분석기와 데이터 편집기의 **합성 fixtures** 및 장기 회귀 시험, 실제 게임 실행으로만 판정 가능한 검증을 구분합니다. 현재 테스트 케이스나 실행 기록은 아직 없습니다.

향후 구조: `fixtures/`(공개 가능한 최소 입력), `unit/`(파서·의미·편집), `integration/`(구조 데이터→모드, 레퍼런스), `runtime/`(600~720 엔진 시작·세이브·경제·AI), `benchmarks/`(속도/메모리/시간).

게임 없이 가능한 정적 검사·모델 예측을 실제 엔진 검증 성공으로 보고하지 않습니다. 게임 설치 파일·실제 세이브·개인 경로가 공개 CI에 유출되지 않도록 합니다.

[검증 기준](../docs/quality/VALIDATION.md) · [개발 순서](../docs/planning/MASTER_ROADMAP.md) · [구조](../docs/architecture/REPOSITORY_LAYOUT.md)
