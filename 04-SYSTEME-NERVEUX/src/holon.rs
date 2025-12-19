//! Holon - La Cellule de base du Bio-Réacteur oXc
//!
//! Un Holon n'est pas une simple donnée, c'est un VECTEUR VIVANT composé de :
//! - Le Verbe Moteur (L'ID unique, l'action)
//! - La Fréquence Hz (La signature vibratoire)
//! - La Méta-Monnaie (La valeur économique)
//! - La Polarité (Besoin/Demande/Offre)
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH
//! UK Patent GB2521986.6 | US Copyright 1-15060416332

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent_type::AgentType;
use crate::frequence::FrequenceSolfeggio;
use crate::lifecycle::Lifecycle;
use crate::verbes::{Polarite, Verbe};

/// Holon - Entité vivante et pensante du Bio-Réacteur oXc
///
/// C'est la cellule autonome qui peut percevoir, agir, se différencier et transmuter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holon {
    /// Identifiant unique (UUID v4)
    pub id: Uuid,

    /// Nom du Holon
    pub nom: String,

    /// Verbe Moteur - L'action principale de ce Holon
    pub verbe: Option<Verbe>,

    /// Signature fréquentielle (Hz Solfeggio)
    pub frequence: FrequenceSolfeggio,

    /// Type d'agent (différenciation épigénétique)
    pub agent_type: AgentType,

    /// État du cycle de vie
    pub lifecycle: Lifecycle,

    /// Polarité BDO (Besoin/Demande/Offre)
    pub polarite: Polarite,

    /// Couleur hex pour l'affichage
    pub couleur_hex: String,

    /// Méta-données libres
    pub meta: HolonMeta,

    /// Timestamp de création (Unix epoch)
    pub created_at: u64,

    /// Timestamp de dernière modification
    pub updated_at: u64,
}

/// Méta-données d'un Holon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HolonMeta {
    /// Description textuelle
    pub description: Option<String>,

    /// Tags / étiquettes
    pub tags: Vec<String>,

    /// Valeur économique (Méta-Monnaie)
    pub valeur: Option<f64>,

    /// Unité de la valeur
    pub unite_valeur: Option<String>,

    /// Source / origine
    pub source: Option<String>,

    /// Données additionnelles (JSON)
    pub extra: Option<serde_json::Value>,
}

impl Holon {
    /// Crée un nouveau Holon avec les valeurs par défaut
    pub fn new(nom: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: Uuid::new_v4(),
            nom: nom.into(),
            verbe: None,
            frequence: FrequenceSolfeggio::default(),
            agent_type: AgentType::default(),
            lifecycle: Lifecycle::default(),
            polarite: Polarite::Demande,
            couleur_hex: AgentType::default().couleur_hex().to_string(),
            meta: HolonMeta::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Crée un Holon avec un verbe moteur
    pub fn avec_verbe(nom: impl Into<String>, verbe: Verbe) -> Self {
        let mut holon = Self::new(nom);
        holon.verbe = Some(verbe);
        holon.frequence = verbe.frequence();
        holon.couleur_hex = holon.frequence.couleur_hex().to_string();
        holon
    }

    /// Builder pattern - définit le verbe
    pub fn verbe(mut self, verbe: Verbe) -> Self {
        self.verbe = Some(verbe);
        self.frequence = verbe.frequence();
        self
    }

    /// Builder pattern - définit la fréquence
    pub fn frequence(mut self, freq: FrequenceSolfeggio) -> Self {
        self.frequence = freq;
        self.couleur_hex = freq.couleur_hex().to_string();
        self
    }

    /// Builder pattern - définit le type d'agent
    pub fn agent(mut self, agent: AgentType) -> Self {
        self.agent_type = agent;
        self.couleur_hex = agent.couleur_hex().to_string();
        self
    }

    /// Builder pattern - définit la polarité
    pub fn polarite(mut self, pol: Polarite) -> Self {
        self.polarite = pol;
        self
    }

    /// Builder pattern - définit la description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.meta.description = Some(desc.into());
        self
    }

    /// Builder pattern - ajoute un tag
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.meta.tags.push(tag.into());
        self
    }

    /// Builder pattern - définit la valeur économique
    pub fn valeur(mut self, valeur: f64, unite: impl Into<String>) -> Self {
        self.meta.valeur = Some(valeur);
        self.meta.unite_valeur = Some(unite.into());
        self
    }

    /// Active le Holon (passe de Germe à Actif)
    pub fn activer(&mut self) -> Result<(), &'static str> {
        self.lifecycle = self.lifecycle.transitionner(Lifecycle::Actif)?;
        self.touch();
        Ok(())
    }

    /// Met le Holon en pause
    pub fn pause(&mut self) -> Result<(), &'static str> {
        self.lifecycle = self.lifecycle.transitionner(Lifecycle::EnPause)?;
        self.touch();
        Ok(())
    }

    /// Archive le Holon
    pub fn archiver(&mut self) -> Result<(), &'static str> {
        self.lifecycle = self.lifecycle.transitionner(Lifecycle::Archive)?;
        self.touch();
        Ok(())
    }

    /// Transmute le Holon (transformation)
    pub fn transmuter(&mut self) -> Result<(), &'static str> {
        self.lifecycle = self.lifecycle.transitionner(Lifecycle::Transmute)?;
        self.touch();
        Ok(())
    }

    /// Composte le Holon (fin de vie, recyclage)
    pub fn composter(&mut self) -> Result<(), &'static str> {
        self.lifecycle = self.lifecycle.transitionner(Lifecycle::Composte)?;
        self.touch();
        Ok(())
    }

    /// Différencie l'agent selon un signal de l'environnement
    pub fn differencier(&mut self, signal: &str) {
        self.agent_type = AgentType::differencier(signal);
        self.couleur_hex = self.agent_type.couleur_hex().to_string();
        self.touch();
    }

    /// Calcule le score de résonance avec un autre Holon
    pub fn resonance_avec(&self, autre: &Holon) -> f64 {
        // Résonance de base sur les fréquences
        let freq_resonance = self.frequence.resonance_avec(&autre.frequence);

        // Bonus si même polarité compatible (Besoin ↔ Offre)
        let polarite_bonus = match (&self.polarite, &autre.polarite) {
            (Polarite::Besoin, Polarite::Offre) | (Polarite::Offre, Polarite::Besoin) => 0.3,
            (Polarite::Demande, _) | (_, Polarite::Demande) => 0.1,
            _ => 0.0,
        };

        // Bonus si même verbe
        let verbe_bonus = match (&self.verbe, &autre.verbe) {
            (Some(v1), Some(v2)) if v1 == v2 => 0.2,
            _ => 0.0,
        };

        (freq_resonance + polarite_bonus + verbe_bonus).clamp(0.0, 1.0)
    }

    /// Met à jour le timestamp de modification
    fn touch(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }
}

impl std::fmt::Display for Holon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} [{}] @ {} - {}",
            self.agent_type.emoji(),
            self.nom,
            self.lifecycle,
            self.frequence,
            self.polarite.symbole()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verbes::VerbeBDO;

    #[test]
    fn test_creation_holon() {
        let holon = Holon::new("Test");
        assert_eq!(holon.nom, "Test");
        assert_eq!(holon.lifecycle, Lifecycle::Germe);
        assert_eq!(holon.agent_type, AgentType::Veilleur);
    }

    #[test]
    fn test_holon_avec_verbe() {
        let holon = Holon::avec_verbe("Nourricier", Verbe::Bdo(VerbeBDO::Nourrir));
        assert_eq!(holon.frequence, FrequenceSolfeggio::Hz528);
    }

    #[test]
    fn test_activation() {
        let mut holon = Holon::new("Test");
        assert!(holon.activer().is_ok());
        assert_eq!(holon.lifecycle, Lifecycle::Actif);
    }

    #[test]
    fn test_differenciation() {
        let mut holon = Holon::new("Test");
        holon.differencier("Je veux protéger ma famille");
        assert_eq!(holon.agent_type, AgentType::Gardien);
    }

    #[test]
    fn test_resonance() {
        let h1 = Holon::new("A").frequence(FrequenceSolfeggio::Hz528);
        let h2 = Holon::new("B").frequence(FrequenceSolfeggio::Hz528);
        let score = h1.resonance_avec(&h2);
        assert!(score > 0.9); // Même fréquence = haute résonance
    }
}
