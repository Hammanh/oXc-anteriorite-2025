//! YATASANA - Interface Humain-Grille oXc
//!
//! Chat Alchimique — Le terminal où le Verbe devient Matière.
//!
//! © 2025 Marc Victor R BOUCHER alias HammÅnH
//! UK Patent GB2521986.6 | US Copyright 1-15060416332

use std::io::{self, Write};
use colored::*;

use systeme_nerveux::{
    Verbe, VerbeBDO, VerbeNatif,
    SessionDialogue, EtatDialogue,
    Ontologie,
    Holon, FrequenceSolfeggio, Polarite,
    ResonanceEngine,
    dialogue::formater_question,
};

const BANNER: &str = r#"
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║   ██╗   ██╗ █████╗ ████████╗ █████╗ ███████╗ █████╗ ███╗   ██╗ █████╗        ║
║   ╚██╗ ██╔╝██╔══██╗╚══██╔══╝██╔══██╗██╔════╝██╔══██╗████╗  ██║██╔══██╗       ║
║    ╚████╔╝ ███████║   ██║   ███████║███████╗███████║██╔██╗ ██║███████║       ║
║     ╚██╔╝  ██╔══██║   ██║   ██╔══██║╚════██║██╔══██║██║╚██╗██║██╔══██║       ║
║      ██║   ██║  ██║   ██║   ██║  ██║███████║██║  ██║██║ ╚████║██║  ██║       ║
║      ╚═╝   ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝       ║
║                                                                               ║
║                    oXc Bio-Réacteur — Chat Alchimique                         ║
║                         © 2025 HammÅnH                                        ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
"#;

fn main() {
    // Afficher la bannière
    println!("{}", BANNER.green());
    println!("{}", "Interface Humain-Grille activée. Fréquence: 528 Hz".cyan());
    println!();
    afficher_aide();

    // Stockage des Holons créés
    let mut holons: Vec<Holon> = Vec::new();
    let engine = ResonanceEngine::default();

    // Boucle principale
    loop {
        print!("{}", "\n⚡ oXc > ".yellow().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Commandes système
        match input.to_lowercase().as_str() {
            "quit" | "exit" | "q" => {
                println!("{}", "\n🙏 Que la Lumière guide ton chemin. À bientôt.\n".green());
                break;
            }
            "help" | "?" | "aide" => {
                afficher_aide();
                continue;
            }
            "verbes" | "list" => {
                afficher_verbes();
                continue;
            }
            "holons" | "ls" => {
                afficher_holons(&holons);
                continue;
            }
            "clear" | "cls" => {
                print!("\x1B[2J\x1B[1;1H");
                println!("{}", BANNER.green());
                continue;
            }
            "match" | "resonance" => {
                if holons.len() < 2 {
                    println!("{}", "⚠ Il faut au moins 2 Holons pour chercher des résonances.".yellow());
                } else {
                    chercher_resonances(&holons, &engine);
                }
                continue;
            }
            _ => {}
        }

        // Essayer de parser un verbe
        if let Some(verbe) = Verbe::from_str(input) {
            println!("\n{} Verbe détecté: {} @ {}",
                "✓".green(),
                format!("{}", verbe).cyan().bold(),
                format!("{}", verbe.frequence()).magenta()
            );

            // Démarrer le dialogue
            match SessionDialogue::demarrer(verbe) {
                Ok(mut session) => {
                    if let Some(holon) = executer_dialogue(&mut session) {
                        println!("\n{}", "═══ HOLON CRISTALLISÉ ═══".green().bold());
                        afficher_holon(&holon);
                        holons.push(holon);
                        println!("\n{}", format!("📦 {} Holons en mémoire", holons.len()).cyan());
                    }
                }
                Err(e) => {
                    println!("{} {}", "✗".red(), e);
                }
            }
        } else {
            // Commande inconnue
            println!("{} Verbe inconnu: '{}'. Tape {} pour la liste.",
                "?".yellow(),
                input.red(),
                "verbes".cyan()
            );
        }
    }
}

/// Exécute un dialogue complet et retourne le Holon construit
fn executer_dialogue(session: &mut SessionDialogue) -> Option<Holon> {
    println!("\n{}", format!("─── Dialogue: {} ───", session.ontologie.verbe_nom).blue());
    println!("{}", session.ontologie.description.dimmed());

    loop {
        match session.question_actuelle() {
            Some(question) => {
                let etape = session.etape_actuelle().unwrap();
                println!("{}", formater_question(question, etape).cyan());

                print!("{}", "→ ".yellow());
                io::stdout().flush().unwrap();

                let mut reponse = String::new();
                if io::stdin().read_line(&mut reponse).is_err() {
                    continue;
                }

                let reponse = reponse.trim();

                // Commandes spéciales pendant le dialogue
                match reponse.to_lowercase().as_str() {
                    "quit" | "q" | "annuler" => {
                        session.annuler();
                        println!("{}", "Dialogue annulé.".yellow());
                        return None;
                    }
                    "back" | "b" | "precedent" => {
                        if let Err(e) = session.precedent() {
                            println!("{}", e.yellow());
                        }
                        continue;
                    }
                    _ => {}
                }

                match session.repondre(reponse) {
                    Ok(msg) => {
                        println!("{}", msg.green());
                    }
                    Err(e) => {
                        println!("{} {}", "✗".red(), e);
                    }
                }
            }
            None => {
                // Dialogue terminé
                if let EtatDialogue::Termine { resultat } = &session.etat {
                    println!("\n{}", "─── Résultat ───".green());
                    for (k, v) in resultat {
                        println!("  {} → {}", k.cyan(), v.white());
                    }
                }
                break;
            }
        }
    }

    session.construire_holon()
}

/// Affiche un Holon formaté
fn afficher_holon(holon: &Holon) {
    println!("  {} {}", "ID:".dimmed(), holon.id.to_string().dimmed());
    println!("  {} {}", "Nom:".cyan(), holon.nom.white().bold());
    if let Some(verbe) = &holon.verbe {
        println!("  {} {}", "Verbe:".cyan(), format!("{}", verbe).yellow());
    }
    println!("  {} {}", "Fréquence:".cyan(), format!("{}", holon.frequence).magenta());
    println!("  {} {}", "Agent:".cyan(), format!("{}", holon.agent_type).green());
    println!("  {} {}", "Polarité:".cyan(), holon.polarite.symbole().blue());
    println!("  {} {}", "État:".cyan(), format!("{}", holon.lifecycle).white());

    if !holon.meta.tags.is_empty() {
        println!("  {} {}", "Tags:".dimmed(),
            holon.meta.tags.iter()
                .take(5)
                .map(|t| t.dimmed().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

/// Affiche la liste des Holons en mémoire
fn afficher_holons(holons: &[Holon]) {
    if holons.is_empty() {
        println!("{}", "Aucun Holon en mémoire. Commence par un VERBE !".yellow());
        return;
    }

    println!("\n{}", "═══ HOLONS EN MÉMOIRE ═══".green().bold());
    for (i, h) in holons.iter().enumerate() {
        println!("\n{}. {}", (i + 1).to_string().cyan(), h);
    }
    println!();
}

/// Cherche les résonances entre les Holons
fn chercher_resonances(holons: &[Holon], engine: &ResonanceEngine) {
    println!("\n{}", "═══ RÉSONANCES DÉTECTÉES ═══".magenta().bold());

    let mut found = false;
    for (i, h1) in holons.iter().enumerate() {
        for h2 in holons.iter().skip(i + 1) {
            if let Some(m) = engine.calculer(h1, h2) {
                found = true;
                println!(
                    "\n  {} {} {} {}",
                    h1.nom.cyan(),
                    "↔".yellow(),
                    h2.nom.cyan(),
                    format!("{}", m).green()
                );
            }
        }
    }

    if !found {
        println!("{}", "Aucune résonance significative trouvée.".dimmed());
    }
}

/// Affiche les verbes disponibles
fn afficher_verbes() {
    println!("\n{}", "═══ VERBES DISPONIBLES ═══".cyan().bold());

    println!("\n{}", "▸ VERBES BDO (Actions concrètes):".yellow());
    let bdo = [
        ("NOURRIR", "528 Hz", "Alimenter, sustenter"),
        ("GUÉRIR", "528 Hz", "Soigner, régénérer"),
        ("ÉCHANGER", "639 Hz", "Troquer, permuter"),
        ("ÉDUQUER", "852 Hz", "Former, enseigner"),
        ("RÉPARER", "528 Hz", "Restaurer, corriger"),
        ("CONNECTER", "639 Hz", "Relier, associer"),
        ("CO-CRÉER", "528 Hz", "Collaborer ensemble"),
        ("CULTIVER", "528 Hz", "Faire croître"),
        ("SAUVER", "741 Hz", "Préserver, protéger"),
        ("TRANSPORTER", "417 Hz", "Déplacer, acheminer"),
        ("DÉCIDER", "741 Hz", "Choisir, trancher"),
        ("DÉMARRER", "417 Hz", "Initier, lancer"),
    ];

    for (verbe, hz, desc) in bdo {
        println!("  {} {} - {}",
            verbe.green(),
            format!("({})", hz).magenta().dimmed(),
            desc.dimmed()
        );
    }

    println!("\n{}", "▸ VERBES NATIFS (Cycle de conscience):".yellow());
    let natifs = [
        ("PERCEVOIR", "852 Hz", "Observer sans jugement"),
        ("FORMULER", "741 Hz", "Cristalliser le ressenti"),
        ("MANIFESTER", "396 Hz", "Exécuter dans la matière"),
    ];

    for (verbe, hz, desc) in natifs {
        println!("  {} {} - {}",
            verbe.blue(),
            format!("({})", hz).magenta().dimmed(),
            desc.dimmed()
        );
    }
    println!();
}

/// Affiche l'aide
fn afficher_aide() {
    println!("{}", "═══ COMMANDES ═══".cyan().bold());
    println!("  {}        Liste les verbes disponibles", "verbes".green());
    println!("  {}        Liste les Holons en mémoire", "holons".green());
    println!("  {}         Cherche les résonances", "match".green());
    println!("  {}         Efface l'écran", "clear".green());
    println!("  {}          Quitter", "quit".green());
    println!();
    println!("{}", "═══ UTILISATION ═══".cyan().bold());
    println!("  Tape un {} pour démarrer un dialogue.", "VERBE".yellow());
    println!("  Exemple: {}", "NOURRIR".green());
    println!();
    println!("{}", "Pendant un dialogue:".dimmed());
    println!("  {}       Question précédente", "back".dimmed());
    println!("  {}       Annuler le dialogue", "quit".dimmed());
}
