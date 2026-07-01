# shirabe

**브라우저 자동화의 재설계 —— CDP로 헤드리스 Chromium 계열 브라우저를 제어하고, ort 스타일의 무설정 백엔드 리졸버를 갖춥니다.**

shirabe는 가벼운 Rust 네이티브 브라우저 자동화 라이브러리이자 디버그 서버입니다.
Chrome DevTools Protocol을 말하는 모든 브라우저(Google Chrome·Chromium·Microsoft Edge)를
하나의 수작성 CDP 엔진으로 구동하며, 작은 HTTP API로 노출합니다. tairitsu 패키저에서
분리하여 독자적으로 다듬은 브라우저 기반입니다.

지침 사상은 [ort](https://crates.io/crates/ort)가 ONNX Runtime에 채택한 것과 같습니다.
**브라우저를 직접 설치할 필요가 없습니다.** 고정된 Chrome for Testing 빌드가 빌드 시
(또는 최초 사용 시) 공유 캐시로 내려받아지고, 투명하게 식별·구동됩니다.

전체 기능과 HTTP API 표는 루트 [README](../../README.md)를 참고하세요.

> 개발 중이며 API는 향후 변경될 수 있습니다.
