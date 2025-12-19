//! Dialogueur - Machine à États pour la co-construction dialogique
//!
//! C'est le cœur de la rupture paradigmatique oXc :
//! Google attend des mots-clés → oXc POSE DES QUESTIONS.
//!
//! Input : "NOURRIR"
//! Output : Séquence de questions → Objet transactionnel structuré
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::verbes::Verbe;
use crate::ontologies::{Ontologie, Question, TypeReponse};
use crate::holon::Holon;
use crate::frequence::FrequenceSolfeggio;

/// État du dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EtatDialogue {
    /// Dialogue pas encore démarré
    Initial,
    /// Dialogue en cours avec index de l'étape actuelle
    EnCours {
        etape_index: usize,
        donnees: HashMap<String, String>,
    },
    /// Dialogue terminé avec l'objet transactionnel construit
    Termine {
        resultat: HashMap<String, String>,
    },
    /// Dialogue en erreur
    Erreur(String),
    /// Dialogue annulé par l'utilisateur
    Annule,
}

/// Session de dialogue pour un Verbe
#[derive(Debug, Clone)]
pub struct SessionDialogue {
    /// Verbe moteur de cette session
    pub verbe: Verbe,
    /// Ontologie chargée pour ce verbe
    pub ontologie: Ontologie,
    /// État actuel du dialogue
    pub etat: EtatDialogue,
    /// Historique des réponses avec timestamps
    pub historique: Vec<ReponseHistorique>,
}

/// Entrée dans l'historique des réponses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReponseHistorique {
    pub question_id: String,
    pub question_texte: String,
    pub reponse: String,
    pub timestamp: u64,
}

/// Résultat de la validation d'une réponse
#[derive(Debug)]
pub enum ValidationResult {
    Valide,
    Invalide(String),
    Vide,
}

impl SessionDialogue {
    /// Démarre une nouvelle session de dialogue pour un Verbe
    pub fn demarrer(verbe: Verbe) -> Result<Self, String> {
        let ontologie = Ontologie::charger(&verbe)
            .ok_or_else(|| format!("Pas d'ontologie connue pour le verbe {:?}", verbe))?;

        Ok(Self {
            verbe,
            ontologie,
            etat: EtatDialogue::EnCours {
                etape_index: 0,
                donnees: HashMap::new(),
            },
            historique: Vec::new(),
        })
    }

    /// Obtient la question actuelle (si en cours)
    pub fn question_actuelle(&self) -> Option<&Question> {
        match &self.etat {
            EtatDialogue::EnCours { etape_index, .. } => {
                self.ontologie.sequence.get(*etape_index)
            }
            _ => None,
        }
    }

    /// Obtient le numéro de l'étape actuelle (1-indexed pour l'affichage)
    pub fn etape_actuelle(&self) -> Option<(usize, usize)> {
        match &self.etat {
            EtatDialogue::EnCours { etape_index, .. } => {
                Some((*etape_index + 1, self.ontologie.sequence.len()))
            }
            _ => None,
        }
    }

    /// Vérifie si le dialogue est terminé
    pub fn est_termine(&self) -> bool {
        matches!(self.etat, EtatDialogue::Termine { .. })
    }

    /// Vérifie si le dialogue est en cours
    pub fn est_en_cours(&self) -> bool {
        matches!(self.etat, EtatDialogue::EnCours { .. })
    }

    /// Valide une réponse selon le type attendu
    fn valider_reponse(&self, question: &Question, reponse: &str) -> ValidationResult {
        let reponse_trimmed = reponse.trim();

        // Vérifier si vide
        if reponse_trimmed.is_empty() {
            if question.obligatoire {
                return ValidationResult::Vide;
            } else if let Some(defaut) = &question.defaut {
                // Utiliser la valeur par défaut
                return ValidationResult::Valide;
            }
        }

        // Valider selon le type
        match &question.type_reponse {
            TypeReponse::Nombre => {
                if reponse_trimmed.parse::<i64>().is_err() {
                    return ValidationResult::Invalide("Veuillez entrer un nombre entier".into());
                }
            }
            TypeReponse::Decimal => {
                if reponse_trimmed.parse::<f64>().is_err() {
                    return ValidationResult::Invalide("Veuillez entrer un nombre".into());
                }
            }
            TypeReponse::Choix(options) => {
                // Accepter le numéro ou le texte exact
                let est_valide = options.iter().any(|o| {
                    o.to_lowercase() == reponse_trimmed.to_lowercase()
                }) || reponse_trimmed.parse::<usize>()
                    .map(|n| n > 0 && n <= options.len())
                    .unwrap_or(false);

                if !est_valide {
                    return ValidationResult::Invalide(format!(
                        "Choix invalide. Options: {}",
                        options.join(", ")
                    ));
                }
            }
            TypeReponse::Frequence => {
                if let Ok(hz) = reponse_trimmed.parse::<u16>() {
                    if FrequenceSolfeggio::from_hz(hz).is_none() {
                        return ValidationResult::Invalide(
                            "Fréquence invalide. Valeurs: 174, 285, 396, 417, 432, 528, 639, 741, 852, 963".into()
                        );
                    }
                } else {
                    return ValidationResult::Invalide("Veuillez entrer une fréquence en Hz".into());
                }
            }
            TypeReponse::Booleen => {
                let lower = reponse_trimmed.to_lowercase();
                if !["oui", "non", "o", "n", "yes", "no", "y", "true", "false", "1", "0"].contains(&lower.as_str()) {
                    return ValidationResult::Invalide("Répondez par Oui ou Non".into());
                }
            }
            _ => {} // Texte, Date, Montant : accepter tel quel pour l'instant
        }

        ValidationResult::Valide
    }

    /// Enregistre une réponse et passe à la question suivante
    pub fn repondre(&mut self, reponse: &str) -> Result<String, String> {
        // Extraire l'état actuel
        let (etape_index, mut donnees) = match &self.etat {
            EtatDialogue::EnCours { etape_index, donnees } => (*etape_index, donnees.clone()),
            EtatDialogue::Termine { .. } => return Err("Dialogue déjà terminé".into()),
            EtatDialogue::Annule => return Err("Dialogue annulé".into()),
            EtatDialogue::Erreur(e) => return Err(format!("Dialogue en erreur: {}", e)),
            EtatDialogue::Initial => return Err("Dialogue pas encore démarré".into()),
        };

        // Récupérer la question actuelle
        let question = self.ontologie.sequence.get(etape_index)
            .ok_or("Index de question invalide")?
            .clone();

        // Valider la réponse
        let reponse_finale = match self.valider_reponse(&question, reponse) {
            ValidationResult::Valide => reponse.trim().to_string(),
            ValidationResult::Vide => {
                if let Some(defaut) = &question.defaut {
                    defaut.clone()
                } else {
                    return Err("Cette question est obligatoire".into());
                }
            }
            ValidationResult::Invalide(msg) => return Err(msg),
        };

        // Normaliser les choix (convertir numéro en texte)
        let reponse_normalisee = if let TypeReponse::Choix(options) = &question.type_reponse {
            if let Ok(n) = reponse_finale.parse::<usize>() {
                if n > 0 && n <= options.len() {
                    options[n - 1].clone()
                } else {
                    reponse_finale
                }
            } else {
                reponse_finale
            }
        } else {
            reponse_finale
        };

        // Enregistrer la réponse
        donnees.insert(question.id.clone(), reponse_normalisee.clone());

        // Ajouter à l'historique
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.historique.push(ReponseHistorique {
            question_id: question.id.clone(),
            question_texte: question.texte.clone(),
            reponse: reponse_normalisee,
            timestamp: now,
        });

        // Passer à l'étape suivante
        let next_index = etape_index + 1;

        if next_index >= self.ontologie.sequence.len() {
            // Dialogue terminé !
            self.etat = EtatDialogue::Termine { resultat: donnees };
            Ok("✓ Dialogue terminé. Objet transactionnel construit.".into())
        } else {
            // Continuer
            self.etat = EtatDialogue::EnCours {
                etape_index: next_index,
                donnees,
            };
            Ok("✓ Réponse enregistrée.".into())
        }
    }

    /// Annule le dialogue
    pub fn annuler(&mut self) {
        self.etat = EtatDialogue::Annule;
    }

    /// Revient à la question précédente
    pub fn precedent(&mut self) -> Result<(), String> {
        if let EtatDialogue::EnCours { etape_index, donnees } = &self.etat {
            if *etape_index == 0 {
                return Err("Déjà à la première question".into());
            }

            let prev_index = etape_index - 1;
            let mut new_donnees = donnees.clone();

            // Retirer la réponse de la question actuelle si elle existe
            if let Some(q) = self.ontologie.sequence.get(prev_index) {
                new_donnees.remove(&q.id);
            }

            self.etat = EtatDialogue::EnCours {
                etape_index: prev_index,
                donnees: new_donnees,
            };

            // Retirer du historique
            if !self.historique.is_empty() {
                self.historique.pop();
            }

            Ok(())
        } else {
            Err("Impossible de revenir en arrière dans cet état".into())
        }
    }

    /// Construit un Holon à partir du résultat du dialogue
    pub fn construire_holon(&self) -> Option<Holon> {
        if let EtatDialogue::Termine { resultat } = &self.etat {
            let nom = resultat.values().next()
                .map(|s| s.clone())
                .unwrap_or_else(|| format!("{}", self.verbe));

            let mut holon = Holon::avec_verbe(nom, self.verbe);

            // Ajouter les métadonnées
            holon.meta.description = Some(format!(
                "Créé par dialogue {} - {} réponses",
                self.ontologie.verbe_nom,
                resultat.len()
            ));

            for (k, v) in resultat {
                holon.meta.tags.push(format!("{}:{}", k, v));
            }

            // Extraire la fréquence si présente
            if let Some(freq_str) = resultat.get("frequence") {
                if let Ok(hz) = freq_str.parse::<u16>() {
                    if let Some(freq) = FrequenceSolfeggio::from_hz(hz) {
                        holon.frequence = freq;
                    }
                }
            }

            Some(holon)
        } else {
            None
        }
    }

    /// Obtient le résultat final (si terminé)
    pub fn resultat(&self) -> Option<&HashMap<String, String>> {
        if let EtatDialogue::Termine { resultat } = &self.etat {
            Some(resultat)
        } else {
            None
        }
    }
}

/// Formatte une question pour l'affichage CLI
pub fn formater_question(question: &Question, etape: (usize, usize)) -> String {
    let mut output = format!(
        "\n[{}/{}] {}",
        etape.0, etape.1, question.texte
    );

    // Ajouter les options si c'est un choix
    if let TypeReponse::Choix(options) = &question.type_reponse {
        output.push_str("\n");
        for (i, opt) in options.iter().enumerate() {
            output.push_str(&format!("  {}. {}\n", i + 1, opt));
        }
    }

    // Ajouter l'aide si présente
    if let Some(aide) = &question.aide {
        output.push_str(&format!("  💡 {}\n", aide));
    }

    // Indiquer si optionnel avec défaut
    if !question.obligatoire {
        if let Some(defaut) = &question.defaut {
            output.push_str(&format!("  (Optionnel, défaut: {})\n", defaut));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verbes::VerbeBDO;

    #[test]
    fn test_demarrer_session() {
        let verbe = Verbe::Bdo(VerbeBDO::Nourrir);
        let session = SessionDialogue::demarrer(verbe);
        assert!(session.is_ok());
        assert!(session.unwrap().est_en_cours());
    }

    #[test]
    fn test_dialogue_complet() {
        let verbe = Verbe::Bdo(VerbeBDO::Nourrir);
        let mut session = SessionDialogue::demarrer(verbe).unwrap();

        // Répondre à toutes les questions
        assert!(session.repondre("Des pommes").is_ok());
        assert!(session.repondre("10").is_ok());
        assert!(session.repondre("1").is_ok()); // Choix "Immédiat"
        assert!(session.repondre("Paris").is_ok());

        assert!(session.est_termine());

        let resultat = session.resultat().unwrap();
        assert_eq!(resultat.get("quoi"), Some(&"Des pommes".to_string()));
        assert_eq!(resultat.get("quantite"), Some(&"10".to_string()));
    }

    #[test]
    fn test_construire_holon() {
        let verbe = Verbe::Bdo(VerbeBDO::Nourrir);
        let mut session = SessionDialogue::demarrer(verbe).unwrap();

        session.repondre("Légumes bio").unwrap();
        session.repondre("5").unwrap();
        session.repondre("Demain").unwrap();
        session.repondre("Lyon").unwrap();

        let holon = session.construire_holon();
        assert!(holon.is_some());
    }
}
