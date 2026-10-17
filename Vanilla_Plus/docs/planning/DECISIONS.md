# Vanilla Plus — 범위 결정·미해결 사항

- **확정된 사용자 요구:** SAECVAR 저장소에 편의성(반복 조작/UI/정보/자동화)과 동화·악명·편입 등 소폭 밸런스 조사 내용을 전부 기능 단위 문서화하고 기존 로드맵 반영.
- **확정 범위:** 고대 토탈 컨버전이나 완전한 경제·인구·군사 오버홀을 새 프로젝트에 포함하지 않음.
- **설계 제안:** `Vanilla_Plus`는 임시 시리즈명, F01~F18(56개 세부 요구), Q0~Q8 일정, Core/UI/Automation/Balance Lite/Compatibility 논리 모듈. 이 모듈을 각각 별도 Steam 모드로 배포할지는 미정.
- **안전 기본값:** 자동화는 사용자 보호/사유 표시/기존 제약 유지, 게임 규칙 변경은 선택형이며 프리셋 수치는 테스트 가설.
- **미확정 Q01:** 대상 정식 패치/DLC/엔진 API 및 UI 파일 경로.
- **미확정 Q02:** GUI/PM 자동화와 부분 Undo 등의 모딩 기술적 한계 및 대안.
- **미확정 Q03:** 다른 대형 UI 모드와의 파일 충돌/호환 패치 범위.
- **미확정 Q04:** 수치 완화의 5년·20년 AI/플레이어별 편차, 멀티플레이.
- **미확정 Q05:** 로컬 도구를 Twilight 툴체인과 공유할지 여부. Twilight 사용자 중단된 M1의 재개는 별도 지시가 있어야 함.

[기능 전체](../features/README.md) · [QA](../quality/QA_PLAN.md) · [포트폴리오 일정](../../../docs/PORTFOLIO_ROADMAP.md).

## 2026-10-17 교차 검수 보완 기록

- 56개 `Fxx-Ryy`는 [등록부](REQUIREMENT_REGISTER.csv)에서 상태 `PLANNED/PENDING/NOT_RUN`으로 통일. 18개 기능군의 Workshop/커뮤니티 자료는 [FEATURE_SOURCE_MATRIX.csv](FEATURE_SOURCE_MATRIX.csv)에 **범주 단위 참고 근거**로 연결.
- 공통 안전·권한·대칭·성능 요구 X01~X07은 F18에서 별도 [횡단 요구사항](../features/CROSS_CUTTING_REQUIREMENTS.md)으로 정리. 실제 모딩 코드 변경이나 기존 수치 채택을 의미하지 않음.
- 외부 공식 일정 변경은 [VERSION_MATRIX](../quality/VERSION_MATRIX.md)에서 별도 확인. 1.14 베타 결과와 미래 1.15 정식 검증을 합산하지 않음.
