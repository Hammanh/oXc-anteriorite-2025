//! Verbes Moteurs oXc - Les 19 verbes du système
//!
//! "Au commencement était le Verbe, et le Verbe était Moteur."
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH

use serde::{Deserialize, Serialize};
use crate::frequence::FrequenceSolfeggio;

/// Les 7 Verbes du Cycle de Conscience (L'Âme)
/// Ils structurent la pensée avant l'action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerbeNatif {
    /// 852 Hz - Entrée - Voir ce qui est, sans jugement (CNV Observation)
    Percevoir,
    /// 741 Hz - Traitement - Cristalliser le ressenti en structure
    Formuler,
    /// 639 Hz - Connexion - Trouver l'écho dans le réseau
    Resonner,
    /// 528 Hz - Ancrage - Définir la forme pensée
    Cristalliser,
    /// 417 Hz - Maillage - Lier les ressources nécessaires
    Tisser,
    /// 396 Hz - Sortie - Exécuter dans la matière
    Manifester,
    /// 963 Hz - Transcendance - Dépasser, élever
    Transcender,
}

impl VerbeNatif {
    /// Retourne la fréquence Hz associée
    pub fn frequence(&self) -> FrequenceSolfeggio {
        match self {
            Self::Percevoir => FrequenceSolfeggio::Hz852,
            Self::Formuler => FrequenceSolfeggio::Hz741,
            Self::Resonner => FrequenceSolfeggio::Hz639,
            Self::Cristalliser => FrequenceSolfeggio::Hz528,
            Self::Tisser => FrequenceSolfeggio::Hz417,
            Self::Manifester => FrequenceSolfeggio::Hz396,
            Self::Transcender => FrequenceSolfeggio::Hz963,
        }
    }

    /// Retourne la phase du cycle
    pub fn phase(&self) -> &'static str {
        match self {
            Self::Percevoir => "Entrée",
            Self::Formuler => "Traitement",
            Self::Resonner => "Connexion",
            Self::Cristalliser => "Ancrage",
            Self::Tisser => "Maillage",
            Self::Manifester => "Sortie",
            Self::Transcender => "Transcendance",
        }
    }

    /// Retourne la fonction du verbe
    pub fn fonction(&self) -> &'static str {
        match self {
            Self::Percevoir => "Voir ce qui est, sans jugement",
            Self::Formuler => "Cristalliser le ressenti en structure",
            Self::Resonner => "Trouver l'écho dans le réseau",
            Self::Cristalliser => "Définir la forme pensée",
            Self::Tisser => "Lier les ressources nécessaires",
            Self::Manifester => "Exécuter dans la matière",
            Self::Transcender => "Dépasser les limites, élever",
        }
    }

    /// Tous les verbes natifs
    pub fn tous() -> &'static [VerbeNatif] {
        &[
            Self::Percevoir,
            Self::Formuler,
            Self::Resonner,
            Self::Cristalliser,
            Self::Tisser,
            Self::Manifester,
            Self::Transcender,
        ]
    }
}

impl std::fmt::Display for VerbeNatif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nom = match self {
            Self::Percevoir => "PERCEVOIR",
            Self::Formuler => "FORMULER",
            Self::Resonner => "RÉSONNER",
            Self::Cristalliser => "CRISTALLISER",
            Self::Tisser => "TISSER",
            Self::Manifester => "MANIFESTER",
            Self::Transcender => "TRANSCENDER",
        };
        write!(f, "{}", nom)
    }
}

/// Les 12 Verbes BDO (Besoins - Demandes - Offres)
/// Actions concrètes dans le monde matériel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerbeBDO {
    /// Préserver, protéger
    Sauver,
    /// Alimenter, sustenter
    Nourrir,
    /// Initier, lancer
    Demarrer,
    /// Troquer, permuter
    Echanger,
    /// Restaurer, corriger
    Reparer,
    /// Former, enseigner
    Eduquer,
    /// Relier, associer
    Connecter,
    /// Collaborer, co-créer
    CoCreer,
    /// Faire croître, développer
    Cultiver,
    /// Soigner, régénérer
    Guerir,
    /// Choisir, trancher
    Decider,
    /// Déplacer, acheminer
    Transporter,
}

impl VerbeBDO {
    /// Retourne la fréquence Hz associée (selon le domaine d'action)
    pub fn frequence(&self) -> FrequenceSolfeggio {
        match self {
            Self::Sauver => FrequenceSolfeggio::Hz741,      // Protection
            Self::Nourrir => FrequenceSolfeggio::Hz528,     // Création/Vie
            Self::Demarrer => FrequenceSolfeggio::Hz417,    // Transformation
            Self::Echanger => FrequenceSolfeggio::Hz639,    // Connexion
            Self::Reparer => FrequenceSolfeggio::Hz528,     // Guérison
            Self::Eduquer => FrequenceSolfeggio::Hz852,     // Intuition
            Self::Connecter => FrequenceSolfeggio::Hz639,   // Connexion
            Self::CoCreer => FrequenceSolfeggio::Hz528,     // Création
            Self::Cultiver => FrequenceSolfeggio::Hz528,    // Croissance
            Self::Guerir => FrequenceSolfeggio::Hz528,      // Guérison
            Self::Decider => FrequenceSolfeggio::Hz741,     // Expression
            Self::Transporter => FrequenceSolfeggio::Hz417, // Mouvement
        }
    }

    /// Retourne la polarité BDO
    pub fn polarite(&self) -> Polarite {
        match self {
            Self::Sauver | Self::Reparer | Self::Guerir => Polarite::Besoin,
            Self::Nourrir | Self::Eduquer | Self::Cultiver => Polarite::Offre,
            Self::Demarrer | Self::Echanger | Self::Connecter |
            Self::CoCreer | Self::Decider | Self::Transporter => Polarite::Demande,
        }
    }

    /// Tous les verbes BDO
    pub fn tous() -> &'static [VerbeBDO] {
        &[
            Self::Sauver,
            Self::Nourrir,
            Self::Demarrer,
            Self::Echanger,
            Self::Reparer,
            Self::Eduquer,
            Self::Connecter,
            Self::CoCreer,
            Self::Cultiver,
            Self::Guerir,
            Self::Decider,
            Self::Transporter,
        ]
    }
}

impl std::fmt::Display for VerbeBDO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nom = match self {
            Self::Sauver => "SAUVER",
            Self::Nourrir => "NOURRIR",
            Self::Demarrer => "DÉMARRER",
            Self::Echanger => "ÉCHANGER",
            Self::Reparer => "RÉPARER",
            Self::Eduquer => "ÉDUQUER",
            Self::Connecter => "CONNECTER",
            Self::CoCreer => "CO-CRÉER",
            Self::Cultiver => "CULTIVER",
            Self::Guerir => "GUÉRIR",
            Self::Decider => "DÉCIDER",
            Self::Transporter => "TRANSPORTER",
        };
        write!(f, "{}", nom)
    }
}

/// Polarité BDO - Besoins / Demandes / Offres
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Polarite {
    /// YIN - Appel du vide, réception
    Besoin,
    /// VECTEUR - Mise en mouvement, formulation
    Demande,
    /// YANG - Capacité de réponse, émission
    Offre,
}

impl Polarite {
    pub fn symbole(&self) -> &'static str {
        match self {
            Self::Besoin => "☯ YIN",
            Self::Demande => "→ VECTEUR",
            Self::Offre => "☯ YANG",
        }
    }
}

/// Verbe unifié - peut être Natif ou BDO
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Verbe {
    Natif(VerbeNatif),
    Bdo(VerbeBDO),
}

impl Verbe {
    /// Retourne la fréquence du verbe
    pub fn frequence(&self) -> FrequenceSolfeggio {
        match self {
            Self::Natif(v) => v.frequence(),
            Self::Bdo(v) => v.frequence(),
        }
    }

    /// Parse un verbe depuis une chaîne
    pub fn from_str(s: &str) -> Option<Self> {
        let s_upper = s.to_uppercase();
        let s_clean: String = s_upper.chars()
            .filter(|c| c.is_alphabetic())
            .collect();

        // Essaie d'abord les verbes natifs
        match s_clean.as_str() {
            "PERCEVOIR" => return Some(Self::Natif(VerbeNatif::Percevoir)),
            "FORMULER" => return Some(Self::Natif(VerbeNatif::Formuler)),
            "RESONNER" | "RÉSONNER" => return Some(Self::Natif(VerbeNatif::Resonner)),
            "CRISTALLISER" => return Some(Self::Natif(VerbeNatif::Cristalliser)),
            "TISSER" => return Some(Self::Natif(VerbeNatif::Tisser)),
            "MANIFESTER" => return Some(Self::Natif(VerbeNatif::Manifester)),
            "TRANSCENDER" => return Some(Self::Natif(VerbeNatif::Transcender)),
            _ => {}
        }

        // Puis les verbes BDO
        match s_clean.as_str() {
            "SAUVER" => Some(Self::Bdo(VerbeBDO::Sauver)),
            "NOURRIR" => Some(Self::Bdo(VerbeBDO::Nourrir)),
            "DEMARRER" | "DÉMARRER" => Some(Self::Bdo(VerbeBDO::Demarrer)),
            "ECHANGER" | "ÉCHANGER" => Some(Self::Bdo(VerbeBDO::Echanger)),
            "REPARER" | "RÉPARER" => Some(Self::Bdo(VerbeBDO::Reparer)),
            "EDUQUER" | "ÉDUQUER" => Some(Self::Bdo(VerbeBDO::Eduquer)),
            "CONNECTER" => Some(Self::Bdo(VerbeBDO::Connecter)),
            "COCREER" | "COCRÉER" => Some(Self::Bdo(VerbeBDO::CoCreer)),
            "CULTIVER" => Some(Self::Bdo(VerbeBDO::Cultiver)),
            "GUERIR" | "GUÉRIR" => Some(Self::Bdo(VerbeBDO::Guerir)),
            "DECIDER" | "DÉCIDER" => Some(Self::Bdo(VerbeBDO::Decider)),
            "TRANSPORTER" => Some(Self::Bdo(VerbeBDO::Transporter)),
            _ => None,
        }
    }
}

impl std::fmt::Display for Verbe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Natif(v) => write!(f, "{}", v),
            Self::Bdo(v) => write!(f, "{}", v),
        }
    }
}
