# SAECVAR — 전 시리즈 통합 로드맵

> 2026-10-17. **T와 V는 독립 개발 트랙**. 이 문서는 프로젝트 포트폴리오의 최상위 선후관계이며 개별 시리즈 마일스톤을 대체하지 않는다.

| 트랙 | 프로젝트 | 목적 | 현황 | 공식 로드맵 |
|---|---|---|---|---|
| T | [Twilight of Antiquity](../Twilight_of_Antiquity/README.md) | 서기 600~720년 토탈 컨버전 | M0 기반 구현, G0 최소 날짜 실증 수용, M1 초안 사용자 중단 | [M0~M10](../Twilight_of_Antiquity/docs/planning/MASTER_ROADMAP.md) |
| V | [Vanilla Plus](../Vanilla_Plus/README.md) (가칭) | Vanilla+ 편의성·소폭 밸런스 | 연구·기능 설계 문서 반영, 엔진 검증 미착수 | [Q0~Q8](../Vanilla_Plus/docs/planning/MASTER_ROADMAP.md) |

## 통합 순서

```text
SAECVAR: README / 문서 규칙 / 버전·출처 기록
        ├─ Twilight: M0(기반) + G0(최소 실증) → M1(사용자 중단 유지) → M2~M10(기획)
        └─ Vanilla Plus: Q0(버전/API 조사) → Q1(최소 정적·엔진 검증 도구)
                   → Q2(F04/F05/F06 정보·UI) → Q3(F01/F03 건설·기반시설)
                   → Q4(F02 PM 추천·실행) → Q5(F10~F15 경제·외교·군사)
                   → Q6(F07/F08/F09/F17 선택형 Balance Lite)
                   → Q7(F16/F18 역사/뉴스·현지화·성능) → Q8(호환/출시)
```

### 서로의 상태를 침범하지 않는 규칙

1. `Twilight_of_Antiquity/`의 고대 콘텐츠·Rust toolchain·tests와 `Vanilla_Plus/`의 편의성 UI·데이터·tests는 별도 소유. 동시 변경 시 경로별 검토.
2. **Twilight M1의 추가 개발은 사용자 요청으로 중단되어 있다.** QoL Q1 개발을 이유로 자동 재개하지 않으며, QoL 최소 검사기는 그 구현을 선행 조건으로 삼지 않는다.
3. 향후 동일한 PDX 문법 검사, override diff, 로컬라이제이션/테스트 규칙이 실제로 중복되면 라이선스·버전·API 일치성을 검증한 뒤 공통 추출을 결정한다.
4. M0/G0의 Twilight 검증 증거는 QoL 모드의 로딩·전쟁·경제 검증이 아니다. 두 프로젝트의 완료 선언은 분리.
5. 최상위 로드맵의 한 단계 완료란 설계 문서 존재가 아니라 해당 시리즈의 테스트 및 실제 게임 증거가 필요한 경우 그것이 확보된 상태를 의미한다.

### 격리된 일정·개발 리소스

- Twilight: [현재 상태](../Twilight_of_Antiquity/docs/planning/STATUS.md), [기존 M0/G0 증거](../Twilight_of_Antiquity/docs/quality/M0_G0_REPORT.md)를 그대로 유지한다.
- QoL: 먼저 [56개 요구](../Vanilla_Plus/docs/features/README.md)를 기능별 엔진 지원 여부로 분류하고, 기능을 순차 수직 슬라이스로 만든다.
- 각 연구 항목에는 `확인됨/추정/미검증/불가` 구분과 대상 패치/DLC를 별도 기록한다.
