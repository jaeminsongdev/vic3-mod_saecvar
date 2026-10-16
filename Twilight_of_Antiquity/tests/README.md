# tests — 자체 검사·회귀 및 엔진 검증

합성 fixture 검사와 실제 엔진 검증을 구분합니다. 현재 Rust 경로 소유권 회귀 테스트와 `runtime/test_g0.py`의 실험 안전성 테스트가 있습니다. 게임 실행 증거는 Git 제외된 `.local/g0/`에 보존하며, 공개 요약은 [M0/G0 결과](../docs/quality/M0_G0_REPORT.md)를 참조합니다.

[G0 실행 방법](runtime/README.md). `fixtures/pdx`에는 이미 작성한 M1 초안의 공개 가능한 합성 입력 3개가 있습니다. 해당 파서 테스트 11개와 `integration/check_m1.py`는 초안 검증 결과이며 M1 완료를 뜻하지 않습니다. 사용자 요청으로 M1 추가 개발은 중단했습니다.

향후 구조: `fixtures/`(공개 가능한 최소 입력), `unit/`(파서·의미·편집), `integration/`(구조 데이터→모드, 레퍼런스), `runtime/`(600~720 엔진 시작·세이브·경제·AI), `benchmarks/`(속도/메모리/시간).

게임 없이 가능한 정적 검사·모델 예측을 실제 엔진 검증 성공으로 보고하지 않습니다. 게임 설치 파일·실제 세이브·개인 경로가 공개 CI에 유출되지 않도록 합니다.

[검증 기준](../docs/quality/VALIDATION.md) · [개발 순서](../docs/planning/MASTER_ROADMAP.md) · [구조](../docs/architecture/REPOSITORY_LAYOUT.md)
