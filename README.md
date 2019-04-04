# Contrast

[//]: # (Contrast is a Rust crate developed as part of the Bordeaux computer science Master.)

Notre projet est programmé en *Rust*. C'est un langage qui évolue très fréquemment.
Par conséquent, il vous faut la dernière version du compilateur, c'est actuellement la
version **1.33.0**.


### Installation
Nous vous conseillons d'installer [**rustup**](https://rustup.rs/) qui est l'installateur
officiel de la toolchain de *Rust*. Il se chargera de la maintenir à jour.
La commande ```rustup update``` vous le confirmera.

La toolchain comporte entre autre :
* **rustc** (compilateur)
* **cargo** (gestionnaire de paquets)

> Attention : sous Windows, vous devrez probablement installer FreeType manuellement.

### Compilation
C'est avec *cargo* que se compile le projet. Pour cela rien de plus simple, il suffit de vous
placer dans le répertoire d'un exemple, c'est-à-dire un dossier qui contient un fichier **Cargo.toml**
et d'exécuter les commandes suivantes :
* ```cargo build``` pour compiler.
* ```cargo run``` pour compiler & exécuter l'exemple.
* ```cargo run --release``` pour compiler avec optimisations & exécuter l'exemple.
* ```cargo test``` pour compiler & lancer les tests.
* ```cargo doc --open``` pour générer la doc et l'ouvrir à l'index dans votre navigateur.

Certaines opérations peuvent prendre du temps...
Pour la documentation il est plus pertinent de la générer depuis le dossier **contrast-renderer**, vous aurez ainsi par héritage l'ensemble de la documentation du projet.
> Attention : ne lancez pas **cargo run** depuis la racine de **contrast** et **contrast-renderer** car il ne dispose pas d'exécutable.

### Arborescence

Le projet est découpé en deux crates principales :
* *contrast* : la couche bas niveau qui représente le cœur de notre implémentation.
* *contrast-renderer* : la couche haut niveau qui dépend de *contrast* et *luminance* en charge du rendu.

L'ensemble des programmes de démonstration de *contrast* sont présents dans le dossier **contrast-renderer/examples**.

### Hello Triangle

```rust
use contrast_renderer::LumiRenderer;
use contrast::marks::pointmark::Shape;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn main()
{
    // Initialize the renderer, opening a window.
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Hello, world!");

    // Initialize contrast, allowing us to handle the marks.
    let contrast = renderer.get_contrast_mut();

    // Add a mark into contrast
    contrast.add_point_mark().set_position((400.0, 400.0, 0.0))
            .set_size((300.0, 300.0))
            .set_color((1.0, 0.0, 0.0, 1.0))
            .set_shape(Shape::Triangle);

    // Indicate to contrast that a new mark was added and that it needs to refresh.
    contrast.mark_dirty_all();

    // Run the renderer, which makes it display the marks and listen to devices events.
    renderer.run();
}
```
