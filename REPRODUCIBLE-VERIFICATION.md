# REPRODUCIBLE-VERIFICATION.md — Procédure de Vérification

**Guide pas-à-pas pour vérifier l'authenticité et l'intégrité de ce dépôt.**

---

## 1. VÉRIFICATION DE L'INTÉGRITÉ DES FICHIERS

### Étape 1 : Cloner le dépôt

```bash
git clone https://github.com/Hammanh/oXc-anteriorite-2025.git
cd oXc-anteriorite-2025
```

### Étape 2 : Vérifier les checksums

```bash
sha256sum -c MANIFEST.sha256
```

**Résultat attendu** : Tous les fichiers doivent afficher `OK`.

---

## 2. VÉRIFICATION DE L'IDENTITÉ DE L'AUTEUR

### Documents à consulter

| Document | Contenu |
|----------|---------|
| `AUTHORSHIP.md` | Identité canonique de l'auteur |
| `IDENTITE-AUTEUR-HAMMANH.md` | Attestation avec référence passeport |
| `ERRATUM-NOM-AUTEUR.md` | Correction de l'erreur d'identité |

### Vérification croisée

1. Le nom **Marc Victor R BOUCHER** doit apparaître dans tous les documents officiels
2. Le pseudonyme **HammÅnH** doit être lié à ce nom
3. Aucune autre identité ne doit être présentée comme auteur

---

## 3. VÉRIFICATION DE L'ENTREPRISE

### Documents officiels

Les documents suivants se trouvent dans `/YATASANA TECHNOLOGIES LLC/Document fondation/` :

| Document | Vérification |
|----------|--------------|
| `MergedSS4AndArticles_-_YATASANA_Technologies.pdf` | Articles of Organization (Wyoming) |
| `EIN_Letter_-_YATASANA_Technologies_LLC.pdf` | Lettre IRS avec EIN |

### Vérification en ligne

**Wyoming Secretary of State** :
- Site : https://wyobiz.wyo.gov/
- Rechercher : "YATASANA Technologies LLC"
- ID attendu : 2025-001804429
- Date Filing : Oct 31 2025

---

## 4. VÉRIFICATION DES DÉPÔTS ZENODO

### DOI à vérifier

| DOI | URL |
|-----|-----|
| `10.5281/zenodo.17907126` | https://doi.org/10.5281/zenodo.17907126 |
| `10.5281/zenodo.17919637` | https://doi.org/10.5281/zenodo.17919637 |
| `10.5281/zenodo.17921269` | https://doi.org/10.5281/zenodo.17921269 |

### Vérification

1. Accéder à chaque URL
2. Vérifier que l'auteur est **Marc Victor R Boucher** ou **HammÅnH**
3. Vérifier les dates de dépôt
4. Télécharger et comparer avec les fichiers locaux

---

## 5. VÉRIFICATION US COPYRIGHT OFFICE

### Case Numbers

| Case # | Description |
|--------|-------------|
| `1-15055037321` | oXc Architecture |
| `1-15055007411` | The Weaver Effect |
| `1-15057280351` | HOLON BOX |

### Procédure

1. Accéder à : https://www.copyright.gov/
2. Utiliser la recherche publique
3. Entrer le numéro de case
4. Vérifier le titulaire et la date

---

## 6. VÉRIFICATION IPFS

### CID à vérifier

```
bafybeigzqdz3cny2fteawuiwqqeim3nojsssmkye5ejy2m3v67nbapjmci
```

### Procédure

```bash
# Via gateway IPFS public
curl https://ipfs.io/ipfs/bafybeigzqdz3cny2fteawuiwqqeim3nojsssmkye5ejy2m3v67nbapjmci
```

Ou accéder via navigateur :
- https://ipfs.io/ipfs/bafybeigzqdz3cny2fteawuiwqqeim3nojsssmkye5ejy2m3v67nbapjmci

---

## 7. VÉRIFICATION DE L'HISTORIQUE GIT

### Commits initiaux

```bash
git log --oneline --all | tail -20
```

### Vérifier les dates

```bash
git log --format="%H %ai %s" | head -50
```

### Premier commit

Le premier commit doit dater du **19 novembre 2024** ou avant.

---

## 8. VÉRIFICATION DE LA LICENCE

### Fichier : `LICENSE-OXC-v1.0.md`

Vérifier que :
1. Le copyright est au nom de **Marc Victor R BOUCHER (HammÅnH)**
2. L'entreprise pour juridiction est **YATASANA Technologies LLC**
3. La juridiction est **Wyoming, USA**

---

## 9. CHECKLIST COMPLÈTE

```
[ ] Checksums vérifiés (MANIFEST.sha256)
[ ] Identité auteur confirmée (AUTHORSHIP.md)
[ ] Entreprise vérifiée (Wyoming Secretary of State)
[ ] EIN vérifié (IRS Letter)
[ ] DOI Zenodo accessibles et corrects
[ ] US Copyright cases vérifiés
[ ] CID IPFS accessible
[ ] Historique Git cohérent
[ ] Licence correcte
```

---

## 10. EN CAS DE DOUTE

Si une vérification échoue ou si vous avez des questions :

- **Email** : hammanh@proton.me
- **GitHub** : [@Hammanh](https://github.com/Hammanh)

---

**Document créé** : 25 décembre 2025
**Auteur** : Marc Victor R BOUCHER (HammÅnH)

*Ce document permet à toute personne de vérifier indépendamment l'authenticité de ce dépôt.*
