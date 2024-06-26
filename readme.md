# SVG Lint - Convertisseur d'icônes SVG pour React

Ce plugin `svg-lint` est conçu pour transformer des fichiers SVG en un format valide pour React, en convertissant les attributs SVG de `kebab-case` en `camelCase` et en optimisant les fichiers pour une meilleure intégration dans les applications React.


## Pourquoi utiliser SVG Lint?

Les icônes SVG sont largement utilisées dans le développement web pour leur qualité et leur scalabilité. Toutefois, l'intégration directe de SVG dans les projets React peut souvent mener à des erreurs dues à des incompatibilités de format d'attributs. SVG Lint résout ce problème en:

- Convertissant automatiquement les attributs de `kebab-case` (typiques des fichiers SVG) en `camelCase` (requis par JSX/React).
- Fournissant un linting pour assurer que les SVG sont prêts à l'emploi dans vos applications React sans modifications manuelles.
- Optimisant les fichiers SVG pour de meilleures performances lors du chargement dans des applications web.

## Installation

Pour installer SVG Lint, vous avez besoin de Node.js et npm. Exécutez la commande suivante pour installer le plugin dans votre projet (en tant que dependance de dev):

```bash
npm install react-svg-lint -D
```


## Configuration

Modifier votre package.json pour ajouter le shemin vers votre fichier d'icone *(.tsx, .ts, .js, .jsx, .svg)
```bash
{
  "scripts": {
    "svg-lint": "svg-lint path/to/your-file.tsx",
  },
```

## Utilisation
Vous pouvez tout simplement executer la commande suivant et cela linteras votre fichier

```bash
npm run svg-lint
```

### Petite considération
Pour le moment le plugin n'est disponible que pour windows, elle seras disponible pour les autres plateformes dans sa prochaine MAJ.
Et ne vous inquitez pas au cas ou vous envoyer votre code en prod sur une autre plateforme puisque elle n'est pas derstiner pour la prod


## TODO:
- Automatiser le build pour ios et linux