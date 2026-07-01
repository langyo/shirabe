# shirabe

**Automatización de navegadores, reimaginada — control de la familia Chromium (headless) mediante CDP, con un resolutor de backend sin configuración al estilo ort.**

shirabe es una librería ligera y nativa de Rust para automatización de
navegadores, además de un servidor de depuración. Controla cualquier navegador
que hable Chrome DevTools Protocol — Google Chrome, Chromium, Microsoft Edge —
mediante un único motor CDP escrito a mano, y lo expone a través de una pequeña
API HTTP. Es la base de navegador extraída del empaquetador tairitsu,
reforzada para funcionar por sí sola.

La filosofía es la misma que la de [ort](https://crates.io/crates/ort) para
ONNX Runtime: **nunca deberías tener que instalar un navegador a mano.** Una
versión fijada de Chrome for Testing se descarga a una caché compartida (al
compilar o en el primer uso), se localiza de forma transparente y se controla
vía CDP.

Para la lista completa de funciones y de la API HTTP, consulta el
[README](../../README.md) raíz.

> En desarrollo; la API puede cambiar en el futuro.
