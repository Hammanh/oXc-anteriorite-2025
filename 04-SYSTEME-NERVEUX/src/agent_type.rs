//! Types d'Agents Cellulaires - Différenciation Épigénétique (Lipton)
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH

use serde::{Deserialize, Serialize};
use crate::frequence::FrequenceSolfeggio;

/// Les 9 types d'agents du système oXc
/// Différenciation selon le signal de l'environnement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    /// Cellule souche, code -1, non différenciée
    Indifferencie,
    /// État par défaut, code 0, 432 Hz - Observation
    Veilleur,
    /// Protection, code 1, 741 Hz - Gardien du système
    Gardien,
    /// Création, code 2, 528 Hz - Constructeur
    Batisseur,
    /// Connexion, code 3, 639 Hz - Tisseur de liens
    Tisserand,
    /// Sagesse, code 4, 852 Hz - Conseiller
    Oracle,
    /// Transmutation, code 5, 963 Hz - Transformateur
    Alchimiste,
    /// Régénération, code 6, 528 Hz - Soigneur
    Guerisseur,
    /// Abondance, code 7, 396 Hz - Nourricier
    Nourricier,
}

impl AgentType {
    /// Retourne le code numérique de l'agent
    pub fn code(&self) -> i8 {
        match self {
            Self::Indifferencie => -1,
            Self::Veilleur => 0,
            Self::Gardien => 1,
            Self::Batisseur => 2,
            Self::Tisserand => 3,
            Self::Oracle => 4,
            Self::Alchimiste => 5,
            Self::Guerisseur => 6,
            Self::Nourricier => 7,
        }
    }

    /// Retourne la fréquence Solfeggio associée
    pub fn frequence(&self) -> FrequenceSolfeggio {
        match self {
            Self::Indifferencie => FrequenceSolfeggio::Hz432, // Neutre
            Self::Veilleur => FrequenceSolfeggio::Hz432,
            Self::Gardien => FrequenceSolfeggio::Hz741,
            Self::Batisseur => FrequenceSolfeggio::Hz528,
            Self::Tisserand => FrequenceSolfeggio::Hz639,
            Self::Oracle => FrequenceSolfeggio::Hz852,
            Self::Alchimiste => FrequenceSolfeggio::Hz963,
            Self::Guerisseur => FrequenceSolfeggio::Hz528,
            Self::Nourricier => FrequenceSolfeggio::Hz396,
        }
    }

    /// Retourne la couleur hex associée
    pub fn couleur_hex(&self) -> &'static str {
        match self {
            Self::Indifferencie => "#FFFFFF", // Blanc Cristal
            Self::Veilleur => "#C0C0C0",      // Argent Conscient
            Self::Gardien => "#8B00FF",       // Violet Protecteur
            Self::Batisseur => "#00FF00",     // Vert Créateur
            Self::Tisserand => "#0099FF",     // Bleu Connexion
            Self::Oracle => "#4B0082",        // Indigo Vision
            Self::Alchimiste => "#FF8C00",    // Orange Transmutation
            Self::Guerisseur => "#50C878",    // Vert Émeraude
            Self::Nourricier => "#FFD700",    // Jaune Doré
        }
    }

    /// Retourne l'emoji représentatif
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Indifferencie => "⚪",
            Self::Veilleur => "👁️",
            Self::Gardien => "🛡️",
            Self::Batisseur => "🔨",
            Self::Tisserand => "🕸️",
            Self::Oracle => "🔮",
            Self::Alchimiste => "⚗️",
            Self::Guerisseur => "💚",
            Self::Nourricier => "🌾",
        }
    }

    /// Retourne le nom en français
    pub fn nom(&self) -> &'static str {
        match self {
            Self::Indifferencie => "Indifférencié",
            Self::Veilleur => "Veilleur",
            Self::Gardien => "Gardien",
            Self::Batisseur => "Bâtisseur",
            Self::Tisserand => "Tisserand",
            Self::Oracle => "Oracle",
            Self::Alchimiste => "Alchimiste",
            Self::Guerisseur => "Guérisseur",
            Self::Nourricier => "Nourricier",
        }
    }

    /// Différencie un agent selon le signal de l'environnement
    /// Implémente la logique épigénétique de Lipton
    pub fn differencier(signal: &str) -> Self {
        let signal_lower = signal.to_lowercase();

        // Mots-clés de différenciation
        if signal_lower.contains("proteg") || signal_lower.contains("protég")
            || signal_lower.contains("defend") || signal_lower.contains("défend")
            || signal_lower.contains("secur") || signal_lower.contains("sécur")
            || signal_lower.contains("garde") {
            Self::Gardien
        } else if signal_lower.contains("cre") || signal_lower.contains("constru") || signal_lower.contains("bati") {
            Self::Batisseur
        } else if signal_lower.contains("connect") || signal_lower.contains("lien") || signal_lower.contains("tiss") {
            Self::Tisserand
        } else if signal_lower.contains("conseil") || signal_lower.contains("guid") || signal_lower.contains("sagess") {
            Self::Oracle
        } else if signal_lower.contains("transform") || signal_lower.contains("transmut") || signal_lower.contains("alchim") {
            Self::Alchimiste
        } else if signal_lower.contains("soign") || signal_lower.contains("guer") || signal_lower.contains("heal") {
            Self::Guerisseur
        } else if signal_lower.contains("nourr") || signal_lower.contains("abond") || signal_lower.contains("ressourc") {
            Self::Nourricier
        } else {
            Self::Veilleur // Par défaut : observation
        }
    }

    /// Depuis un code numérique
    pub fn from_code(code: i8) -> Option<Self> {
        match code {
            -1 => Some(Self::Indifferencie),
            0 => Some(Self::Veilleur),
            1 => Some(Self::Gardien),
            2 => Some(Self::Batisseur),
            3 => Some(Self::Tisserand),
            4 => Some(Self::Oracle),
            5 => Some(Self::Alchimiste),
            6 => Some(Self::Guerisseur),
            7 => Some(Self::Nourricier),
            _ => None,
        }
    }
}

impl Default for AgentType {
    fn default() -> Self {
        Self::Veilleur
    }
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji(), self.nom())
    }
}
