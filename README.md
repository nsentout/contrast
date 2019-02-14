# Contrast

[//]: # (Contrast is a Rust crate developed as part of the Bordeaux computer science Master.)

Notre projet est programmé en *Rust*. C'est un langage qui évolue très fréquemment.
Par conséquence, il vous faut la dernière version du compilateur, c'est actuellement la
version **1.32.0**.


### Installation
Nous vous conseillons d'installer [**rustup**](https://rustup.rs/) qui est l'installateur
officiel de la toolchain de *Rust*. Il se chargera de la maintenir à jour.
La commande ```rustup update``` vous le confirmera.

La toolchain comporte entre autre :
* **rustc** (compilateur)
* **cargo** (gestionnaire de paquets)

### Compilation
C'est avec *cargo* que se compile le projet, pour cela rien de plus simple, il suffit de vous
placer dans le répertoire du projet, c'est-à-dire le dossier qui contient le fichier **Cargo.toml**
et d'exécuter les commandes suivantes :
* ```cargo build``` pour compiler.
* ```cargo run``` pour compiler & exécuter l'exemple.
* ```cargo test``` pour compiler & lancer les tests.
* ```cargo doc --open``` pour générer la doc et l'ouvrir à l'index dans votre navigateur.

Certaines opérations peuvent prendre du temps...

### Exemple
Notre projet est le développement d'une bibliothèque mais il y a un programme d'exemple (*main.rs*) qui,
si tout se passe bien, doit afficher 100k petits triangles dont la position et la couleur est générée
aléatoirement.

### Arborescence
Tout le code est dans le dossier *src*, il est divisé en plusieurs fichiers qui sont
des modules *Rust*. Le seul shader actuellement utilisé, celui pour afficher
les points, est dans le dossier *src/shaders*.