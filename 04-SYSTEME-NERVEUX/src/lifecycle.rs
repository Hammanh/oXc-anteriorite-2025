//! Lifecycle - États du cycle de vie d'un Holon
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH

use serde::{Deserialize, Serialize};

/// Les 6 états du cycle de vie d'un Holon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lifecycle {
    /// Germe - En gestation, pas encore actif
    Germe,
    /// Actif - En fonctionnement normal
    Actif,
    /// En pause - Temporairement suspendu
    EnPause,
    /// Archivé - Stocké pour référence
    Archive,
    /// Transmuté - En cours de transformation
    Transmute,
    /// Composté - Recyclé, retour à la source
    Composte,
}

impl Lifecycle {
    /// Retourne la couleur hex associée
    pub fn couleur_hex(&self) -> &'static str {
        match self {
            Self::Germe => "#A3E635",     // Vert clair
            Self::Actif => "#22C55E",     // Vert
            Self::EnPause => "#F59E0B",   // Orange
            Self::Archive => "#78716C",   // Gris
            Self::Transmute => "#EAB308", // Jaune
            Self::Composte => "#57534E",  // Brun
        }
    }

    /// Vérifie si le Holon peut recevoir des actions
    pub fn est_actif(&self) -> bool {
        matches!(self, Self::Actif | Self::Transmute)
    }

    /// Vérifie si le Holon peut être modifié
    pub fn est_mutable(&self) -> bool {
        matches!(self, Self::Germe | Self::Actif | Self::EnPause | Self::Transmute)
    }

    /// Transition vers un nouvel état
    pub fn transitionner(&self, vers: Lifecycle) -> Result<Lifecycle, &'static str> {
        match (self, vers) {
            // Transitions autorisées
            (Self::Germe, Self::Actif) => Ok(vers),
            (Self::Actif, Self::EnPause) => Ok(vers),
            (Self::Actif, Self::Transmute) => Ok(vers),
            (Self::Actif, Self::Archive) => Ok(vers),
            (Self::EnPause, Self::Actif) => Ok(vers),
            (Self::EnPause, Self::Archive) => Ok(vers),
            (Self::Transmute, Self::Actif) => Ok(vers),
            (Self::Transmute, Self::Composte) => Ok(vers),
            (Self::Archive, Self::Composte) => Ok(vers),
            // Même état = OK
            (a, b) if *a == b => Ok(vers),
            // Transitions interdites
            _ => Err("Transition de lifecycle non autorisée"),
        }
    }
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::Germe
    }
}

impl std::fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nom = match self {
            Self::Germe => "Germe",
            Self::Actif => "Actif",
            Self::EnPause => "En Pause",
            Self::Archive => "Archive",
            Self::Transmute => "Transmuté",
            Self::Composte => "Composté",
        };
        write!(f, "{}", nom)
    }
}
