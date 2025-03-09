# Progression CLI

***English version below***

Progression CLI est un client Rust pour l'application web [Progression](https://progression.crosemont.qc.ca/) developpé au [Collège Rosemont](https://www.crosemont.qc.ca/) par [Patrick Lafrance](https://git.dti.crosemont.quebec/plafrance).

Le but du client est d'offrir une manière de "clôner" les questions localement et pouvoir accéder aux fonctionnalités de bases.

### Comment l'application fonctionne

#### Clôner

Pour clôner une question, vous pouvez faire: 
```bash
progression_cli -c https://progression.crosemont.qc.ca/question?uri=aHR0cHM6Ly9wcm9ncmVzc2lvbi5wYWdlcy5kdGkuY3Jvc2Vtb250LnF1ZWJlYy9jb250ZW51L2RlbW8vZGExNDNjMTEtZjc2My00MDY0LWJjMDMtNTYxZGNkODFjNmUyL2luZm8ueW1s
```
Cela va créer un nouveau dossier avec les fichiers à l'intérieur. Notez qu'il n'y a pas ```?include...``` à la fin de l'URL.

La commande va clôner l'ébauche de tout les langages par défaut. Vous pouvez ajouter ```--only-lang [langage]``` pour clôner l'ébauche de seulement un langage.

#### Envoyer une réponse

Pour envoyer une réponse, vous pouvez aller dans le dossier de la question et faire:
```bash
progression_cli -s
```

Cela va envoyer la réponse à l'API et le retour sera dans le fichier ```answer.md```. Pour le moment, vous **devez** avoir un ébauche de présent dans le dossier.

---

Progression CLI is a Rust client for the [Progression](https://progression.crosemont.qc.ca/) web application developed at [Collège Rosemont](https://www.crosemont.qc.ca/) by [Patrick Lafrance](https://git.dti.crosemont.quebec/plafrance).

The client is made to offer a way to "clone" the questions locally and be able to use the basic functionalities the app offers.

### How does the application work

#### Cloning

To clone a question, you have to do: 
```bash
progression_cli -c https://progression.crosemont.qc.ca/question?uri=aHR0cHM6Ly9wcm9ncmVzc2lvbi5wYWdlcy5kdGkuY3Jvc2Vtb250LnF1ZWJlYy9jb250ZW51L2RlbW8vZGExNDNjMTEtZjc2My00MDY0LWJjMDMtNTYxZGNkODFjNmUyL2luZm8ueW1s
```
This will create a new directory with all the necessary files inside. Note that there's no ```?include...``` at the end of the URL.

By default, the command will clone the draft for all languages. You can add ```--only-lang [language]``` to clone the draft of only one language.

#### Send an answer

To send an answer, you can go in the question directory and do :
```bash
progression_cli -s
```

This will send your answer to the API the return will be put in ```answer.md```. Right now, you **have** to have only one question draft in the directory for it to work.

## Fonctionnalités planifiées / Planned functionalities
- [x] Basic auth with base64
- [x] Question cloning
- [x] Submit answers
- [ ] Submit tests

## Build and run from source
Build the package:
```bash 
cargo build
```

Build and run the package:
```bash 
cargo run -- [your arguments]
```

Build a release of the package:
```bash
cargo build --release
```