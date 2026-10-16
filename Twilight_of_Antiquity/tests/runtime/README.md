# M0/G0 실행 절차

Python 표준 라이브러리와 Rust stable만 사용한다. 저장소 루트에서 아래 명령을 실행한다. 설치 폴더나 사용자 프로필 전체를 복사하지 않는다.

## 자동 검사

```powershell
cargo fmt --all --check --manifest-path Twilight_of_Antiquity/toolchain/Cargo.toml
cargo test --workspace -j 1 --manifest-path Twilight_of_Antiquity/toolchain/Cargo.toml
python -m unittest discover -s Twilight_of_Antiquity/tests/runtime -v
python Twilight_of_Antiquity/tests/integration/check_m0.py
```

경로 소유권·실험 입력·파일 계약 검사이며 실제 엔진 검증은 아니다. 인공 오류는 Rust 임시 디렉터리 fixture와 `test_g0.py`에서 만든다. 게임 파일을 공개 fixture에 사용하지 않는다.

## 실험 프로필 생성·실행

`<install>`은 `binaries/`, `game/`, `launcher/`를 포함한 설치 루트다. 실제 경로는 로컬 명령 인자로만 전달한다. 각 실험 ID는 새 이름을 사용한다. 동일 ID 재생성·재실행은 기존 증거 보존을 위해 거부한다.

```powershell
python Twilight_of_Antiquity/tests/runtime/g0.py stage --install '<install>' --mode date --id date-example
python Twilight_of_Antiquity/tests/runtime/g0.py launch --install '<install>' --id date-example
python Twilight_of_Antiquity/tests/runtime/g0.py collect --id date-example
```

- `baseline`: 원작 1836~1936, 모드 없이 비교.
- `date`: 원작 세계에 600~720 날짜만 변경.
- `compat`: 날짜 변경 + 통치 법률의 시작일 비교 6곳 + 프랑스 왕위 이벤트 임계연도 2곳을 로컬 파생 파일에서만 보정. 실제 엔진 성공은 미검증이다. 초기 조약 문제 등 모든 시대 충돌을 해결하는 패치가 아니다.
- `boundary`: 원작 세계에 시작일 `719.12.30`, 종료일 `720.1.1`. 며칠 진행으로 종료/계속 플레이를 관찰한다. 600년 세이브나 120년 장기 진행 검증을 대체하지 않는다.

실험 폴더는 `<series>/.local/g0/<id>/`다. `environment.local.json`은 설치 경로·버전·원본 해시·설치 DLC·날짜 참조 후보를, `run.local.json`은 프로필·모드 입력 해시를, `launch.local.json`은 PID·명령을 기록한다. 로그·세이브·개인 경로는 Git에 추가하지 않는다.

`launch`는 `-debug_mode -gdpr-compliant -userdir=<isolated-profile>`로 실제 GUI 엔진을 실행한다. 실행 중인 Victoria 3가 있으면 거부한다. Steam을 먼저 정상 실행한다. 자식 프로세스에 게임 App ID를 지정하지만, 이것만으로 DLC 소유권·활성화를 확정하지 않는다. 별도 프로필의 로그가 실제로 생성됐는지 확인한다. 공식 headless 검사 기능을 확인한 것으로 취급하지 않는다.

메모리 여유가 부족하면 새 게임을 추가 실행하지 않는다. 필요하면 현재 실험을 게임 메뉴에서 종료하고 증거를 수집한다. 타 앱·사용자 게임을 자동 종료하거나 시스템 캐시를 변경하지 않는다.

## 사용자가 런처에서 실행한 세션 관찰

```powershell
python Twilight_of_Antiquity/tests/runtime/g0.py observe --install '<install>' --profile '<user-profile>' --id user-example
python Twilight_of_Antiquity/tests/runtime/g0.py collect --id user-example
```

프로필 설정·플레이셋·세이브를 변경하지 않고 로그 스냅샷, 실제 로드된 모드의 메타데이터·날짜 파일 해시, 세이브 파일 해시 목록을 로컬 증거에 남긴다. `collect`는 매번 다른 시각의 폴더를 생성한다. 사용자가 다른 세션을 열면 기존 로그가 바뀔 수 있으므로 실험 직후 수집한다. `observe`는 다른 게임 프로세스를 실행하지 않는다.

## 런처 인식과 되돌림

공식 구조는 `mod/.metadata/metadata.json`과 게임 상대 경로를 미러링하는 파일이다. 로컬 등록 시 사용자 Victoria 3 프로필의 `mod/saecvar_g0`를 이 저장소의 `mod/`에 연결했다. 런처가 해당 폴더의 메타데이터를 읽는 것을 확인했다. 등록 경로는 `.local/` 증거와 이 작업의 실행 기록에만 둔다.

현재 모드 소스에는 법률·이벤트 실험 보정이 없다. 실험 종료 후 런처에서 이 모드를 비활성화하면 기본 게임으로 돌아갈 수 있다. 다른 모드/세이브를 삭제하지 않는다. 링크를 정리할 경우 대상이 여전히 이 프로젝트의 junction인지 먼저 확인하고 링크 자체만 제거해야 한다.

## 수동 엔진 체크리스트

1. 런처의 SAECVAR 이름·대상 버전·단독 활성화를 확인한다.
2. 새 게임 날짜 `600.1.1`, 국가 선택 후 실제 플레이 진입을 확인한다.
3. 며칠 진행하고 `saecvar_g0_600_smoke`라는 새 이름으로 수동 저장한다.
4. 같은 세이브를 불러와 날짜 유지와 추가 진행을 확인한다.
5. 별도 `boundary` 실험에서 `720.1.1` 도달 및 종료/계속 플레이를 기록한다. 실제 실증은 기존 한국어 프로필의 런처에서 `SAECVAR G0 - 720년 종료 검증` 모드만 켜서 완료했다. 격리 프로필은 언어 설정이 없으면 영어로 시작할 수 있으므로 현재는 사용자가 직접 기존 한국어 런처로 실행하는 방식을 우선한다.
6. 각 실험의 오류를 baseline과 비교한다. 로그 존재나 프로그램 실행만으로 PASS를 쓰지 않는다.

관찰한 날짜·선택 국가·활성 DLC·종료 방식·증거 위치를 [보고서](../../docs/quality/M0_G0_REPORT.md)에 남긴다.
