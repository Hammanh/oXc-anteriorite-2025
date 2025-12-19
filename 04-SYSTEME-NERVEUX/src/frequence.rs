//! Fréquences Solfeggio - Signature vibratoire des Holons
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH

use serde::{Deserialize, Serialize};

/// Les 10 fréquences Solfeggio du système oXc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum FrequenceSolfeggio {
    /// 174 Hz - Fondation, Sécurité primale
    Hz174 = 174,
    /// 285 Hz - Guérison des tissus
    Hz285 = 285,
    /// 396 Hz - Libération de la peur, Manifestation
    Hz396 = 396,
    /// 417 Hz - Transformation, Changement
    Hz417 = 417,
    /// 432 Hz - Harmonie, Fréquence Terre
    Hz432 = 432,
    /// 528 Hz - Amour/ADN, Création, Guérison
    Hz528 = 528,
    /// 639 Hz - Connexion, Relations
    Hz639 = 639,
    /// 741 Hz - Expression, Structure
    Hz741 = 741,
    /// 852 Hz - Intuition, Perception
    Hz852 = 852,
    /// 963 Hz - Unité, Transcendance, Le Verbe
    Hz963 = 963,
}

impl FrequenceSolfeggio {
    /// Retourne la valeur Hz
    pub fn hz(&self) -> u16 {
        *self as u16
    }

    /// Retourne la couleur hex associée
    pub fn couleur_hex(&self) -> &'static str {
        match self {
            Self::Hz174 => "#8B0000", // Rouge Ombre
            Self::Hz285 => "#FF4500", // Rouge Orange
            Self::Hz396 => "#FF0000", // Rouge
            Self::Hz417 => "#FF8C00", // Orange
            Self::Hz432 => "#DAA520", // Jaune Terre
            Self::Hz528 => "#00FF00", // Vert
            Self::Hz639 => "#0099FF", // Bleu
            Self::Hz741 => "#6366F1", // Indigo
            Self::Hz852 => "#8B5CF6", // Violet
            Self::Hz963 => "#F5F5F5", // Blanc Unité
        }
    }

    /// Retourne l'intention associée
    pub fn intention(&self) -> &'static str {
        match self {
            Self::Hz174 => "Sécurité primale",
            Self::Hz285 => "Guérison tissus",
            Self::Hz396 => "Libération peur / Manifestation",
            Self::Hz417 => "Transformation",
            Self::Hz432 => "Harmonie Terre",
            Self::Hz528 => "Amour / ADN / Création",
            Self::Hz639 => "Connexion",
            Self::Hz741 => "Expression / Structure",
            Self::Hz852 => "Intuition / Perception",
            Self::Hz963 => "Unité / Transcendance",
        }
    }

    /// Calcule le ratio harmonique entre deux fréquences
    /// Retourne un score de résonance entre 0.0 et 1.0
    pub fn resonance_avec(&self, autre: &FrequenceSolfeggio) -> f64 {
        let f1 = self.hz() as f64;
        let f2 = autre.hz() as f64;

        // Ratio harmonique
        let ratio = if f1 > f2 { f1 / f2 } else { f2 / f1 };

        // Harmoniques parfaites : 1.0, 1.5, 2.0, 3.0
        let harmoniques = [1.0, 1.5, 2.0, 2.5, 3.0, 4.0];

        // Trouve l'harmonique la plus proche
        let mut min_distance = f64::MAX;
        for h in harmoniques {
            let distance = (ratio - h).abs();
            if distance < min_distance {
                min_distance = distance;
            }
        }

        // Score inverse de la distance (plus proche = meilleur score)
        let score = 1.0 / (1.0 + min_distance);
        score.clamp(0.0, 1.0)
    }

    /// Fréquences depuis un entier
    pub fn from_hz(hz: u16) -> Option<Self> {
        match hz {
            174 => Some(Self::Hz174),
            285 => Some(Self::Hz285),
            396 => Some(Self::Hz396),
            417 => Some(Self::Hz417),
            432 => Some(Self::Hz432),
            528 => Some(Self::Hz528),
            639 => Some(Self::Hz639),
            741 => Some(Self::Hz741),
            852 => Some(Self::Hz852),
            963 => Some(Self::Hz963),
            _ => None,
        }
    }
}

impl Default for FrequenceSolfeggio {
    fn default() -> Self {
        Self::Hz432 // Fréquence Terre par défaut
    }
}

impl std::fmt::Display for FrequenceSolfeggio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Hz", self.hz())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_identique() {
        let f = FrequenceSolfeggio::Hz528;
        assert!((f.resonance_avec(&f) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_resonance_harmonique() {
        // 528 et 264 seraient en ratio 2:1 (octave parfait)
        // On teste des fréquences proches
        let f1 = FrequenceSolfeggio::Hz396;
        let f2 = FrequenceSolfeggio::Hz852;
        let score = f1.resonance_avec(&f2);
        assert!(score > 0.3); // Doit avoir une certaine résonance
    }
}
