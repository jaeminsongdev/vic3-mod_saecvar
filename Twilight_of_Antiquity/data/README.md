# data — 600년 역사·게임 정형 데이터 원본

출처와 불확실성을 가진 역사·경제·인구 데이터를 저장할 계획입니다. 현재 실제 데이터 파일/스키마는 아직 만들지 않았습니다.

향후 `historical/`(정치체·문화·종교·POP), `sources/`(문헌·고고학·추정 근거), `schemas/`(데이터 계약), `balance/`(게임 밸런스 입력)으로 분리할 수 있습니다.

각 도메인마다 **authoritative source를 단일 지정**하고 생성된 PDX와의 소유권 충돌을 막습니다. 사료 신뢰도 A/B/C/U 및 추정 연대·영향권을 보존하며, 자료가 부족하다고 가상의 경계를 사실인 것처럼 단정하지 않습니다.

[세계사 범위](../docs/history/WORLD_600.md) · [데이터 생성·검증 로드맵](../docs/planning/MASTER_ROADMAP.md) · [저장소 구조](../docs/architecture/REPOSITORY_LAYOUT.md)
