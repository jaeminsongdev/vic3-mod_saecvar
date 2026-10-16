# 14. SAECVAR 통합 모딩 툴체인 — 기능 카탈로그

- 상태: **v0.3 기획·기능 범위** (구현 완료 아님)
- 작성·검토 기준: 2026-10-16
- 대상: `Twilight_of_Antiquity`를 첫 사용자로 하는 **Codex + 사람 공통 모딩 도구**
- 상세 구현 선후관계: [17 통합 개발 로드맵](17_MASTER_ROADMAP.md)
- 레포 식별자: [15 검증된 참고 저장소](15_REFERENCE_REPOSITORIES.md)

## 개념 및 집계

- **공통 기능 A~F 56개**: 같은 모델과 검증기를 CLI/API와 GUI가 공유.
- **Codex 특화 AI 14개**: 에이전트의 안전한 수정·자료 검색·자동 반복을 돕는 인터페이스.
- **Human 특화 HU 16개**: 사람이 시각적·직관적으로 작업하는 인터페이스.
- **횡단 원칙 X 10개**: 모든 기능에 적용하는 소스·트랜잭션·버전·검증 계약.
- **총 96개의 추적 항목**(고유 기능 86개 + 횡단 기반 원칙 10개). **96개 구현 모듈이 필요하다는 뜻은 아님.**

**중요:** 표의 참고 레포 ID는 '그 기능을 설계할 때 연구할 근거'이며, 해당 레포가 기능 전체를 이미 구현했다고 보장하거나 소스 복제를 허용한다는 의미가 아니다. 특히 X07 같은 작업 동시성 기능은 SAECVAR의 **자체 신규 설계 책임**이다.

### 범위와 책임

`M#`는 최초로 작동할 최소 기능을 갖추는 개발 마일스톤을 의미하고 완성 시점을 뜻하지 않는다. 지원 가능한 PDX 포맷, 게임 적용 규칙, 맵 바이너리, GUI 미리보기 정확도는 실제 게임 버전에 대해 별도 실증해야 한다. 사람용 지도·게임 GUI 편집기는 일반적인 업무용 UI보다 뒤에 구현한다.

## A · 프로젝트·PDX 코드와 안전한 편집

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| A01 | 프로젝트·워크스페이스 | 모드 생성, 시리즈 분리, 메타데이터, 설정 | M0 | [R13](15_REFERENCE_REPOSITORIES.md#r13), [R16](15_REFERENCE_REPOSITORIES.md#r16) |
| A02 | 게임 환경 인식 | 원본·DLC·플레이셋·로드순서·버전 | M0 | [R13](15_REFERENCE_REPOSITORIES.md#r13), [R16](15_REFERENCE_REPOSITORIES.md#r16), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| A03 | 원본 데이터·문서 탐색 | 정의·파일·원본 예제 검색 | M1 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R04](15_REFERENCE_REPOSITORIES.md#r04), [R16](15_REFERENCE_REPOSITORIES.md#r16) |
| A04 | PDX 파싱 | Lexer/CST/AST, 중복키, 위치·오류 복구 | M1 | [R02](15_REFERENCE_REPOSITORIES.md#r02), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| A05 | 모드 적용 해석 | 파일·객체 오버레이와 provenance | M2 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R11](15_REFERENCE_REPOSITORIES.md#r11) |
| A06 | 의미 분석 | 심볼, 타입, Scope, 참조·로컬라이제이션 | M3 | [R04](15_REFERENCE_REPOSITORIES.md#r04), [R05](15_REFERENCE_REPOSITORIES.md#r05), [R01](15_REFERENCE_REPOSITORIES.md#r01) |
| A07 | 소스 보존 편집 | 구조적 수정, 안전 rename·서식 보존 | M4 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R05](15_REFERENCE_REPOSITORIES.md#r05) |
| A08 | 변경 트랜잭션 | 미리보기, semantic diff, 적용·되돌리기 | M4 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| A09 | 참조·변경 영향 그래프 | 사용처·삭제 영향·의존성 순환 | M3 | [R05](15_REFERENCE_REPOSITORIES.md#r05), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| A10 | 콘텐츠 생성기 | 정의 템플릿, 안정 ID, 파일·로컬라이제이션 동시 생성 | M4 | [R10](15_REFERENCE_REPOSITORIES.md#r10), [R13](15_REFERENCE_REPOSITORIES.md#r13) |

## B · 역사·지도·국가 데이터

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| B01 | 사료·불확실성 관리 | 출처·연대·자료 충돌·신뢰도 A/B/C/U | M6 | [R16](15_REFERENCE_REPOSITORIES.md#r16) |
| B02 | 주·프로빈스 지도 편집 | 소속·국경·주 분할·검증 | M8 | [R07](15_REFERENCE_REPOSITORIES.md#r07), [R08](15_REFERENCE_REPOSITORIES.md#r08) |
| B03 | 지도 레이어 시각화 | 정치·문화·종교·인구·자원 분포 | M8 | [R07](15_REFERENCE_REPOSITORIES.md#r07), [R08](15_REFERENCE_REPOSITORIES.md#r08) |
| B04 | 국가 생성·편집 | 국가·수도·문화·시작 소유권 | M5 | [R07](15_REFERENCE_REPOSITORIES.md#r07), [R09](15_REFERENCE_REPOSITORIES.md#r09), [R10](15_REFERENCE_REPOSITORIES.md#r10) |
| B05 | 인구 편집 | POP 규모·문화·종교·직업·주 분포 | M5 | [R07](15_REFERENCE_REPOSITORIES.md#r07), [R09](15_REFERENCE_REPOSITORIES.md#r09) |
| B06 | 문화·종교 편집 | 정의·시대 분포·연결 관계 | M6 | [R10](15_REFERENCE_REPOSITORIES.md#r10), [R09](15_REFERENCE_REPOSITORIES.md#r09) |
| B07 | 시작 역사 시나리오 | 국가·국경·건물·군사·기술·법률 시작값 | M5 | [R09](15_REFERENCE_REPOSITORIES.md#r09), [R13](15_REFERENCE_REPOSITORIES.md#r13) |
| B08 | 정형 데이터 일괄 생성 | 구조화 원본→PDX 파일 생성·검증 | M6 | [R11](15_REFERENCE_REPOSITORIES.md#r11), [R03](15_REFERENCE_REPOSITORIES.md#r03) |

## C · 경제·기술·정치·군사 콘텐츠

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| C01 | 상품 편집 | 가격·수요·공급·참조 | M5 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R10](15_REFERENCE_REPOSITORIES.md#r10) |
| C02 | 건물 편집 | 고용·건설·지역 제한·생산 연결 | M5 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R10](15_REFERENCE_REPOSITORIES.md#r10) |
| C03 | 생산방식 편집 | 입출력, 고용, 해금 조건 | M5 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R10](15_REFERENCE_REPOSITORIES.md#r10) |
| C04 | 정적 경제 관계 분석 | 상품 공급망·순환·의존성·구조적 병목 | M6 | [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| C05 | 기술 트리 편집 | 연구·선행·해금·전파 | M6 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R10](15_REFERENCE_REPOSITORIES.md#r10) |
| C06 | 정치·법률 편집 | 이해집단·제도·법률과 효과 | M7 | [R10](15_REFERENCE_REPOSITORIES.md#r10), [R12](15_REFERENCE_REPOSITORIES.md#r12) |
| C07 | 국가 운영 데이터 | 행정·조세·국고·정부 효과 | M7 | [R10](15_REFERENCE_REPOSITORIES.md#r10), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| C08 | 외교 데이터 | 관계·조약·속국·전쟁목표 | M7 | [R12](15_REFERENCE_REPOSITORIES.md#r12), [R10](15_REFERENCE_REPOSITORIES.md#r10) |
| C09 | 군사 콘텐츠 | 병종·군수·건물·전투 보정 | M7 | [R10](15_REFERENCE_REPOSITORIES.md#r10), [R12](15_REFERENCE_REPOSITORIES.md#r12) |
| C10 | AI 정의·가중치 | AI 기술·건설·외교·정치 스크립트 | M7 | [R04](15_REFERENCE_REPOSITORIES.md#r04), [R05](15_REFERENCE_REPOSITORIES.md#r05) |

## D · 이벤트·일지·분기

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| D01 | 이벤트 제작 | 트리거·선택지·효과·가중치 | M7 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R10](15_REFERENCE_REPOSITORIES.md#r10) |
| D02 | Journal Entry 제작 | 일지 진행·성공·실패·종료 | M7 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R12](15_REFERENCE_REPOSITORIES.md#r12) |
| D03 | 국가 성립·해체 | 조건부 형성·분열·통일·후계국 | M7 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R04](15_REFERENCE_REPOSITORIES.md#r04) |
| D04 | 재사용 스크립트 | scripted trigger/effect/value/modifier | M7 | [R04](15_REFERENCE_REPOSITORIES.md#r04), [R05](15_REFERENCE_REPOSITORIES.md#r05) |
| D05 | On Action 연동 | 이벤트 발생 조건·후속 호출 | M7 | [R12](15_REFERENCE_REPOSITORIES.md#r12), [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| D06 | 이벤트 의존성 그래프 | 호출 관계·분기·순환 탐색 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| D07 | 조건식 정적 해석 | 도달 불가능 조건·모순 후보 | M9 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| D08 | 역사적 분기 검사 | 연도 고정·국가 강제멸망 경고 | M9 | [R01](15_REFERENCE_REPOSITORIES.md#r01), [R04](15_REFERENCE_REPOSITORIES.md#r04) |

## E · 로컬라이제이션·GUI·에셋

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| E01 | 로컬라이제이션 | 키·인코딩·언어별 누락·중복 | M6 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| E02 | 번역·용어 통일 | 용어집·언어별 편차·번역 검수 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| E03 | 이벤트 창 프리뷰 | 텍스트·선택지·상태 표시 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| E04 | 게임 GUI 파일 편집 | GUI 트리·위젯·레이아웃·바인딩 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R12](15_REFERENCE_REPOSITORIES.md#r12) |
| E05 | UI 스키마 검사 | 위젯 유형·리소스·바인딩 연결 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R04](15_REFERENCE_REPOSITORIES.md#r04) |
| E06 | 그래픽 에셋 처리 | DDS/PNG/TGA 검사·전환·미리보기 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R15](15_REFERENCE_REPOSITORIES.md#r15) |
| E07 | 국기·문장 제작 | 색·문양·패턴·에셋 참조 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R15](15_REFERENCE_REPOSITORIES.md#r15) |
| E08 | 에셋 참조 그래프 | 아이콘·텍스처·모델·스크립트 참조 | M8 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R06](15_REFERENCE_REPOSITORIES.md#r06) |

## F · 검증·테스트·배포

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| F01 | 정적 진단 | 구문·Scope·스키마·적용 분석 | M3 | [R01](15_REFERENCE_REPOSITORIES.md#r01), [R04](15_REFERENCE_REPOSITORIES.md#r04), [R05](15_REFERENCE_REPOSITORIES.md#r05) |
| F02 | 도메인 검증 | 역사·경제·기술·법률 규칙 | M6 | [R04](15_REFERENCE_REPOSITORIES.md#r04), [R01](15_REFERENCE_REPOSITORIES.md#r01) |
| F03 | 변경 영향 분석 | 직접·간접 참조와 회귀 위험 | M3 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R05](15_REFERENCE_REPOSITORIES.md#r05) |
| F04 | 진단 보고 | JSON/SARIF/HTML, 원인·위치·근거 | M3 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| F05 | 회귀 테스트 | Fixture·golden·fuzz·오탐 기준 | M1 | [R02](15_REFERENCE_REPOSITORIES.md#r02), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| F06 | 증분 분석 | 파일 캐시·영향 추적·재검사 | M9 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| F07 | 런타임 로그 통합 | error.log 등 별도 실행 증거 수집 | M9 | [R18](15_REFERENCE_REPOSITORIES.md#r18), [R14](15_REFERENCE_REPOSITORIES.md#r14) |
| F08 | 테스트 시나리오 | 국가·경제·외교 등의 재현 입력 | M5 | [R18](15_REFERENCE_REPOSITORIES.md#r18), [R13](15_REFERENCE_REPOSITORIES.md#r13) |
| F09 | 장기 밸런스 분석 | 경제 성장·AI·세이브 비교 | M10 | [R14](15_REFERENCE_REPOSITORIES.md#r14), [R17](15_REFERENCE_REPOSITORIES.md#r17) |
| F10 | 성능 측정 | 로딩·실게임 일/월 진행과 회귀 | M9 | [R14](15_REFERENCE_REPOSITORIES.md#r14), [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| F11 | 게임 버전 호환성 | 패치별 스키마·오버레이·기능 변화 | M0 | [R16](15_REFERENCE_REPOSITORIES.md#r16), [R04](15_REFERENCE_REPOSITORIES.md#r04) |
| F12 | 배포·패키징 | 모드 빌드·메타데이터·릴리스 준비 | M10 | [R13](15_REFERENCE_REPOSITORIES.md#r13), [R06](15_REFERENCE_REPOSITORIES.md#r06) |

## AI · Codex 작업 특화

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| AI01 | 자연어→작업 요구 | 사용자 요청을 검사 가능한 변경 목록으로 변환 | M5 | [R18](15_REFERENCE_REPOSITORIES.md#r18) |
| AI02 | 계획/작업 분해 | 선행 조건·완료 기준·의존성 계획 | M5 | [R18](15_REFERENCE_REPOSITORIES.md#r18) |
| AI03 | 원본 근거 우선 검색 | 게임 원본·문서·실제 예제 찾아보기 | M1 | [R18](15_REFERENCE_REPOSITORIES.md#r18), [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| AI04 | 관련 컨텍스트 조립 | 작업 대상에 필요한 참조만 인출 | M5 | [R05](15_REFERENCE_REPOSITORIES.md#r05), [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| AI05 | 구조화 편집 API | 경로/행 치환 대신 객체 단위 변경 | M4 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| AI06 | 안전한 배치 작업 | 국가/POP/시설 다중 생성·변경 계획 | M6 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R07](15_REFERENCE_REPOSITORIES.md#r07) |
| AI07 | 변경 영향 예측 | 참조 그래프와 적용 결과 미리 확인 | M4 | [R05](15_REFERENCE_REPOSITORIES.md#r05), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| AI08 | 자동 검증 재시도 | 수정→진단→수정→재검사 가드 | M5 | [R18](15_REFERENCE_REPOSITORIES.md#r18), [R01](15_REFERENCE_REPOSITORIES.md#r01) |
| AI09 | 오류 근본 원인 | 표면 오류에서 원인 참조·원본 추적 | M5 | [R18](15_REFERENCE_REPOSITORIES.md#r18), [R01](15_REFERENCE_REPOSITORIES.md#r01) |
| AI10 | 자동화 안전 제어 | 경로 허용목록·정책·잠금·롤백 | M4 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R18](15_REFERENCE_REPOSITORIES.md#r18) |
| AI11 | 변경 보고 | Diff·진단·근거·미검증 상태 설명 | M5 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R18](15_REFERENCE_REPOSITORIES.md#r18) |
| AI12 | 개발 지식 재활용 | 패치별 예외·결정·사례 인덱스 | M9 | [R16](15_REFERENCE_REPOSITORIES.md#r16), [R18](15_REFERENCE_REPOSITORIES.md#r18) |
| AI13 | 테스트 생성 | 오류/성공 Fixture 자동 제안·검토 | M6 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R02](15_REFERENCE_REPOSITORIES.md#r02) |
| AI14 | 병렬 작업 조정 | 객체/파일 충돌·분기 병합·검증 | M9 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R13](15_REFERENCE_REPOSITORIES.md#r13) |

## HU · 사람용 편집기 특화

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| HU01 | 프로젝트 대시보드 | 모드 상태·버전·오류·대상 시리즈 탐색 | M7 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| HU02 | 코드 편집 도우미 | 자동완성·정의 이동·Rename·hover | M7 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R05](15_REFERENCE_REPOSITORIES.md#r05) |
| HU03 | 국가 폼 편집기 | 국가·영토·시작 속성 시각 입력 | M7 | [R09](15_REFERENCE_REPOSITORIES.md#r09), [R10](15_REFERENCE_REPOSITORIES.md#r10) |
| HU04 | 지도 편집기 | 프로빈스 선택·주 변경·영토 미리보기 | M8 | [R07](15_REFERENCE_REPOSITORIES.md#r07), [R08](15_REFERENCE_REPOSITORIES.md#r08) |
| HU05 | POP 표·필터 | 문화·종교·직업별 인구 데이터 편집 | M7 | [R09](15_REFERENCE_REPOSITORIES.md#r09), [R07](15_REFERENCE_REPOSITORIES.md#r07) |
| HU06 | 경제 편집기 | 입출력 그래프·상품/건물 폼 | M7 | [R10](15_REFERENCE_REPOSITORIES.md#r10), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| HU07 | 기술트리 그래프 | 선행/후속 노드와 편집 | M8 | [R10](15_REFERENCE_REPOSITORIES.md#r10), [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| HU08 | 정치·외교 GUI | 법률·이해집단·관계 정의 탐색·변경 | M8 | [R10](15_REFERENCE_REPOSITORIES.md#r10), [R12](15_REFERENCE_REPOSITORIES.md#r12) |
| HU09 | 이벤트 그래프 UI | 분기·호출·결과 시각 편집 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| HU10 | 역사 연표 UI | 사료·실제 사건·가상 분기 표시 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R16](15_REFERENCE_REPOSITORIES.md#r16) |
| HU11 | 게임 GUI 비주얼 편집 | 위젯 트리·캔버스·미리보기 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| HU12 | 국기·에셋 편집 | 문장·색·아이콘 제작 | M8 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R15](15_REFERENCE_REPOSITORIES.md#r15) |
| HU13 | 로컬라이제이션 편집 | 언어별 표·키·번역 오류 | M7 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| HU14 | 오류 대시보드 | 진단 탐색·원인·수정 제안 | M7 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| HU15 | 변경 검토·승인 UI | 소스/의미 Diff·승인/취소 | M7 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R07](15_REFERENCE_REPOSITORIES.md#r07) |
| HU16 | 밸런스 대시보드 | 성능·경제·AI·세이브 지표 | M10 | [R14](15_REFERENCE_REPOSITORIES.md#r14), [R17](15_REFERENCE_REPOSITORIES.md#r17) |

## X · 횡단 기반 원칙

| ID | 기능 | 기능 계약/목적 | 최초 목표 | 참고 레포 |
|---|---|---|---|---|
| X01 | 데이터 소유권 정의 | 엔티티/파일별 authoritative source 단일화 | M0 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| X02 | 원본 보존 구조적 편집 | 주석·공백·순서·중복 키 손실 방지 | M4 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| X03 | 의미 Diff | 객체/속성 관점 변경 전후 비교 | M4 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R05](15_REFERENCE_REPOSITORIES.md#r05) |
| X04 | 버전별 Schema | 게임 버전·DLC·도구 버전별 진단 규칙 | M0 | [R04](15_REFERENCE_REPOSITORIES.md#r04), [R16](15_REFERENCE_REPOSITORIES.md#r16) |
| X05 | Provenance | 최종 적용 정의의 원본·모드·파일 범위 | M2 | [R03](15_REFERENCE_REPOSITORIES.md#r03) |
| X06 | 트랜잭션 | 다중 파일 staging·검증·commit·rollback | M4 | [R06](15_REFERENCE_REPOSITORIES.md#r06), [R08](15_REFERENCE_REPOSITORIES.md#r08) |
| X07 | 동시수정 통제 | Codex/사람의 revision token·낙관적 충돌 검사 | M4 | [R06](15_REFERENCE_REPOSITORIES.md#r06) |
| X08 | Generated/Manual Ownership | 생성 산출물과 수동 편집 원본의 관리 구분 | M0 | [R11](15_REFERENCE_REPOSITORIES.md#r11), [R13](15_REFERENCE_REPOSITORIES.md#r13) |
| X09 | 재현 가능한 빌드 | 도구·게임버전·원본 해시 고정 및 동일 산출물 | M6 | [R13](15_REFERENCE_REPOSITORIES.md#r13), [R16](15_REFERENCE_REPOSITORIES.md#r16) |
| X10 | 미지원·불확실성 보고 | 미지원 스키마, 역사 추정과 엔진 검사 필요 표시 | M3 | [R03](15_REFERENCE_REPOSITORIES.md#r03), [R18](15_REFERENCE_REPOSITORIES.md#r18) |

## 기능 사용 원칙

1. **검증 코어는 하나:** Codex가 호출한 CLI 결과와 사람이 GUI에서 본 검사 결과의 기준이 동일해야 한다.
2. **쓰기 엔진은 하나:** 양쪽의 작업은 같은 EditPlan과 revision/transaction 정책을 통과한다.
3. **모든 데이터가 한 JSON DB에 들어가지는 않음:** 데이터 영역별 단 하나의 authoritative source를 명시하고, 생성 파일은 빌드 산출물로 추적한다.
4. **정적 검증 ≠ 게임 검증:** 구조·참조와 런타임 엔진/AI/GUI를 구분한다.
5. **참고 코드는 의존성이 아님:** Tiger/Jomini/pdx-parser-go/CWTools/Toolkit 등은 연구용이며 사용/재배포는 별도 결정.
6. **사료 검증은 구조 검사와 별개:** 역사적 신뢰도·불확실성은 고고학/문헌 근거를 동반하고, 자동 검사만으로 사실을 확정하지 않는다.

## 카탈로그 검수 규칙

새 항목은 ID·설명·우선 마일스톤·직접적 혹은 비교 가능한 참고 출처·검증 방법을 필수로 갖는다. 삭제/통합시 기존 ID를 조용히 재사용하지 않고 변경 기록에 남긴다.
