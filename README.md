# Contrast

[//]: # (Contrast is a Rust crate developed as part of the Bordeaux computer science Master.)

Notre projet est programmé en *Rust*. C'est un langage qui évolue très fréquemment.
Par conséquent, il vous faut la dernière version du compilateur, c'est actuellement la
version **1.32.0**.


### Installation
Nous vous conseillons d'installer [**rustup**](https://rustup.rs/) qui est l'installateur
officiel de la toolchain de *Rust*. Il se chargera de la maintenir à jour.
La commande ```rustup update``` vous le confirmera.

La toolchain comporte entre autre :
* **rustc** (compilateur)
* **cargo** (gestionnaire de paquets)

### Compilation
C'est avec *cargo* que se compile le projet. Pour cela rien de plus simple, il suffit de vous
placer dans le répertoire du projet, c'est-à-dire le dossier qui contient le fichier **Cargo.toml**
et d'exécuter les commandes suivantes :
* ```cargo build``` pour compiler.
* ```cargo run``` pour compiler & exécuter l'exemple.
* ```cargo run --release``` pour compiler avec optimisations & exécuter l'exemple.
* ```cargo test``` pour compiler & lancer les tests.
* ```cargo doc --open``` pour générer la doc et l'ouvrir à l'index dans votre navigateur.

Certaines opérations peuvent prendre du temps...

### Exemple
Notre projet est le développement d'une bibliothèque mais il y a un programme d'exemple (*main.rs*).
Si vous l'exécutez, vous devriez voir une fenêtre s'ouvrir contenant 100 000 petits triangles dont 
la position et la couleur ont été générées aléatoirement.

### Arborescence
Tout le code, à l'exception de la macro procédurale, est dans le dossier *src*. Il est divisé en 
plusieurs fichiers qui sont des modules *Rust*. Le seul shader actuellement utilisé, celui pour afficher
les marques de type point, est dans le dossier *src/shaders*.

La macro procédurale a pour but de générer automatiquement du code à la compilation.
Elle nous permet d'éviter de dupliquer le code commun aux marques. Vous pourrez la trouver dans
le dossier *mark_macro_derive*.

Concernant la contenu des fichiers, *pointmark.rs*, *linemark.rs* et *polygonmark.rs* sont les fichiers
contenant la structure de la marque correspondant au nom du fichier, ainsi que son implémentation.
*properties.rs* contient les structures communes utilisées par toutes les marques.
*lib.rs* permet de lier les différents modules entre eux et *camera.rs* contient le code de prototype de caméra.

Enfin, *markscontainer.rs* est le fichier contenant la structure principale de la bibliothèque, ainsi que son
implémentation et des tests appropriés.