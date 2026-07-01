# shirabe

**L'automatisation de navigateur repensée — contrôle des navigateurs Chromium (headless) via CDP, avec un résolveur de backend sans configuration façon ort.**

shirabe est une bibliothèque d'automatisation de navigateur légère et native
Rust, ainsi qu'un serveur de débogage. Il pilote tout navigateur parlant le
Chrome DevTools Protocol — Google Chrome, Chromium, Microsoft Edge — au travers
d'un seul moteur CDP écrit à la main, et expose le tout via une petite API HTTP.
C'est le socle navigateur extrait du empaqueteur tairitsu, durci pour se tenir
seul.

La philosophie est la même que celle d'[ort](https://crates.io/crates/ort) pour
ONNX Runtime : **vous ne devriez jamais avoir à installer un navigateur à la
main.** Une version épinglée de Chrome for Testing est récupérée dans un cache
partagé (à la compilation ou à la première utilisation), localisée
transparentemente et pilotée via CDP.

Pour la liste complète des fonctionnalités et de l'API HTTP, voir le
[README](../../README.md) racine.

> En cours de développement ; l'API peut changer à l'avenir.
