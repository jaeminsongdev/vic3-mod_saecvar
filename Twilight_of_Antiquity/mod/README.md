# mod — Victoria 3 게임 모드 루트

게임이 실제로 읽는 경로만 유지하는 디렉터리입니다. **지금은 콘텐츠가 없는 스캐폴딩 상태**이며, 런처 인식/서기 600년 시작 테스트도 완료하지 않았습니다.

실행 단계에서 필요한 `.metadata/metadata.json`과 `common/`, `events/`, `localization/`, `map_data/`, `gfx/`, `gui/` 등은 실제 대상 게임 버전을 확인하고 **사용하는 폴더만** 만듭니다. Victoria 3 모드 폴더는 게임 `game/` 아래의 상대 경로를 미러링해야 합니다. [Paradox 모딩 안내](https://www.paradoxinteractive.com/games/victoria-3/news/victoria-3-dev-diary-60-modding).

`data/`에서 생성한 PDX라면 생성 원본·해시를 추적하고, 직접 편집하는 PDX라면 소스 보존 편집 정책을 적용합니다.

**패키징:** 이 README는 개발 전용이므로 배포/런처로 복사하지 않습니다. 원본 Victoria 3 및 DLC 파일도 저장소/패키지에 복사하지 않습니다. [폴더 구조 상세](../docs/architecture/REPOSITORY_LAYOUT.md).
