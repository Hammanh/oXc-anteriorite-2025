//! Moteur de Résonance - Matching par fréquences vibratoires
//!
//! "Nous ne comparons pas des chaînes de caractères (String Matching).
//!  Nous superposons des VECTEURS VIBRATOIRES."
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH

use crate::holon::Holon;
use crate::verbes::Polarite;

/// Résultat d'un matching par résonance
#[derive(Debug, Clone)]
pub struct ResonanceMatch {
    /// Holon source
    pub source_id: uuid::Uuid,
    /// Holon cible
    pub cible_id: uuid::Uuid,
    /// Score de résonance (0.0 à 1.0)
    pub score: f64,
    /// Type de connexion
    pub connexion: TypeConnexion,
}

/// Type de connexion entre deux Holons
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeConnexion {
    /// Besoin ↔ Offre : Arc électrique, connexion directe
    ArcElectrique,
    /// Même fréquence : Résonance harmonique
    Harmonique,
    /// Demande intermédiaire : Pont de connexion
    Pont,
    /// Faible résonance
    Faible,
}

/// Moteur de résonance pour le matching des Holons
pub struct ResonanceEngine {
    /// Seuil minimum de résonance pour un match
    seuil_min: f64,
    /// Seuil pour un arc électrique (connexion forte)
    seuil_arc: f64,
}

impl Default for ResonanceEngine {
    fn default() -> Self {
        Self {
            seuil_min: 0.3,
            seuil_arc: 0.7,
        }
    }
}

impl ResonanceEngine {
    /// Crée un nouveau moteur avec des seuils personnalisés
    pub fn new(seuil_min: f64, seuil_arc: f64) -> Self {
        Self { seuil_min, seuil_arc }
    }

    /// Calcule la résonance entre deux Holons
    pub fn calculer(&self, source: &Holon, cible: &Holon) -> Option<ResonanceMatch> {
        let score = source.resonance_avec(cible);

        if score < self.seuil_min {
            return None;
        }

        let connexion = self.determiner_connexion(source, cible, score);

        Some(ResonanceMatch {
            source_id: source.id,
            cible_id: cible.id,
            score,
            connexion,
        })
    }

    /// Trouve tous les matches pour un Holon source parmi une liste de cibles
    pub fn trouver_matches<'a>(
        &self,
        source: &Holon,
        cibles: impl Iterator<Item = &'a Holon>,
    ) -> Vec<ResonanceMatch> {
        let mut matches: Vec<ResonanceMatch> = cibles
            .filter(|c| c.id != source.id) // Exclure soi-même
            .filter_map(|c| self.calculer(source, c))
            .collect();

        // Trier par score décroissant
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        matches
    }

    /// Trouve le meilleur match (Besoin → Offre ou Offre → Besoin)
    pub fn trouver_meilleur_match<'a>(
        &self,
        source: &Holon,
        cibles: impl Iterator<Item = &'a Holon>,
    ) -> Option<ResonanceMatch> {
        self.trouver_matches(source, cibles).into_iter().next()
    }

    /// Détermine le type de connexion
    fn determiner_connexion(&self, source: &Holon, cible: &Holon, score: f64) -> TypeConnexion {
        // Arc électrique : Besoin ↔ Offre avec score élevé
        let est_complementaire = matches!(
            (&source.polarite, &cible.polarite),
            (Polarite::Besoin, Polarite::Offre) | (Polarite::Offre, Polarite::Besoin)
        );

        if est_complementaire && score >= self.seuil_arc {
            TypeConnexion::ArcElectrique
        } else if source.frequence == cible.frequence {
            TypeConnexion::Harmonique
        } else if source.polarite == Polarite::Demande || cible.polarite == Polarite::Demande {
            TypeConnexion::Pont
        } else {
            TypeConnexion::Faible
        }
    }

    /// Applique la loi de résonance oXc
    ///
    /// "Si (Vecteur_A == Vecteur_B) ET (Frequence_A harmonise Frequence_B)
    ///  ALORS Connexion = ARC ÉLECTRIQUE INSTANTANÉ."
    pub fn loi_resonance(&self, a: &Holon, b: &Holon) -> bool {
        // Vecteurs identiques (même verbe)
        let meme_vecteur = match (&a.verbe, &b.verbe) {
            (Some(v1), Some(v2)) => v1 == v2,
            _ => false,
        };

        // Fréquences harmoniques
        let harmonise = a.frequence.resonance_avec(&b.frequence) > 0.5;

        // Polarités complémentaires
        let complementaire = matches!(
            (&a.polarite, &b.polarite),
            (Polarite::Besoin, Polarite::Offre) | (Polarite::Offre, Polarite::Besoin)
        );

        (meme_vecteur || harmonise) && complementaire
    }
}

impl std::fmt::Display for ResonanceMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbole = match self.connexion {
            TypeConnexion::ArcElectrique => "⚡",
            TypeConnexion::Harmonique => "🎵",
            TypeConnexion::Pont => "🌉",
            TypeConnexion::Faible => "~",
        };
        write!(
            f,
            "{} {} ({:.0}%)",
            symbole,
            match self.connexion {
                TypeConnexion::ArcElectrique => "ARC ÉLECTRIQUE",
                TypeConnexion::Harmonique => "Harmonique",
                TypeConnexion::Pont => "Pont",
                TypeConnexion::Faible => "Faible",
            },
            self.score * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frequence::FrequenceSolfeggio;
    use crate::verbes::{Verbe, VerbeBDO};

    #[test]
    fn test_arc_electrique() {
        let engine = ResonanceEngine::default();

        let besoin = Holon::avec_verbe("Affamé", Verbe::Bdo(VerbeBDO::Nourrir))
            .polarite(Polarite::Besoin)
            .frequence(FrequenceSolfeggio::Hz528);

        let offre = Holon::avec_verbe("Fermier", Verbe::Bdo(VerbeBDO::Nourrir))
            .polarite(Polarite::Offre)
            .frequence(FrequenceSolfeggio::Hz528);

        let result = engine.calculer(&besoin, &offre);
        assert!(result.is_some());

        let m = result.unwrap();
        assert_eq!(m.connexion, TypeConnexion::ArcElectrique);
        assert!(m.score > 0.7);
    }

    #[test]
    fn test_loi_resonance() {
        let engine = ResonanceEngine::default();

        let a = Holon::avec_verbe("A", Verbe::Bdo(VerbeBDO::Guerir))
            .polarite(Polarite::Besoin);

        let b = Holon::avec_verbe("B", Verbe::Bdo(VerbeBDO::Guerir))
            .polarite(Polarite::Offre);

        assert!(engine.loi_resonance(&a, &b));
    }
}
