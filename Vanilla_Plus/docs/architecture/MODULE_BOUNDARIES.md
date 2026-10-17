# Vanilla Plus — 논리 모듈·실행 계약

> 스크립트·GUI에서 실제 사용 가능한 인터페이스는 Q0에 검증한다. 아래는 코딩 전 개념적 아키텍처다.

| 경계 | 소유 기능 | 책임 | 실제 모드 규칙 변경 |
|---|---|---|---|
| Core | 전 기능 공통 | 게임빌드·DLC 기준, 설정 가용성, 오류/중단 정책, override 목록 | 원칙상 없음 |
| UI & Insights | F04/F05/F06/F14/F15/F16/F18 | 읽기 전용 경제·POP·군사·정치 지표, 알림·필터·단축키 | 없음 |
| Operations | F01/F02/F03/F10/F11/F12/F13 | 기존 법률·기술·경제비용을 지키는 일괄·자동 조작 | 정상 게임 행동만 |
| Balance Lite | F07/F08/F09/F17 | 악명/동화/편입/칙령/교통의 제한적 보정·상한 | 선택형, 비활성 기본 검토 |
| Compatibility | 모든 기능 | 정적 검사, GUI/defines 충돌, 세이브·DLC·버전·타모드 조합 | 없음 |

```text
상태 관측 → 타깃 게임버전/권한 검증 → 영향·비용 평가
 → 추천 근거 표시 → 수동 확인 또는 사전 허용 정책
 → 실행 직전 재검증 → 엔진이 허용한 조작 수행 → 결과/중단 사유 기록
```

## 기능 계약(설계안, 게임 API 아님)

```yaml
feature_id: F02-R03
game_build: "실측 필요"
dlc_profile: "실측 필요"
feasibility: PENDING
observations: []
expected_effect: null
permission_checked: false
vanilla_overrides: []
fallback: "signal_missing => no action"
evidence: []
```

- 없는 엔진 이벤트·불가능한 값 읽기·범용 게임 상태 Undo를 가능하다고 단정하지 않는다.
- 기존 모드 코드는 라이선스 확인 전 직접 복사하지 않으며 Steam 링크는 설계 참고 정보만 저장한다.
- 동일 GUI 파일 override·defines 충돌·시뮬레이션 틱 비용을 개별 관리한다.
- 게임 패치로 기능이 바닐라에 추가되면 그 기능의 구현 우선순위를 낮추거나 제거한다.
- 별도 Steam 모드 패키지 5개로 분리할지는 구현/호환성 검증 후 결정. 현 단계에서는 위 분류가 **논리 경계**다.

## 횡단 정책의 소유권

[X01~X07](../features/CROSS_CUTTING_REQUIREMENTS.md)은 모든 기능군에 적용되며, F18 전용이 아니다. [요구 ID 등록부](../planning/REQUIREMENT_REGISTER.csv)의 테스트·실현성 상태를 기준으로 릴리스 범위를 결정한다. 외부 Steam Workshop 사례는 코드 재사용 승인이나 실제 게임 호환성의 증거가 아니며, [기능별 근거표](../planning/FEATURE_SOURCE_MATRIX.csv)는 범주 단위 참고 매핑이다.
