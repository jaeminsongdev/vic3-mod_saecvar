# mod — Victoria 3 게임 모드 루트

게임이 실제로 읽는 경로를 유지하는 디렉터리입니다. **현재는 G0 날짜 실험 모드**이며, 1836년 세계 데이터에 시작일 `600.1.1`, 종료일 `720.1.1`만 적용합니다. 역사적 600년 세계는 아직 구현하지 않았습니다.

실제 파일은 `.metadata/metadata.json`과 `common/defines/zz_saecvar_g0_dates.txt`입니다. 대상은 1.13.11입니다. 런처 인식, 사용자에 의한 600년 진입·진행·저장/로드 및 별도 경계 모드의 720년 종료·계속 플레이를 확인했습니다. G0는 최소 날짜 실증이며 120년 연속 진행이나 전체 엔진 호환성 완료는 아닙니다. 상세 판정은 [실험 결과](../docs/quality/M0_G0_REPORT.md)를 기준으로 판단합니다.

Victoria 3 모드는 게임 `game/` 아래의 상대 경로를 미러링합니다. [Paradox 모딩 안내](https://www.paradoxinteractive.com/games/victoria-3/news/victoria-3-dev-diary-60-modding).

법률·이벤트의 실험적 연도 보정은 이 소스 모드에 포함하지 않습니다. `tests/runtime/g0.py --mode compat`으로 로컬에서만 생성하며, 원본 게임 파일은 수정하지 않습니다. 종료 경계 실험도 별도 프로필로만 생성합니다.

`data/`에서 생성한 PDX라면 생성 원본·해시를 추적하고, 직접 편집하는 PDX라면 소스 보존 편집 정책을 적용합니다.

**패키징:** 이 README는 개발 전용이므로 배포/런처로 복사하지 않습니다. 원본 Victoria 3 및 DLC 파일도 저장소/패키지에 복사하지 않습니다. [폴더 구조 상세](../docs/architecture/REPOSITORY_LAYOUT.md).
