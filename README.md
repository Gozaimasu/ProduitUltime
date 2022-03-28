# ProduitUltime

Exercice 1 du concours de programmation de la poste 2020 [ici](https://www.isograd-testingservices.com/FR/solutions-challenges-de-code?cts_id=68&reg_typ_id=2&que_str_id=CTSTFR0297&rtn_pag=https%3A%2F%2Fwww.isograd-testingservices.com%2F%2FFR%2Fsolutions-challenges-de-code%3Fcts_id%3D80#)

## Objectif

C'est décidé, vous allez enfin réaliser votre rêve d'ouvrir une boutique en ligne ! Tout est prêt pour vos futurs clients : vous avez reçu les produits, votre tout nouveau site web est terminé, il n'y a plus qu'à faire le lancement officel.

A peine lancé que ça y est, les premiers visiteurs arrivent ! Les premières recherches sont faites sur votre site, il faut maintenant faire des mises en avant de produits susceptibles de toucher le plus grand nombre de clients.


## Données
### Entrée

Ligne 1 : un entier N compris entre 1 et 40 représentant le nombre d'articles de votre liste
Lignes 2 à N+1 : un entier positif s entre 1 et 100 pour le score de l'article, un espace et une chaîne de caractères pour la référence de l'article (entre 2 et 10 caractères)

On vous garantit que tous les scores sont uniques, et que les références ne peuvent contenir que des lettres et des chiffres.

### Sortie

Sur une ligne, le nom de l'article dont le score est le plus élevé. Attention : la solution attendue est sensible à la casse (majuscules/minuscules)


## Exemple

Pour le catalogue suivant :

4  
1 Table1234  
30 POULE985  
5 ART42  
29 R4

L'article ayant le score le plus élevé est : POULE985.
