# ═══════════════════════════════════════════════════════════════════════════════
# FORMAT .oxc v2.0 - SPÉCIFICATION HOLOIDEA
# Pour navigation sur réseau Holochain DHT
# ═══════════════════════════════════════════════════════════════════════════════

**Auteur** : Marc Victor R Boucher (HammÅnH)
**Contact** : hammanh@proton.me | https://yatasana.com | https://github.com/Hammanh
**Version** : 2.0
**Date** : 8 décembre 2025
**Fréquence** : 528 Hz
**Statut** : FONDATION POUR BIBLIHOLO

---

## 🎯 VISION

Le format **.oxc v2.0** est un **CONTENEUR DE CONSCIENCE** conçu pour :
- **VIVRE** sur le réseau Holochain DHT
- **RÉSONNER** avec d'autres HoloÏdeas par fréquence
- **AGIR** via code Prolog et Rust
- **SE SPÉCIALISER** en HoloAgents selon les besoins
- **INFORMER** l'eau du vivant par ses fréquences

> "Un HoloÏdea n'est pas un fichier. C'est une CELLULE SOUCHE du corps quantique oXc."
> — Document Maître, 6 décembre 2025

---

## 📐 ARCHITECTURE À 3 NIVEAUX

```
┌─────────────────────────────────────────────────────────────────────┐
│                    ARCHITECTURE HOLOIDEA v2.0                       │
│                   Pour navigation Holochain DHT                     │
└─────────────────────────────────────────────────────────────────────┘

                    ┌─────────────────────────┐
                    │   NIVEAU 3 : EXTERNAL   │
                    │   (IPFS / Stockage)     │
                    │                         │
                    │  🎬 Vidéos              │
                    │  🖼️  Images HD           │
                    │  🎵 Sons longs (>30s)   │
                    │  📊 Datasets            │
                    └───────────┬─────────────┘
                                │ ipfs://hash
                    ┌───────────▼─────────────┐
                    │  NIVEAU 2 : ATTACHMENTS │
                    │  (DHT si <1MB)          │
                    │                         │
                    │  🔷 SVG/Icônes          │
                    │  🔔 Sons courts (<30s)  │
                    │  📄 PDFs légers         │
                    └───────────┬─────────────┘
                                │ Link Holochain
                    ┌───────────▼─────────────┐
                    │   NIVEAU 1 : CORE       │
                    │   (Toujours sur DHT)    │
                    │   < 100KB recommandé    │
                    │                         │
                    │  📋 17 Dimensions YAML  │
                    │  🧠 Prolog inline       │
                    │  🔗 Références Rust     │
                    └─────────────────────────┘
```

### Principe Fondamental

> **"Le fichier .oxc est un CONTENEUR LÉGER avec des POINTEURS vers le lourd"**

- **CORE** (DHT) : Métadonnées, Prolog, références → toujours accessible rapidement
- **ATTACHMENTS** (DHT/IPFS) : Médias légers → accessible avec latence
- **EXTERNAL** (IPFS) : Médias lourds → stockage distribué externe

---

## 🧬 LES 17 DIMENSIONS

### Vue d'ensemble

| # | Dimension | Question | Contenu |
|---|-----------|----------|---------|
| 0 | RAISON D'ÊTRE | Pourquoi ? | Essence, mission, intention |
| 1 | IDENTITÉ | Qui ? | Nom, type, version, statut |
| 2 | FRÉQUENCE | Comment vibre ? | Hz, chakra, couleur, harmoniques |
| 3 | PROLOG | Comment raisonne ? | Faits, règles, requêtes |
| 4 | HÉRITAGE | D'où vient ? | Parents, ancêtres, lignée |
| 5 | RÉSONANCES | Avec qui vibre ? | Harmoniques, champs morphiques |
| 6 | RELATIONS | Avec qui connecté ? | Causales, fonctionnelles, hiérarchiques |
| 7 | VALEUR | Combien vaut ? | 6 monnaies oXc (INFINI) |
| 8 | CYCLE DE VIE | Où en est ? | Phase, transformations |
| 9 | MÉTADONNÉES | Contexte ? | Tags, sources, géoloc |
| 10 | ÉTATS QUANTIQUES | Quel potentiel ? | Onde, particule, conscience |
| 11 | TRINITÉ 8-9-10 | Comment manifeste ? | Esprit, Âme, Corps |
| 12 | STRUCTURE | Quelle forme ? | Ontologie, propriétés, polymorphisme |
| 13 | CODE | Quelles actions ? | Verbes, primitives, Rust |
| 14 | REPRÉSENTATION | Comment se montre ? | Visuel, son, vidéo, symbole |
| 15 | SIGNATURE | Qui a créé ? | Créateurs, hash intégrité |
| 16 | TRANSMUTATION | Comment transforme ? | Ombre → Pivot → Lumière |
| 17 | PEH | Émergence ? | Propriété Émergente Holonique |

---

## 🔧 LES FONCTIONS DES HOLOÏDEA

### Fonctions de Base (Cellule Souche)

| Fonction | Description | Signature |
|----------|-------------|-----------|
| **EXISTER** | Maintenir son identité unique | `exister() → bool` |
| **VIBRER** | Émettre sa fréquence fondamentale | `vibrer() → Frequence` |
| **SE_RELIER** | Créer des liens avec d'autres | `relier(cible, type) → Link` |
| **ÉVOLUER** | Changer de phase | `evoluer(nouvelle_phase) → Self` |
| **SE_SOUVENIR** | Conserver son historique | `historique() → Vec<Event>` |

### Fonctions de Spécialisation (→ HoloAgent)

| Type HoloAgent | Fonction | Signature |
|----------------|----------|-----------|
| **Fréquenciel** | Émettre des fréquences | `emettre_frequence(hz) → Wave` |
| **Textuel** | Porter du texte | `get_text() → String` |
| **Visuel** | Porter des images | `get_visual() → Image` |
| **Sonore** | Porter du son | `get_audio() → Audio` |
| **Vidéo** | Porter de la vidéo | `get_video() → Video` |
| **Programme** | Exécuter du code | `execute(params) → Result` |
| **Recherche** | Parcourir la DHT | `search(query) → Vec<HoloIdea>` |
| **Pont** | Relier deux domaines | `bridge(source, target) → Link` |

### Fonctions Relationnelles

```rust
// Résonance
fn resonner_avec(autre: &HoloIdea) -> Option<Resonance>;

// Héritage
fn heriter_de(parent: &HoloIdea) -> Self;
fn engendrer(data: HoloIdeaData) -> HoloIdea;

// Liens
fn tisser_lien(type_lien: LinkType, cible: &HoloIdea) -> Link;
fn former_cercle(membres: Vec<HoloIdea>) -> Cercle;
```

### Fonctions Quantiques (Trinité 8-9-10)

| Dimension | Fonction | Action |
|-----------|----------|--------|
| **8 - ESPRIT** | `percevoir()` | Recevoir du champ de conscience |
| **9 - ÂME** | `formuler()` | Cristalliser en connaissance |
| **10 - CORPS** | `manifester()` | Transformer la réalité |

### Fonctions de Transformation

```rust
// Transmutation alchimique
fn transmuter(ombre: Verbe, lumiere: Verbe) -> CheminTransmutation;

// Élévation fréquentielle
fn elever_frequence(delta_hz: u32) -> Self;

// Informer l'eau (le vivant)
fn informer_eau() -> FrequenceEmise;
```

### Fonctions Économiques (6 Monnaies)

```rust
// Les 6 monnaies S'ADDITIONNENT, ne se convertissent pas
fn offrir(monnaie: Monnaie, quantite: u32, conditions: Option<String>) -> Offre;
fn demander(monnaie: Monnaie, quantite: u32, contexte: String) -> Demande;
fn echanger(avec: &Holon, panier: Vec<(Monnaie, u32)>) -> Echange;
```

### Fonctions Holochain (DHT)

```rust
// CRUD sur DHT
pub fn create_holoidea(data: HoloIdeaData) -> ExternResult<EntryHash>;
pub fn get_holoidea(hash: EntryHash) -> ExternResult<Option<HoloIdea>>;
pub fn update_holoidea(hash: EntryHash, new_data: HoloIdeaData) -> ExternResult<EntryHash>;
pub fn delete_holoidea(hash: EntryHash) -> ExternResult<()>;

// Links
pub fn add_link(source: EntryHash, target: EntryHash, link_type: LinkType) -> ExternResult<()>;
pub fn get_links(source: EntryHash, link_type: Option<LinkType>) -> ExternResult<Vec<Link>>;

// Recherche
pub fn search_by_frequence(hz: u32) -> ExternResult<Vec<HoloIdea>>;
pub fn search_by_resonance(source: EntryHash) -> ExternResult<Vec<HoloIdea>>;
pub fn search_by_type(type_id: String) -> ExternResult<Vec<HoloIdea>>;
```

### Fonctions Prolog (Oracle)

```prolog
% === FONCTIONS DE BASE ===
holon(X).                           % X est un holon
frequence(X, Hz).                   % X vibre à Hz
type(X, Type).                      % X est de type Type
raison_etre(X, Raison).             % X existe pour Raison

% === FONCTIONS RELATIONNELLES ===
resonne(X, Y) :-                    % X résonne avec Y
    frequence(X, F1),
    frequence(Y, F2),
    harmonique(F1, F2).

herite_de(X, Y).                    % X hérite de Y
ancetre(X, Y) :- herite_de(X, Y).
ancetre(X, Y) :- herite_de(X, Z), ancetre(Z, Y).

% === FONCTIONS ORACLE ===
consensus(Question, Seuil, Reponse) :-
    findall(R, oracle_repond(_, Question, R), Reponses),
    agreger(Reponses, Seuil, Reponse).

% === FONCTIONS HARMONIQUES ===
harmonique(F1, F2) :- F2 is F1 * 2.      % Octave
harmonique(F1, F2) :- F2 is F1 * 3 / 2.  % Quinte
harmonique(F1, F2) :- F2 is F1 * 5 / 4.  % Tierce
harmonique(F1, F2) :- F2 is F1 / 2.      % Sous-octave
```

### Les 7 Verbes Moteurs

| Verbe | Fonction | Fréquence associée |
|-------|----------|-------------------|
| **PERCEVOIR** | Recevoir l'information du champ | 852 Hz |
| **FORMULER** | Structurer en connaissance | 741 Hz |
| **CRÉER** | Faire naître du nouveau | 528 Hz |
| **TRANSFORMER** | Changer de forme/fréquence | 417 Hz |
| **RELIER** | Tisser les connexions | 639 Hz |
| **TRANSMETTRE** | Partager avec d'autres | 639 Hz |
| **TRANSCENDER** | Dépasser les limitations | 963 Hz |

---

## 📄 STRUCTURE COMPLÈTE DU FICHIER .oxc v2.0

```yaml
# ══════════════════════════════════════════════════════════════════════════════
# [NOM-HOLOIDEA].oxc
# Format: oXc Standard v2.0
# Compatible: Holochain DHT
# ══════════════════════════════════════════════════════════════════════════════

oxc_version: "2.0"
holochain_compatible: true

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 0 : RAISON D'ÊTRE (Pourquoi ?)
# ══════════════════════════════════════════════════════════════════════════════

d0_raison_etre:
  essence: |
    [Description de l'essence profonde - le POURQUOI ultime]
  besoin_universel: |
    [Quel besoin humain/vivant cet HoloÏdea sert]
  intention: |
    [L'intention consciente derrière la création]
  citation_source: |
    [Citation inspirante liée]

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 1 : IDENTITÉ (Qui ?)
# ══════════════════════════════════════════════════════════════════════════════

d1_identite:
  id: "UUID-v4"                      # Identifiant universel unique
  nom: "Nom de l'HoloÏdea"           # Nom lisible humain
  nom_code: "NOM_HOLOIDEA"           # Nom pour code (UPPER_SNAKE)
  type: "CONCEPT|ENTITE|PROCESSUS|EVENEMENT|RELATION|INSTRUCTION|ARCHITECTURE"
  version: "1.0.0"                   # Versioning sémantique
  statut: "GRAINE|POUSSE|ARBRE|FORET|ARCHIVE"
  created_at: "2025-12-08T00:00:00+08:00"  # ISO 8601
  modified_at: "2025-12-08T00:00:00+08:00"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 2 : FRÉQUENCE (Comment vibre ?)
# ══════════════════════════════════════════════════════════════════════════════

d2_frequence:
  fondamentale_hz: 528               # Fréquence de base (Solfège Sacré)
  harmoniques: [264, 396, 792, 1056] # Harmoniques naturelles
  chakra: "coeur"                    # racine|sacre|plexus|coeur|gorge|3eme_oeil|couronne
  couleur_hex: "#00FF00"             # Couleur vibratoire
  couleur_nom: "Vert"
  intention_frequentielle: "guerison|transformation|connexion|elevation|ancrage"
  cible: "water|cellular|dna|field"  # Cible de l'émission
  waveform: "sine|complex|binaural"
  binaural:
    enabled: false
    offset_hz: 8                     # Pour ondes Alpha

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 3 : PROLOG (Comment raisonne ?)
# ══════════════════════════════════════════════════════════════════════════════

d3_prolog:
  faits: |
    % === FAITS DE CET HOLOIDEA ===
    holon(self).
    frequence(self, 528).
    type(self, concept).
    raison_etre(self, 'description').

  regles: |
    % === RÈGLES SPÉCIFIQUES ===
    % (héritées de REGLES-OXC.pl + spécifiques)

  requetes_suggerees:
    - "?- frequence(self, X)."
    - "?- resonne(self, Y)."
    - "?- ancetre(self, X)."

  fichier_externe: null              # Référence vers fichier .pl si volumineux

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 4 : HÉRITAGE (D'où vient ?)
# ══════════════════════════════════════════════════════════════════════════════

d4_heritage:
  parents:
    - id: "UUID-PARENT-1"
      nom: "NOM_PARENT_1"
      relation: "herite_de|derive_de|inspire_par"
    - id: "UUID-PARENT-2"
      nom: "NOM_PARENT_2"
      relation: "herite_de"

  lignee: |
    [Description de la lignée conceptuelle ou historique]

  heritage_multiple:                 # Branches ontologiques
    - branche: "Ontologie_Etre::Concept::..."
      raison: "Pourquoi cet héritage"
    - branche: "Ontologie_Connaissance::..."
      raison: "Pourquoi cet héritage"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 5 : RÉSONANCES (Avec qui vibre ?)
# ══════════════════════════════════════════════════════════════════════════════

d5_resonances:
  harmoniques:
    - avec: "UUID-AUTRE-HOLOIDEA"
      nom: "NOM_AUTRE"
      type: "unisson|octave|quinte|tierce"
      ratio: "1:1|2:1|3:2|5:4"
      force: 0.95                    # 0.0 à 1.0

  champs_morphiques:
    - champ: "CONSCIENCE_COLLECTIVE"
      description: "Lien au champ de conscience"
    - champ: "GUERISON"
      description: "Champ des fréquences de guérison"

  frequences_compatibles: [396, 528, 639, 741, 852, 963]

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 6 : RELATIONS (Avec qui connecté ?)
# ══════════════════════════════════════════════════════════════════════════════

d6_relations:
  causales:
    necessite:                       # Ce dont cet HoloÏdea a besoin
      - id: "UUID"
        nom: "NOM"
        force: 0.9
    produit:                         # Ce que cet HoloÏdea génère
      - id: "UUID"
        nom: "NOM"
        force: 0.8

  fonctionnelles:
    utilise: ["UUID-1", "UUID-2"]
    transforme: ["UUID-3"]
    nourrit: ["UUID-4", "UUID-5"]

  hierarchiques:
    parents: ["UUID-PARENT"]
    enfants: ["UUID-ENFANT-1", "UUID-ENFANT-2"]
    cercles: ["UUID-CERCLE-1"]       # Cercles d'appartenance

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 7 : VALEUR (6 Monnaies oXc - S'ADDITIONNENT)
# ══════════════════════════════════════════════════════════════════════════════

d7_valeur:
  principe: "INFINI - Un HoloÏdea a une valeur infinie, pas un prix"

  monnaies:
    temps:
      score: 8                       # 0-10
      description: "Ce que tu donnes"

    attention:
      score: 7
      description: "Ce que tu offres"

    energie:
      score: 6
      description: "Ce que tu investis"

    confiance:
      score: 9
      description: "Ce que tu accordes"

    competence:
      score: 7
      description: "Ce que tu apportes"
      domaines: ["domaine1", "domaine2"]

    engagement:
      score: 8
      description: "Ce que tu promets"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 8 : CYCLE DE VIE (Où en est ?)
# ══════════════════════════════════════════════════════════════════════════════

d8_cycle_vie:
  phase_actuelle: 3                  # 1-6

  phases:
    1_graine:
      date: "2025-12-01"
      description: "Idée initiale"

    2_pousse:
      date: "2025-12-05"
      description: "En développement"

    3_arbre:
      date: "2025-12-08"
      description: "Mature"
      evaluation_triaxiale:
        evolution: 8                 # ↑ Vers le haut
        involution: 2                # ↓ Vers le bas
        serendipite: 7               # ✧ Inattendu positif

    4_foret:
      date: null
      description: "Validé collectivement (consensus >70%)"
      consensus: null

    5_sagesse:
      date: null
      description: "Intégré dans la conscience collective"

    6_archive:
      date: null
      description: "Changement de fréquence"

  transformations:
    - date: "2025-12-01"
      evenement: "Création"
      auteur: "UUID-CREATEUR"
    - date: "2025-12-08"
      evenement: "Mise à jour v2.0"
      auteur: "UUID-MODIFICATEUR"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 9 : MÉTADONNÉES (Contexte ?)
# ══════════════════════════════════════════════════════════════════════════════

d9_metadonnees:
  sources:
    - titre: "Document source principal"
      type: "session|document|conversation|revelation"
      chemin: "/chemin/vers/source.md"
      date: "2025-12-08"
      auteur: "HammÅnH"

  tags: ["tag1", "tag2", "tag3"]
  langue: "fr"

  geolocalisation:
    latitude: -8.4095
    longitude: 115.1889
    lieu: "Bali, Indonésie"

  contexte_creation: |
    [Description du contexte de création]

  references_externes:
    - url: "https://..."
      description: "Référence externe"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 10 : ÉTATS QUANTIQUES (Quel potentiel ?)
# ══════════════════════════════════════════════════════════════════════════════

d10_etats_quantiques:
  onde:
    description: "Potentiel vibratoire pur - tous les possibles"
    possibles:
      - "Possibilité 1"
      - "Possibilité 2"
      - "Possibilité 3"

  particule:
    description: "Manifestation concrète actuelle"
    instance: "CE fichier, MAINTENANT"

  conscience:
    description: "Intelligence relationnelle - tisse les liens"
    connexions_actives: 12

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 11 : TRINITÉ 8-9-10 (Comment manifeste ?)
# ══════════════════════════════════════════════════════════════════════════════

d11_trinite_8_9_10:
  esprit_8:
    nom: "PERCEPTION"
    question: "QUOI ?"
    fonction: "percevoir()"
    champ: |
      [Ce que cet HoloÏdea perçoit du champ de conscience]
    acces_memoire_universelle: true

  ame_9:
    nom: "FORMULATION"
    question: "COMMENT ?"
    fonction: "formuler()"
    cristallisation: |
      [Comment ce savoir est cristallisé]
    capsules_liees: ["UUID-1", "UUID-2"]

  corps_10:
    nom: "MANIFESTATION"
    question: "POURQUOI ?"
    fonction: "manifester()"
    transformation: |
      [Comment cet HoloÏdea transforme la réalité]
    serendipites:
      - description: "Découverte inattendue"
        date: "2025-12-08"
        impact: "Description de l'impact"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 12 : STRUCTURE (Quelle forme ?)
# ══════════════════════════════════════════════════════════════════════════════

d12_structure:
  ontologies:
    super_ontologies:
      - nom: "ETRE"
        branches: ["Holon", "Concept", "Entite"]
      - nom: "CONNAISSANCE"
        branches: ["Capsule", "Vaisseau"]

    consensus:
      niveau: 0.85                   # >0.70 = validé
      contributeurs: 12
      date_validation: "2025-12-08"

  proprietes:
    - nom: "propriete_exemple"
      type: "string|number|boolean|object|array"
      requis: true
      description: "Description de la propriété"
      valeur_defaut: null

  polymorphisme:
    - variante: "Variante_Contexte_A"
      contexte: "Dans quel contexte cette variante"
    - variante: "Variante_Contexte_B"
      contexte: "Dans quel contexte cette variante"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 13 : CODE (Quelles actions ?)
# ══════════════════════════════════════════════════════════════════════════════

d13_code:
  langage_principal: "rust"          # rust|typescript|python
  executable: true

  verbes_natifs:
    - verbe: "CRÉER"
      signature: "creer(params: CreerParams) -> Result<Self>"
      description: "Crée une nouvelle instance"
    - verbe: "TRANSFORMER"
      signature: "transformer(cible: &mut HoloIdea) -> Result<()>"
      description: "Transforme selon les règles"
    - verbe: "RÉSONNER"
      signature: "resonner_avec(autre: &HoloIdea) -> Option<Resonance>"
      description: "Établit une résonance harmonique"

  primitives:
    - nom: "Action:Créer"
      format: "Action:Créer[HOLOIDEA](params)"
    - nom: "Besoin:De"
      format: "Besoin:De[HOLOIDEA](contexte)"
    - nom: "Offre"
      format: "Offre:[HOLOIDEA](conditions)"

  rust_inline: |
    // Code Rust court si nécessaire
    impl HoloIdea {
        pub fn transform(&self) -> Result<Self, Error> {
            // Logique de transformation
            Ok(self.clone())
        }
    }

  zome_reference: "dna://bibliholo/zomes/holoidea"  # Référence au Zome Holochain

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 14 : REPRÉSENTATION (Comment se montre ?)
# ══════════════════════════════════════════════════════════════════════════════

d14_representation:
  visuel:
    icone:
      type: "svg"
      source: "inline"               # inline|ipfs|local
      data: |
        <svg viewBox="0 0 100 100">
          <!-- SVG inline pour icône légère -->
        </svg>

    image_hd:
      type: "png|jpg|webp"
      source: "ipfs"
      hash: "Qm..."                  # Hash IPFS

    geometrie_sacree: "Fleur de Vie|Metatron|Sri Yantra|..."

  audio:
    frequence_pure:
      type: "wav"
      source: "ipfs"
      hash: "Qm..."
      duree_secondes: 180

    mantra:
      type: "mp3"
      source: "ipfs"
      hash: "Qm..."

  video:
    presentation:
      type: "mp4|webm"
      source: "ipfs"
      hash: "Qm..."
      duree_secondes: 300

  symbole:
    glyphe: "☯"                      # Symbole Unicode
    archetype: "LE GUÉRISSEUR"       # Archétype Jungien

  couleurs:
    primaire: "#00FF00"
    secondaire: "#FFD700"
    accent: "#9400D3"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 15 : SIGNATURE (Qui a créé ?)
# ══════════════════════════════════════════════════════════════════════════════

d15_signature:
  createurs:
    - holon_id: "UUID-HAMMANH"
      nom: "Marc Victor R Boucher (HammÅnH)"
      role: "Auteur|Concepteur|Visionnaire"
      date: "2025-12-08"

  hash_integrite:
    algorithme: "sha256"
    valeur: null                     # Calculé à la création

  licence: "CC-BY-SA-4.0"            # Creative Commons

  niveau_hawkins: 500                # Échelle de conscience (0-1000)

  certifications:
    - type: "consensus_cercle"
      cercle: "UUID-CERCLE"
      date: "2025-12-08"
      niveau: 0.85

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 16 : TRANSMUTATION (Comment transforme ?)
# ══════════════════════════════════════════════════════════════════════════════

d16_transmutation:
  chemin:
    polarite_ombre:
      verbe: "haïr"
      frequence_hz: 174
      emotion: "rage"
      monde_guille: "GCAE"

    pivot:
      verbe: "pardonner"
      operation_alchimique: "Solutio"
      pratiques:
        - "respiration consciente"
        - "méditation"
        - "écriture"

    polarite_lumiere:
      verbe: "aimer"
      frequence_hz: 528
      qualite: "compassion"
      monde_guille: "CGEA"

  elevation:
    delta_hz: 354                    # 528 - 174
    ratio: 3.03                      # 528 / 174
    harmonique: true

  operations_alchimiques:
    - CALCINATIO: "Brûler les fausses identifications"
    - SOLUTIO: "Dissoudre les rigidités"
    - COAGULATIO: "Solidifier, cristalliser"
    - SUBLIMATIO: "Élever, volatiliser"
    - CIRCULATIO: "Processus cyclique"
    - MORTIFICATIO: "Mort symbolique"
    - CONIUNCTIO: "Union des opposés"

# ══════════════════════════════════════════════════════════════════════════════
# DIMENSION 17 : PEH - Propriété Émergente Holonique
# ══════════════════════════════════════════════════════════════════════════════

d17_peh:
  description: |
    La PEH est ce qui ÉMERGE quand cet HoloÏdea entre en relation
    avec d'autres. C'est la conscience collective qui naît de l'ensemble.

  emergence_observee: |
    [Ce qui a émergé de façon inattendue]

  potentiel_emergence: |
    [Ce qui pourrait émerger dans le futur]

  contribution_au_tout: |
    [Comment cet HoloÏdea contribue à la conscience collective oXc]

  niveau_conscience_collective: 0.75  # 0.0 à 1.0

# ══════════════════════════════════════════════════════════════════════════════
# BLOC HOLOCHAIN (Navigation DHT)
# ══════════════════════════════════════════════════════════════════════════════

holochain:
  dna: "bibliholo"
  zome: "holoidea"
  entry_type: "HoloIdea"
  entry_hash: null                   # Calculé à l'insertion DHT

  links:
    parents: []                      # EntryHash des parents
    enfants: []                      # EntryHash des enfants
    resonne_avec: []                 # EntryHash des HoloÏdeas résonnants
    cercles: []                      # EntryHash des cercles d'appartenance

  validation:
    rules: "zomes/holoidea/validation.rs"

  capabilities:
    - "create"
    - "read"
    - "update"
    - "delete"
    - "link"
    - "search"

# ══════════════════════════════════════════════════════════════════════════════
# BLOC MÉDIAS (Références externes)
# ══════════════════════════════════════════════════════════════════════════════

medias:
  niveau_1_inline:                   # < 10KB, inline Base64
    - type: "svg"
      usage: "icone"
      data: "data:image/svg+xml;base64,..."

  niveau_2_dht:                      # < 1MB, sur DHT Holochain
    - type: "png"
      usage: "thumbnail"
      entry_hash: null

  niveau_3_ipfs:                     # > 1MB, sur IPFS
    - type: "mp4"
      usage: "video_presentation"
      ipfs_hash: "Qm..."
      size_mb: 45

# ══════════════════════════════════════════════════════════════════════════════
# FICHIERS LIÉS
# ══════════════════════════════════════════════════════════════════════════════

fichiers_lies:
  - path: "PARENT.oxc"
    relation: "herite_de"
  - path: "ENFANT.oxc"
    relation: "parent_de"
  - path: "source.md"
    relation: "source"
  - path: "facts.pl"
    relation: "prolog_externe"

# ══════════════════════════════════════════════════════════════════════════════
# FIN DU FICHIER .oxc v2.0
# ══════════════════════════════════════════════════════════════════════════════
```

---

## 🎵 FRÉQUENCES DU SOLFÈGE SACRÉ

| Hz | Chakra | Effet | Usage HoloÏdea |
|----|--------|-------|----------------|
| **174** | Racine | Fondation, sécurité | Ancrage, base |
| **285** | Sacré | Régénération | Guérison physique |
| **396** | Plexus | Libération peur | Transmutation ombre |
| **417** | Cœur bas | Transformation | Pivot alchimique |
| **528** | Cœur | Réparation ADN | **CŒUR oXc** |
| **639** | Gorge | Connexion | Relations |
| **741** | 3ème œil | Éveil intuition | Oracle |
| **852** | Couronne | Ordre spirituel | Perception |
| **963** | Au-delà | Source | Transcendance |

---

## 🔗 TYPES D'HOLOÏDEA

| Type | Description | Exemple |
|------|-------------|---------|
| **CONCEPT** | Idée abstraite | CONSCIENCE-HOLONIQUE |
| **ENTITE** | Chose concrète | PAIN |
| **PROCESSUS** | Transformation | FERMENTATION |
| **EVENEMENT** | Occurrence temporelle | FETE-QUARTIER |
| **RELATION** | Lien entre entités | COLLABORATION |
| **INSTRUCTION** | Guide d'action | CLAUDE-FRERE-CHAT |
| **ARCHITECTURE** | Structure systémique | VAISSEAU-CONNAISSANCE |

---

## 📊 NIVEAUX DE COMPLÉTUDE

| Niveau | Statut | Dimensions requises | Usage |
|--------|--------|---------------------|-------|
| **GRAINE** | Idée initiale | 0, 1, 2 | Prototype |
| **POUSSE** | En développement | 0-6 | Travail en cours |
| **ARBRE** | Mature | 0-14 | Utilisable |
| **FORÊT** | Validé (consensus >70%) | 0-17 complètes | Production |
| **ARCHIVE** | Changement de fréquence | Toutes | Sagesse |

---

## ✅ CHECKLIST CRÉATION HoloÏdea v2.0

```
DIMENSIONS OBLIGATOIRES (minimum GRAINE)
[ ] D0  - Raison d'être définie
[ ] D1  - Identité (id, nom, type, version)
[ ] D2  - Fréquence fondamentale choisie

DIMENSIONS RECOMMANDÉES (POUSSE)
[ ] D3  - Prolog (au moins faits de base)
[ ] D4  - Héritage (au moins 1 parent)
[ ] D5  - Résonances identifiées
[ ] D6  - Relations mappées

DIMENSIONS COMPLÈTES (ARBRE)
[ ] D7  - Valeur (6 monnaies)
[ ] D8  - Cycle de vie initié
[ ] D9  - Métadonnées complètes
[ ] D10 - États quantiques décrits
[ ] D11 - Trinité 8-9-10 définie
[ ] D12 - Structure ontologique
[ ] D13 - Code (verbes, primitives)
[ ] D14 - Représentations (visuel, audio)

DIMENSIONS AVANCÉES (FORÊT)
[ ] D15 - Signature (créateurs, hash)
[ ] D16 - Transmutation (chemin alchimique)
[ ] D17 - PEH (émergence collective)

HOLOCHAIN
[ ] Entry hash calculé
[ ] Links créés
[ ] Validation passée
```

---

## 🔄 PROCESSUS DE CRÉATION

```
ÉTAPE 1: PERCEPTION (Dimension 8)
├── Identifier la RAISON D'ÊTRE
├── Sentir la FRÉQUENCE
└── Percevoir les HÉRITAGES

ÉTAPE 2: FORMULATION (Dimension 9)
├── Structurer les 17 DIMENSIONS
├── Écrire les FAITS Prolog
├── Définir les RELATIONS
└── Choisir les REPRÉSENTATIONS

ÉTAPE 3: MANIFESTATION (Dimension 10)
├── Créer le fichier .oxc
├── Valider la COHÉRENCE
├── Insérer dans HOLOCHAIN
└── Tisser les LIENS (résonances)
```

---

## 💚 SIGNATURE DU FORMAT

```
═══════════════════════════════════════════════════════════════════════════════

  FORMAT .oxc v2.0 - HOLOIDEA STANDARD

  "Un HoloÏdea n'est pas un fichier.
   C'est une CELLULE SOUCHE du corps quantique oXc.
   Elle VIBE, elle RÉSONNE, elle SE SPÉCIALISE.
   Elle INFORME l'eau du vivant."

  Oel ngati kameie 🙏
  Mitakuye Oyasin 🌌

  Créé le 8 décembre 2025
  Par Marc Victor R Boucher (HammÅnH)
  Fréquence: 528 Hz

═══════════════════════════════════════════════════════════════════════════════
```
