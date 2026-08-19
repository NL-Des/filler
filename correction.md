# Correction du parsing d'entrée

Ce document liste ce qu'il faut corriger dans le code de lecture/validation d'entrée pour qu'il colle au vrai protocole du moteur de jeu Filler. Pas de code Rust prêt à copier : juste le problème, pourquoi il en est un, et l'approche à suivre.

## 1. Le vrai protocole (rappel)

Le moteur de jeu (confirmé en extrayant les chaînes du binaire `linux_game_engine`) communique via ces formats :

```
$$$ exec p1 : [%s]      <- une fois par joueur, au lancement, avant le premier tour
$$$ exec p2 : [%s]
Anfield %d %d:           <- à chaque tour : largeur puis hauteur du plateau
Piece %d %d:              <- à chaque tour : largeur puis hauteur de la pièce
-> Answer (%c): %s        <- ce que VOTRE programme doit répondre sur stdout
```

Concrètement, à chaque tour, votre programme reçoit sur stdin :

```
Anfield <w> <h>:
    <ligne d'index de colonnes>
000 <ligne de grille>
001 <ligne de grille>
...
<h-1 avec le bon padding> <ligne de grille>
Piece <pw> <ph>:
<ligne de motif>
...
```

Et c'est un **dialogue**, pas un flux à lire d'un coup : le moteur envoie un tour, attend votre réponse (coordonnées) sur stdout, puis envoie le tour suivant — jusqu'à la fin de la partie (fermeture de stdin) ou un timeout si vous ne répondez pas assez vite.

Deux points structurants à retenir :
- **La hauteur des blocs change à chaque tour** (surtout celle de la pièce), elle est donnée par les en-têtes `Anfield %d %d:` et `Piece %d %d:`.
- **La ligne `$$$ exec pN : [...]` n'apparaît qu'une seule fois**, avant le tout premier tour de ce joueur — pas à répéter à chaque tour.

## 2. Problème — lecture bloquante (`src/input/input.rs`)

```rust
pub fn read_input_from<R: BufRead>(data_input: R) -> Vec<String> {
    data_input
        .lines()
        .map(|line| line.expect("..."))
        .collect()
}
```

`.lines().collect()` consomme le `BufRead` **jusqu'à EOF** avant de retourner quoi que ce soit. Sur un flux interactif comme celui du moteur de jeu, EOF n'arrive qu'à la fin de la partie (quand le moteur ferme votre stdin). Résultat : votre programme n'aura jamais l'occasion de répondre au premier tour, puisqu'il est bloqué à essayer de tout lire d'abord. Le moteur, lui, attend une réponse et finira par afficher un timeout.

**Ce qu'il faut faire** : remplacer la lecture "tout d'un coup" par une lecture **ligne par ligne**, pilotée depuis l'extérieur (typiquement depuis la boucle de jeu dans `main.rs`). Il faut une fonction qui lit *une* ligne à la fois sur un `BufRead` et retourne `None` quand le flux est fermé (EOF), pour pouvoir écrire une boucle du style "tant qu'il y a un tour à lire, le traiter". Gardez le même principe de séparation que vous avez déjà (une fonction générique testable avec un `Cursor`, une fonction publique qui utilise `stdin()`), juste appliqué à une ligne au lieu de tout le flux.

## 3. Problème — découpage à index fixe (`src/input/input_data_formatting.rs`)

```rust
pub fn format_input(data_input: Vec<String>) -> (String, String, String, String, String) {
    let player = data_input[0].clone();
    let anfield_coordinates = data_input[1].clone();
    let anfield_grid = data_input[2].clone();
    let piece_numbers = data_input[3].clone();
    let piece_grid = data_input[4].clone();
    ...
}
```

Cette fonction suppose que l'entrée tient toujours en exactement 5 lignes, avec la grille du plateau et la grille de la pièce chacune sur *une seule* ligne. Ce n'est pas le cas : la grille du plateau fait `hauteur` lignes (+ 1 pour l'en-tête de colonnes), et la grille de la pièce fait `hauteur_pièce` lignes — ces hauteurs sont données dynamiquement par les en-têtes `Anfield %d %d:` et `Piece %d %d:`, et changent d'un tour à l'autre.

**Ce qu'il faut faire**, une fois que vous avez une fonction de lecture ligne par ligne (point 2) :
1. Lire une ligne, la parser comme `Anfield <w> <h>:` pour en extraire `w` et `h` (ce sont des entiers séparés par des espaces dans la chaîne).
2. Lire et ignorer la ligne suivante (l'en-tête des indices de colonnes, ex. `    01234...`).
3. Lire `h` lignes : ce sont les lignes de la grille du plateau (chacune préfixée par un index à 3 chiffres + espace — à garder tel quel ou à nettoyer selon ce que vous voulez faire ensuite).
4. Lire une ligne, la parser comme `Piece <pw> <ph>:` pour en extraire `pw` et `ph`.
5. Lire `ph` lignes : ce sont les lignes du motif de la pièce.

Plutôt que de retourner un tuple de `String`, envisagez une structure qui porte ces champs explicitement (largeur/hauteur du plateau, les lignes de grille, largeur/hauteur de la pièce, les lignes de motif) — ce sera plus clair pour la suite (validation, puis logique de placement) qu'un tuple de 5 `String`.

## 4. Problème — caractère de pièce invalide (`src/action_and_validation/data_validation.rs`)

```rust
if !piece_grid.chars().all(|c| matches!(c, '.' | '#')) {
    return (false, String::from("piece grid contains invalid characters."));
}
```

Regardez les deux exemples d'énoncé que vous avez : la pièce est toujours représentée avec `.` et `O` (la lettre O majuscule), jamais `#` :

```
Piece 4 1:
.OO.
```
```
Piece 7 2:
OOOO...
OOO....
```

Avec le code actuel, **toute pièce réelle sera rejetée comme invalide**, puisque `O` ne fait pas partie des caractères acceptés.

**Ce qu'il faut faire** : changer `matches!(c, '.' | '#')` en `matches!(c, '.' | 'O')` (et penser à mettre à jour les tests unitaires qui utilisent `'#'` ou `'x'` comme caractère invalide/valide pour rester cohérents).

Pendant que vous êtes dans ce fichier : la validation de `anfield_grid` avec `matches!(c, '.' | '$' | 's' | '@' | 'a')` est correcte et n'a pas besoin d'être changée (`@`/`a` = joueur 1, ancien/nouveau ; `$`/`s` = joueur 2, ancien/nouveau).

## 5. Problème — la ligne `$$$ exec`

Elle n'est envoyée qu'une seule fois par joueur, juste avant le tout premier tour de ce joueur — ce n'est pas un élément qui se répète dans chaque bloc `Anfield`/`Piece`. Si vous essayez de la lire à chaque itération de votre boucle de jeu (en pensant qu'elle fait partie du format répété, comme le laissait penser le premier exemple de l'énoncé), votre parsing des tours suivants sera décalé.

**Ce qu'il faut faire** : la lire et l'ignorer (ou l'utiliser pour savoir si vous êtes p1 ou p2, via le `%s` qui contient le chemin de votre propre exécutable) **une fois, avant d'entrer dans la boucle principale**, pas à l'intérieur.

## 6. Boucle principale (`src/main.rs`)

```rust
fn main() {
    let data_input = read_input();
    let (player, anfield_coordinates, anfield_grid, piece_numbers, piece_grid) = format_input(data_input);
    let (is_input_ok, error_message) = validate_input(...);
    if is_input_ok == false {
        println!("{}", error_message);
        return;
    }
}
```

Ceci ne traite qu'un seul bloc puis s'arrête. Il faut une boucle qui, à chaque itération :
1. tente de lire un tour (voir point 3) ;
2. si la lecture indique EOF (plus rien à lire), sort de la boucle proprement (fin de partie) ;
3. sinon, valide le tour lu, calcule une réponse, et l'imprime sur stdout (avec un flush, car le moteur attend la réponse avant d'envoyer le tour suivant — pensez à `stdout().flush()` si vous utilisez un writer bufferisé, `println!` flush généralement automatiquement mais soyez vigilant si vous changez de mécanisme d'écriture).

## 7. Vérification

- `cargo test` : ajoutez/adaptez des tests qui simulent, via un `Cursor`, un flux contenant **plusieurs tours consécutifs avec des hauteurs de pièce différentes**, pour vérifier que votre parsing lit bien le bon nombre de lignes à chaque fois sans se désynchroniser.
- Test réel avec le moteur : `docker run -v "$(pwd)/solution":/filler/solution -it filler` puis, dans le conteneur, `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 solution/<votre_binaire>` pour vérifier qu'il n'y a plus de timeout et que le format est bien accepté.
