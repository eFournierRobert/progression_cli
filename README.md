# Progression CLI

***English version below***

Progression CLI est un client Rust pour l'application web [Progression](https://progression.crosemont.qc.ca/) developpé au [Collège Rosemont](https://www.crosemont.qc.ca/) par [Patrick Lafrance](https://git.dti.crosemont.quebec/plafrance).

Le but du client est d'offrir une manière de "clôner" les questions localement et pouvoir accéder aux fonctionnalités de bases.

---

Progression CLI is a Rust client for the [Progression](https://progression.crosemont.qc.ca/) web application developed at [Collège Rosemont](https://www.crosemont.qc.ca/) by [Patrick Lafrance](https://git.dti.crosemont.quebec/plafrance).

The client is made to offer a way to "clone" the questions locally and be able to use the basic functionalities the app offers.

## Fonctionnalités plannifiées / Planned functionalities
- [x] Basic auth with base64
- [x] Question cloning
- [ ] Submit answers
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