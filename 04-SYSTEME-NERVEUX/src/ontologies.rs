//! Ontologies - Arbres de questions pour chaque Verbe
//!
//! Ce fichier définit QUELLES questions poser pour chaque Verbe.
//! C'est la traduction technique de la rupture paradigmatique oXc :
//! Google attend des mots-clés, oXc POSE DES QUESTIONS.
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH

use serde::{Deserialize, Serialize};
use crate::verbes::{Verbe, VerbeBDO, VerbeNatif};

/// Type de réponse attendue pour une question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeReponse {
    /// Texte libre
    Texte,
    /// Nombre entier
    Nombre,
    /// Nombre décimal (quantité, prix)
    Decimal,
    /// Choix parmi une liste
    Choix(Vec<String>),
    /// Montant avec devise
    Montant { devise: String },
    /// Date
    Date,
    /// Oui/Non
    Booleen,
    /// Fréquence Hz (pour résonance)
    Frequence,
}

/// Une Question atomique dans l'ontologie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    /// Identifiant unique de la question
    pub id: String,
    /// Texte de la question à poser
    pub texte: String,
    /// Type de réponse attendue
    pub type_reponse: TypeReponse,
    /// La question est-elle obligatoire ?
    pub obligatoire: bool,
    /// Valeur par défaut si non obligatoire
    pub defaut: Option<String>,
    /// Aide contextuelle
    pub aide: Option<String>,
}

impl Question {
    /// Crée une question texte simple
    pub fn texte(id: impl Into<String>, texte: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            texte: texte.into(),
            type_reponse: TypeReponse::Texte,
            obligatoire: true,
            defaut: None,
            aide: None,
        }
    }

    /// Crée une question nombre
    pub fn nombre(id: impl Into<String>, texte: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            texte: texte.into(),
            type_reponse: TypeReponse::Nombre,
            obligatoire: true,
            defaut: None,
            aide: None,
        }
    }

    /// Crée une question à choix multiples
    pub fn choix(id: impl Into<String>, texte: impl Into<String>, options: Vec<&str>) -> Self {
        Self {
            id: id.into(),
            texte: texte.into(),
            type_reponse: TypeReponse::Choix(options.into_iter().map(String::from).collect()),
            obligatoire: true,
            defaut: None,
            aide: None,
        }
    }

    /// Crée une question montant
    pub fn montant(id: impl Into<String>, texte: impl Into<String>, devise: &str) -> Self {
        Self {
            id: id.into(),
            texte: texte.into(),
            type_reponse: TypeReponse::Montant { devise: devise.into() },
            obligatoire: true,
            defaut: None,
            aide: None,
        }
    }

    /// Builder : rend optionnel avec défaut
    pub fn optionnel(mut self, defaut: impl Into<String>) -> Self {
        self.obligatoire = false;
        self.defaut = Some(defaut.into());
        self
    }

    /// Builder : ajoute une aide
    pub fn avec_aide(mut self, aide: impl Into<String>) -> Self {
        self.aide = Some(aide.into());
        self
    }
}

/// L'Ontologie : La recette complète pour un Verbe
/// Définit la séquence de questions qui co-construisent l'objet transactionnel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ontologie {
    /// Le verbe cible de cette ontologie
    pub verbe_nom: String,
    /// Description de l'ontologie
    pub description: String,
    /// Séquence ordonnée de questions
    pub sequence: Vec<Question>,
}

impl Ontologie {
    /// Charge l'ontologie spécifique au verbe
    pub fn charger(verbe: &Verbe) -> Option<Self> {
        match verbe {
            // ═══════════════════════════════════════════════════════════════
            // VERBES BDO (Actions concrètes)
            // ═══════════════════════════════════════════════════════════════

            Verbe::Bdo(VerbeBDO::Nourrir) => Some(Self {
                verbe_nom: "NOURRIR".into(),
                description: "Alimenter, sustenter un besoin".into(),
                sequence: vec![
                    Question::texte("quoi", "Quel type de nourriture ?")
                        .avec_aide("Ex: légumes, fruits, repas complet, aide alimentaire"),
                    Question::nombre("quantite", "Pour combien de personnes ?"),
                    Question::choix("urgence", "Pour quand ?", vec!["Immédiat", "Demain", "Cette semaine", "Régulier"]),
                    Question::texte("lieu", "Dans quelle zone géographique ?")
                        .optionnel("Local"),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Guerir) => Some(Self {
                verbe_nom: "GUÉRIR".into(),
                description: "Soigner, régénérer".into(),
                sequence: vec![
                    Question::choix("type", "Quel type de guérison ?", vec!["Physique", "Émotionnelle", "Énergétique", "Relationnelle"]),
                    Question::texte("symptome", "Décrivez le symptôme ou la situation"),
                    Question::choix("approche", "Quelle approche préférez-vous ?", vec!["Conventionnelle", "Alternative", "Holistique", "Ouvert"]),
                    Question {
                        id: "frequence".into(),
                        texte: "Fréquence de résonance souhaitée ?".into(),
                        type_reponse: TypeReponse::Frequence,
                        obligatoire: false,
                        defaut: Some("528".into()),
                        aide: Some("528 Hz = Guérison ADN, 285 Hz = Tissus".into()),
                    },
                ],
            }),

            Verbe::Bdo(VerbeBDO::Echanger) => Some(Self {
                verbe_nom: "ÉCHANGER".into(),
                description: "Troquer, permuter des ressources".into(),
                sequence: vec![
                    Question::texte("offre", "Qu'offrez-vous ?"),
                    Question::texte("demande", "Que cherchez-vous en échange ?"),
                    Question::montant("valeur_estimee", "Valeur estimée ?", "EUR"),
                    Question::choix("mode", "Mode d'échange ?", vec!["Troc direct", "Monnaie", "Temps", "Mixte"]),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Eduquer) => Some(Self {
                verbe_nom: "ÉDUQUER".into(),
                description: "Former, enseigner, transmettre".into(),
                sequence: vec![
                    Question::texte("sujet", "Quel sujet ou compétence ?"),
                    Question::choix("niveau", "Quel niveau ?", vec!["Débutant", "Intermédiaire", "Avancé", "Expert"]),
                    Question::choix("format", "Quel format ?", vec!["Individuel", "Groupe", "En ligne", "Présentiel"]),
                    Question::nombre("duree", "Durée estimée (en heures) ?").optionnel("1"),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Reparer) => Some(Self {
                verbe_nom: "RÉPARER".into(),
                description: "Restaurer, corriger".into(),
                sequence: vec![
                    Question::texte("objet", "Quoi réparer ?"),
                    Question::texte("probleme", "Quel est le problème ?"),
                    Question::choix("urgence", "Urgence ?", vec!["Critique", "Important", "Normal", "Quand possible"]),
                    Question::texte("competence", "Compétence requise ?").optionnel("Généraliste"),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Connecter) => Some(Self {
                verbe_nom: "CONNECTER".into(),
                description: "Relier, associer des entités".into(),
                sequence: vec![
                    Question::texte("qui", "Qui ou quoi connecter ?"),
                    Question::texte("avec", "Avec qui ou quoi ?"),
                    Question::choix("type_lien", "Type de connexion ?", vec!["Professionnel", "Personnel", "Projet", "Communauté"]),
                    Question::texte("objectif", "Objectif de la connexion ?"),
                ],
            }),

            Verbe::Bdo(VerbeBDO::CoCreer) => Some(Self {
                verbe_nom: "CO-CRÉER".into(),
                description: "Collaborer pour créer ensemble".into(),
                sequence: vec![
                    Question::texte("projet", "Quel projet co-créer ?"),
                    Question::texte("vision", "Décrivez la vision"),
                    Question::nombre("participants", "Combien de co-créateurs recherchés ?"),
                    Question::choix("engagement", "Niveau d'engagement ?", vec!["Ponctuel", "Régulier", "Intense", "Long terme"]),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Cultiver) => Some(Self {
                verbe_nom: "CULTIVER".into(),
                description: "Faire croître, développer".into(),
                sequence: vec![
                    Question::texte("quoi", "Que cultiver ?")
                        .avec_aide("Plante, compétence, relation, projet..."),
                    Question::texte("terrain", "Sur quel terrain/contexte ?"),
                    Question::choix("horizon", "Horizon temporel ?", vec!["Court terme", "Moyen terme", "Long terme", "Permanent"]),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Sauver) => Some(Self {
                verbe_nom: "SAUVER".into(),
                description: "Préserver, protéger d'un danger".into(),
                sequence: vec![
                    Question::texte("qui_quoi", "Qui ou quoi sauver ?"),
                    Question::texte("danger", "De quel danger ?"),
                    Question::choix("urgence", "Niveau d'urgence ?", vec!["Vital", "Urgent", "Important", "Préventif"]),
                    Question::texte("ressources", "Ressources disponibles ?"),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Transporter) => Some(Self {
                verbe_nom: "TRANSPORTER".into(),
                description: "Déplacer, acheminer".into(),
                sequence: vec![
                    Question::texte("quoi", "Quoi transporter ?"),
                    Question::texte("origine", "Point de départ ?"),
                    Question::texte("destination", "Destination ?"),
                    Question::choix("quand", "Quand ?", vec!["Maintenant", "Aujourd'hui", "Cette semaine", "Planifié"]),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Decider) => Some(Self {
                verbe_nom: "DÉCIDER".into(),
                description: "Choisir, trancher".into(),
                sequence: vec![
                    Question::texte("sujet", "Sujet de la décision ?"),
                    Question::texte("options", "Quelles options ?")
                        .avec_aide("Séparez par des virgules"),
                    Question::texte("criteres", "Critères de décision ?"),
                    Question::choix("methode", "Méthode de décision ?", vec!["Consensus", "Vote", "Expert", "Intuition"]),
                ],
            }),

            Verbe::Bdo(VerbeBDO::Demarrer) => Some(Self {
                verbe_nom: "DÉMARRER".into(),
                description: "Initier, lancer".into(),
                sequence: vec![
                    Question::texte("quoi", "Que démarrer ?"),
                    Question::texte("pourquoi", "Pourquoi maintenant ?"),
                    Question::texte("premiere_etape", "Quelle première étape ?"),
                    Question::choix("ressources", "Ressources nécessaires ?", vec!["Temps", "Argent", "Compétences", "Réseau", "Tout"]),
                ],
            }),

            // ═══════════════════════════════════════════════════════════════
            // VERBES NATIFS (Cycle de conscience)
            // ═══════════════════════════════════════════════════════════════

            Verbe::Natif(VerbeNatif::Percevoir) => Some(Self {
                verbe_nom: "PERCEVOIR".into(),
                description: "Observer sans jugement (CNV)".into(),
                sequence: vec![
                    Question::texte("observation", "Que percevez-vous ?")
                        .avec_aide("Décrivez les faits, sans interprétation"),
                    Question::choix("sens", "Par quel sens ?", vec!["Vue", "Son", "Ressenti", "Intuition", "Multiple"]),
                    Question::texte("contexte", "Dans quel contexte ?"),
                ],
            }),

            Verbe::Natif(VerbeNatif::Formuler) => Some(Self {
                verbe_nom: "FORMULER".into(),
                description: "Cristalliser le ressenti en structure".into(),
                sequence: vec![
                    Question::texte("ressenti", "Quel ressenti voulez-vous formuler ?"),
                    Question::choix("forme", "Sous quelle forme ?", vec!["Besoin", "Demande", "Offre", "Question"]),
                    Question::texte("formulation", "Votre formulation :"),
                ],
            }),

            Verbe::Natif(VerbeNatif::Manifester) => Some(Self {
                verbe_nom: "MANIFESTER".into(),
                description: "Exécuter dans la matière".into(),
                sequence: vec![
                    Question::texte("intention", "Quelle intention manifester ?"),
                    Question::texte("action_concrete", "Quelle action concrète ?"),
                    Question::texte("resultat_attendu", "Résultat attendu ?"),
                    Question {
                        id: "frequence".into(),
                        texte: "Fréquence de manifestation ?".into(),
                        type_reponse: TypeReponse::Frequence,
                        obligatoire: false,
                        defaut: Some("396".into()),
                        aide: Some("396 Hz = Manifestation dans la matière".into()),
                    },
                ],
            }),

            _ => None, // Verbes sans ontologie définie pour l'instant
        }
    }

    /// Liste tous les verbes avec ontologie disponible
    pub fn verbes_disponibles() -> Vec<&'static str> {
        vec![
            "NOURRIR", "GUÉRIR", "ÉCHANGER", "ÉDUQUER", "RÉPARER",
            "CONNECTER", "CO-CRÉER", "CULTIVER", "SAUVER", "TRANSPORTER",
            "DÉCIDER", "DÉMARRER",
            "PERCEVOIR", "FORMULER", "MANIFESTER",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charger_ontologie_nourrir() {
        let verbe = Verbe::Bdo(VerbeBDO::Nourrir);
        let onto = Ontologie::charger(&verbe);
        assert!(onto.is_some());
        assert_eq!(onto.unwrap().sequence.len(), 4);
    }

    #[test]
    fn test_charger_ontologie_guerir() {
        let verbe = Verbe::Bdo(VerbeBDO::Guerir);
        let onto = Ontologie::charger(&verbe);
        assert!(onto.is_some());
    }
}
