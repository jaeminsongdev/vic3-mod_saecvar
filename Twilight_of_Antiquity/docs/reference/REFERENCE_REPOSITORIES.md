# 15. 기능 구현별 참고 저장소 — 검증 목록

- 작성 기준: **2026-10-16**, 공개 GitHub README/문서 중심 1차 확인.
- 사용 정책: **참고 연구 전용**. Tiger를 비롯한 타 도구의 실행·라이브러리 의존성을 SAECVAR Analyzer에 추가하지 않음.
- 우선도는 SAECVAR 개발에 대한 추천이며 외부 프로젝트의 품질 점수가 아님.
- 상세 ID 매핑: [14 기능 카탈로그](TOOLCHAIN_FUNCTION_CATALOG.md).
- 기능·라이선스·지원 버전은 시간이 지나면 바뀔 수 있으므로 실제 구현 착수 시 저장소 **정확한 commit SHA, LICENSE, 지원 버전**을 다시 기록한다.

## 1. 핵심 결론

**처음 조사할 5개 계열:** (1) PDX 파서: **R02 Jomini + R03 pdx-parser-go**, (2) 오버레이/경제 객체: **R03**, (3) 타입/Scope: **R04 CWTools Vic3 Config + R05**, (4) 언어 서버/사람용 UI: **R06**, (5) 버전 차이: **R16**. Tiger(R01)는 오류 규칙 비교용에 한정한다.

**SAECVAR 자체 구현 범위는 참고 저장소의 기능 합집합보다 넓다.** 안전한 구조적 쓰기, 트랜잭션, 사람/Codex 작업 충돌 제어, 역사 사료 신뢰도, 정확한 빌드 재현성 등은 자체 설계가 필요하다.

## 2. 참고 레포 실사표

| ID | 저장소 | 실제로 참고할 기술·기능 | 적용 단계 | 제약 및 검증 주의 | 확인한 라이선스 |
|---|---|---|---|---|---|
| <a id="r01"></a>**R01** | [Tiger](https://github.com/amtep/tiger) | 스크립트 오류 종류·진단 기준·오탐 사례 | 핵심 참고 | 검사 규칙 연구만; 자체 분석기에 실행/라이브러리 의존 금지 | GPL-3.0-or-later |
| <a id="r02"></a>**R02** | [Jomini](https://github.com/rakaly/jomini) | Rust PDX 텍스트/세이브 파서, 성능·퍼징 테스트 | 핵심 참고 | 문법·성능 참고; 외부 파서 사용 아닌 자체 구현 | MIT |
| <a id="r03"></a>**R03** | [pdx-parser-go](https://github.com/kaiser-chris/pdx-parser-go) | 독립 parser, folders, database overlay, provenance, localisation, Victoria 3 경제 | 핵심 참고 | Victoria 3 경제 타입은 1.13 기준; 구현 규칙을 현재 버전에 재검증 | MIT |
| <a id="r04"></a>**R04** | [CWTools Vic3 Config](https://github.com/cwtools/cwtools-vic3-config) | CWT 스키마·Script Effect/Trigger/Scope 규칙 정의 | 핵심 참고 | 사양 후보; 게임 원본·script_docs와 버전별 비교 | MIT |
| <a id="r05"></a>**R05** | [Paradox Language Support](https://github.com/DragonKnightOfBreeze/Paradox-Language-Support) | JetBrains 언어 기능, 심볼 탐색, 정의/참조, Scope 기반 편집 | 핵심 참고 | 고급 언어 기능 설계 참고; config 품질·최신성이 관건 | 별도 확인 |
| <a id="r06"></a>**R06** | [Paradox Modding Toolkit](https://github.com/JDeffner/paradox-modding-toolkit) | LSP, GUI/이벤트 그래프, 로컬라이제이션, 국기·에셋, 출처 보존 편집 UX | 핵심 참고 | 심층 Vic3 오류 검사는 Tiger 연동; Vic3 GUI 렌더 정확도·버전별 지원 검증 | GPL-3.0-or-later |
| <a id="r07"></a>**R07** | [Vic3 Mapdata Editor](https://github.com/EZappala/Victoria3-map-editor) | 주·프로빈스·자원·도시·POP·건물 시각 편집 | 지도·데이터 | 완전한 지형 재창조 도구로 오인 금지 | 별도 확인 |
| <a id="r08"></a>**R08** | [Victoria 3 Lite Map Editor](https://github.com/STERILITZIA02/Victoria-3-lite-map-editor) | state region 내 province 이동, 합법성 검사, override 출력 | 지도·데이터 | 1.13.7용; provinces.png 새로 그리는 편집기가 아님 | 별도 확인 |
| <a id="r09"></a>**R09** | [Babbage](https://github.com/RuanVanNiekerk/Babbage-Victoria-3-Modding-Tool) | Victoria 3 데이터 편집용 WinForms UI | 지도·데이터 | 지원 버전/기능을 직접 확인하고 폼 패턴만 참고 | 별도 확인 |
| <a id="r10"></a>**R10** | [Victoria 3 Modding Tool](https://github.com/FilipeJInacio/Victoria-3-Modding-Tool) | C# 폼 기반 goods/buildings/cultures/laws 등 콘텐츠 편집 | 에디터 | 현재 버전 지원·.NET 7 노후 의존에 주의 | 별도 확인 |
| <a id="r11"></a>**R11** | [Community State Framework](https://github.com/Victoria-3-Modding-Co-op/Community-State-Framework) | state_regions 대형 파일 분할·충돌 감소 패턴 | 데이터 생성 | 필수 런타임 의존성 아님; 파일 분할·빌드 전략 참고 | 별도 확인 |
| <a id="r12"></a>**R12** | [Community Mod Framework](https://github.com/Victoria-3-Modding-Co-op/Community-Mod-Framework) | 게임 내 GUI/정치·일지 확장/모드 간 호환 API | 게임 내 UI | SAECVAR 자체 편집기 GUI와 게임 내부 GUI를 구분 | 별도 확인 |
| <a id="r13"></a>**R13** | [Victoria 3 Mod Template](https://github.com/Victoria-3-Modding-Co-op/Mod-Template) | 모드 디렉터리, metadata, CI/워크플로 초기 구성 | 프로젝트·빌드 | 샘플 metadata의 supported_game_version은 예전 값이므로 복사 전 업데이트 | 별도 확인 |
| <a id="r14"></a>**R14** | [Performance Measuring](https://github.com/Victoria-3-Modding-Co-op/Performance-Measuring) | 게임 날짜/실시간 시간 CSV 수집, 성능 비교 | 테스트 | 실제 게임을 실행해야 측정 가능; 정적 검사 도구 아님 | 별도 확인 |
| <a id="r15"></a>**R15** | [Graphical Assets](https://github.com/Victoria-3-Modding-Co-op/Graphical-Assets) | 아이콘·국기·그래픽 템플릿 및 제작 레퍼런스 | 그래픽 | 개별 리소스 라이선스 확인 전 복사·재배포 금지 | 에셋별 확인 |
| <a id="r16"></a>**R16** | [Modding Digests](https://github.com/Victoria-3-Modding-Co-op/Modding-Digests) | Victoria 3 패치별 데이터/모딩 문서 변경 자료 | 버전 관리 | 게임 원본/문서를 프로젝트에 통째로 넣지 않음, 버전별 검증에 활용 | 자료별 확인 |
| <a id="r17"></a>**R17** | [Pdx-Unlimiter](https://github.com/crschnick/pdx_unlimiter) | Vic3 세이브 탐색·관리·편집과 비교 워크플로 | 테스트 | 실제 저장 파일이 필요하며 엔진 테스트를 대체하지 않음 | 별도 확인 |
| <a id="r18"></a>**R18** | [Paradox AI Modding](https://github.com/JDeffner/paradox-ai-modding) | Codex/Claude Code 자료 조회→정적 검사→실행 증거 구분 워크플로 | Codex 운영 | Tiger 사용 예제는 독립 Analyzer 방침상 대체; 스킬 문구/코드 복제 금지 | GPL-3.0 |

## 3. 주요 기능별 구현 참고 순서

| 구현 시점 | 연구할 레포 | 연구 대상 | 그대로 가져오지 않을 것 |
|---|---|---|---|
| M0 프로젝트·게임버전 | R13 → R16 → R04 | metadata·파일 구조·패치별 규칙·스키마 | 오래된 `supported_game_version` 예제 |
| M1 Lexer/Parser | R02 → R03 → R01 | 텍스트/바이너리 구분, 위치, 중복 키, 오류 복구, fuzz | 라이브러리 링크·소스 복사 |
| M2 Mod Overlay | R03 → R11 → R16 | 폴더 로드·INJECT/REPLACE·provenance·state file 분할 | 모든 경로에 단일 덮어쓰기 규칙 적용 |
| M3 Semantic/Validator | R04 → R05 → R01 → R06 | CWT·Scope·참조 검사·진단 표현 | Toolkit의 Tiger 기반 심층 검사를 자체 구현으로 오인 |
| M4 편집 트랜잭션 | R06 → R03 → R08 | 원본 형식 보존·부분 변경·안전한 출력 | 미확인 자동 패치·데이터 덮어쓰기 |
| M5 경제 수직 슬라이스 | R03 → R10 → R09 | 상품·건물·POP·기술 정의 및 폼 | 특정 도구의 오래된 게임 데이터 그대로 |
| M6 데이터/역사/빌드 | R07 → R11 → R16 | 주/POP/자원·파일 분할·패치 추적 | 제3자 프레임워크 필수 의존 |
| M7 Human MVP | R06 → R05 → R10 → R09 | LSP/폼/코드 탐색/오류 목록 | 처음부터 완전한 IDE 제작 |
| M8 지도·이벤트·GUI | R07 → R08 → R06 → R12 → R15 | 주 변경, 지도 검증, 이벤트 그래프, GUI, 국기/에셋 | Lite 편집기를 새 지형 heightmap 도구로 오인 |
| M9 고급 검사/최적화 | R16 → R03 → R14 → R18 | 증분 검사·회귀·런타임 로그 비교 | 실제 게임 없이 AI/경제/군사 정상동작 보장 |
| M10 릴리스 | R13 → R14 → R17 → R15 | 배포 메타데이터·성능·세이브·리소스 | 원본 게임/허가되지 않은 에셋 재배포 |

## 4. 레포별 중요 주의사항

**R03 pdx-parser-go**는 `script`, `folders`, `database`, `localization`, `asset`, `victoria3`로 계층 분리한 사례다. README의 Victoria 3 타입은 **게임 1.13을 기준**으로 작성되었다고 명시되어 있으며, 시리즈 실제 대상 버전과 다르면 타입·파일 종류를 확인해야 한다.

**R06 Paradox Modding Toolkit**은 LSP·이벤트 그래프·GUI·국기 제작 흐름을 참고한다. 하지만 **심층 오류 검사(Tiger)는 통합 실행을 통해 제공**하므로 우리 독립 Analyzer의 대체 구현이 아니다. GUI의 실제 게임별 렌더링 정확성도 문서 버전/지원 범위에 따라 다르므로 'Victoria 3 전체를 pixel perfect'로 확정하지 않는다.

**R08 Lite Map Editor**는 특히 **1.13.7 기존 프로빈스의 State Region 소속 변경**을 지원한다. 새 해안선/대륙/heightmap을 제작하는 기능과 혼동하지 않는다.

**R11 Community State Framework**는 `map_data/state_regions`의 대형 파일 분할/충돌 관리 원칙을 참고한다. 이를 런타임 의존성으로 반드시 설치해야 한다는 결정은 없다.

**R13 Mod Template** 예시 메타데이터는 과거 게임 버전 문자열을 포함할 수 있다. 우리 실제 target build와 맞춰야 한다.

**R14 Performance Measuring/R17 Pdx-Unlimiter**는 각각 게임 실행 중 측정/실제 세이브 탐색에 사용하는 참고 자료다. 정적 검사기만으로 대체할 수 없는 영역을 정의하는 데 유용하다.

**R15 Graphical Assets**의 개별 에셋은 라이선스·제작자 조건을 확인해야 한다. 공개 레포라는 이유만으로 사용권이 자동 보장되지는 않는다.

## 5. 저작권·의존성 정책

- **GPL 확인:** R01 Tiger, R06 Paradox Modding Toolkit, R18 Paradox AI Modding. 기능·테스트 접근법을 연구하되 저작권을 가진 코드·문구·구체적인 자산을 그대로 복사/번안하지 않는다. 코드 재사용을 추진하면 해당 라이선스 의무 검토부터 수행한다.
- **MIT 확인:** R02 Jomini, R03 pdx-parser-go, R04 CWTools Vic3 Config. 이번 프로젝트의 **독립 구현 원칙은 동일**하며, MIT라는 이유만으로 코드를 반드시 사용할 필요는 없다.
- **그 외:** 라이선스 미확인 항목은 반드시 개별 LICENSE를 확인한 뒤 코드·데이터·에셋 활용 여부를 결정한다.
- **게임 데이터:** 상용 Victoria 3 본편/DLC의 원본 데이터·리소스를 공개 저장소·배포 zip·공개 CI 아티팩트에 포함하지 않는다.
- **검증 소스:** 프로젝트 개발자는 사용 중인 게임 버전과 원본 `script_docs`·모딩 문서에서 실제 규칙을 확인해야 한다. 외부 프로젝트가 다른 버전일 경우 오탐 가능성이 있다.

## 6. 참고 근거 목록

- **R01**: https://github.com/amtep/tiger — 스크립트 오류 종류·진단 기준·오탐 사례.
- **R02**: https://github.com/rakaly/jomini — Rust PDX 텍스트/세이브 파서, 성능·퍼징 테스트.
- **R03**: https://github.com/kaiser-chris/pdx-parser-go — 독립 parser, folders, database overlay, provenance, localisation, Victoria 3 경제.
- **R04**: https://github.com/cwtools/cwtools-vic3-config — CWT 스키마·Script Effect/Trigger/Scope 규칙 정의.
- **R05**: https://github.com/DragonKnightOfBreeze/Paradox-Language-Support — JetBrains 언어 기능, 심볼 탐색, 정의/참조, Scope 기반 편집.
- **R06**: https://github.com/JDeffner/paradox-modding-toolkit — LSP, GUI/이벤트 그래프, 로컬라이제이션, 국기·에셋, 출처 보존 편집 UX.
- **R07**: https://github.com/EZappala/Victoria3-map-editor — 주·프로빈스·자원·도시·POP·건물 시각 편집.
- **R08**: https://github.com/STERILITZIA02/Victoria-3-lite-map-editor — state region 내 province 이동, 합법성 검사, override 출력.
- **R09**: https://github.com/RuanVanNiekerk/Babbage-Victoria-3-Modding-Tool — Victoria 3 데이터 편집용 WinForms UI.
- **R10**: https://github.com/FilipeJInacio/Victoria-3-Modding-Tool — C# 폼 기반 goods/buildings/cultures/laws 등 콘텐츠 편집.
- **R11**: https://github.com/Victoria-3-Modding-Co-op/Community-State-Framework — state_regions 대형 파일 분할·충돌 감소 패턴.
- **R12**: https://github.com/Victoria-3-Modding-Co-op/Community-Mod-Framework — 게임 내 GUI/정치·일지 확장/모드 간 호환 API.
- **R13**: https://github.com/Victoria-3-Modding-Co-op/Mod-Template — 모드 디렉터리, metadata, CI/워크플로 초기 구성.
- **R14**: https://github.com/Victoria-3-Modding-Co-op/Performance-Measuring — 게임 날짜/실시간 시간 CSV 수집, 성능 비교.
- **R15**: https://github.com/Victoria-3-Modding-Co-op/Graphical-Assets — 아이콘·국기·그래픽 템플릿 및 제작 레퍼런스.
- **R16**: https://github.com/Victoria-3-Modding-Co-op/Modding-Digests — Victoria 3 패치별 데이터/모딩 문서 변경 자료.
- **R17**: https://github.com/crschnick/pdx_unlimiter — Vic3 세이브 탐색·관리·편집과 비교 워크플로.
- **R18**: https://github.com/JDeffner/paradox-ai-modding — Codex/Claude Code 자료 조회→정적 검사→실행 증거 구분 워크플로.

> 이 문서는 기능 설계 참고 자료를 기록한다. 정확한 소스 코드 분석, 작동 테스트, 각 레포의 변경 이력·기능 회귀를 모두 수행했다는 뜻은 아니다.
