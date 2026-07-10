# PassKeeper

## 기능

패스워드를 관리하기 위한 윈도우용 프로그램.   

## 설치

프로젝트의 루트 디렉토리에서 'pnpm install'을 이용하여 최초 한번은 필요한 모듈을 설치해야 합니다.

## 배포 폴더 구성

📂root   
├─📂data   
│   └─📄user.dat   
└─📄passkeeper.exe   

## 개발 도구 버전

* RUST 버전 : v.1.96.0 (ac68faa20 2026-05-25)
* TAURI 버전 : v.2.11.5
* TAURI-API 버전 : v.2.11.1
* TAURI-CLI 버전 : v.2.11.4

## 버전정보

### v.1.0.0

* 최초 버전

## 화면

* 로그인 화면 :   
<img src="doc/login.png" width="400" height="300">   
* 메인 화면 :   
<img src="doc/main_view.png" width="400" height="300">   

## 개발 주요 사항

### crypto
* encrypt_data, decrypt_data, hash_data 함수를 재정의 필요.   
> 암호화와 Hash 부분은 개인적으로 추가하여 개발하여야 함. (기본은 암호화 되지 않은 상태로 저장이 됨)   
 (* 위치 : src-tauri > src > module > crypto.rs )   

### bundle.js 번들링

* bundle.js의 상단에 주석을 참고하여 번들링 필요.

### tauri.conf.json

* tauri.conf.json : 스키마 파일을 로컬에 저장하여 참조함. (src-tauri/schema/config.schema.json)
* 로컬 참조시 타우리 버전 업데이트 후 문제가 발생되면, [config.schema.json](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-schema-generator/schemas/config.schema.json)의 파일을 다운받아 사용 해야함.