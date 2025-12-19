//! # Système Nerveux D13 - oXc Bio-Réacteur
//!
//! D13 — Bindings Rust, transduction signal ↔ symbole
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH
//! TOUS DROITS RÉSERVÉS - ALL RIGHTS RESERVED
//! UK Patent GB2521986.6 | US Copyright 1-15060416332

pub mod holon;
pub mod frequence;
pub mod verbes;
pub mod agent_type;
pub mod lifecycle;
pub mod resonance;
pub mod ontologies;
pub mod dialogue;

pub use holon::Holon;
pub use frequence::FrequenceSolfeggio;
pub use verbes::{Verbe, VerbeNatif, VerbeBDO, Polarite};
pub use agent_type::AgentType;
pub use lifecycle::Lifecycle;
pub use resonance::ResonanceEngine;
pub use ontologies::{Ontologie, Question, TypeReponse};
pub use dialogue::{SessionDialogue, EtatDialogue};
