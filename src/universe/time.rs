use std::time::{Duration};
use serenity::all::{ChannelId, CreateMessage, Http};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use chrono::{Utc};
use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use serenity::builder::CreateEmbed;
use crate::database::universe::Universe;
use crate::tr_locale;
use crate::travel::logic::HTTP_CLIENT;

pub static TIME_SLEEPER: Lazy<Arc<Mutex<Option<JoinHandle<()>>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
pub static PENDING_TIME_EVENTS: Lazy<Arc<Mutex<Vec<TimeEvent>>>> = Lazy::new(|| Arc::new(Mutex::new(vec![])));

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimePhase {
    Midnight,
    Sunrise,
    Noon,
    Sunset,
}

#[derive(Debug, Clone)]
pub struct TimeEvent {
    pub universe_id: ObjectId,
    pub trigger_timestamp: u64,
    pub phase_index: u64,
}

impl TimePhase {
    pub fn get_message_key(&self) -> &'static str {
        match self {
            TimePhase::Midnight => "time__midnight",
            TimePhase::Sunrise => "time__sunrise",
            TimePhase::Noon => "time__noon",
            TimePhase::Sunset => "time__sunset",
        }
    }

    pub fn from_index(index: u64) -> Self {
        match index % 4 {
            0 => TimePhase::Midnight,
            1 => TimePhase::Sunrise,
            2 => TimePhase::Noon,
            _ => TimePhase::Sunset,
        }
    }
}

pub async fn setup_universal_time() {
    let universes = match Universe::get_all_universes().await {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Failed to get universes for universal time setup: {:?}", e);
            return;
        }
    };

    let mut events = Vec::new();
    let now = Utc::now().timestamp() as u64;

    for universe in universes {
        let modifier = universe.global_time_modifier as f64 / 100.0;
        if modifier <= 0.0 { continue; }

        let phase_duration_secs = (21600.0 / modifier) as u64;
        let cycle_duration_secs = phase_duration_secs * 4;
        let origin_secs = (universe.time_origin_timestamp / 1000) as u64;

        // Calculer la phase actuelle
        let elapsed_secs = now.saturating_sub(origin_secs);
        let current_phase_idx = elapsed_secs / phase_duration_secs;
        
        // La prochaine phase est current_phase_idx + 1
        let next_phase_idx = current_phase_idx + 1;
        let next_trigger = origin_secs + (next_phase_idx * phase_duration_secs);

        events.push(TimeEvent {
            universe_id: universe.universe_id,
            trigger_timestamp: next_trigger,
            phase_index: next_phase_idx,
        });
    }

    if events.is_empty() {
        println!("Universal Time system initialized: 0 active universes.");
        return;
    }

    // Trier par timestamp croissant
    events.sort_by_key(|e| e.trigger_timestamp);
    
    {
        let mut pending = PENDING_TIME_EVENTS.lock().await;
        *pending = events;
    }

    // Lancer le premier sleeper
    let next_event = {
        let pending = PENDING_TIME_EVENTS.lock().await;
        pending.first().cloned()
    };

    if let Some(event) = next_event {
        let delay = event.trigger_timestamp.saturating_sub(now);
        let mut sleeper = TIME_SLEEPER.lock().await;
        *sleeper = Some(time_process(delay));
        println!("Universal Time system initialized: {} universes. Next event in {}s", PENDING_TIME_EVENTS.lock().await.len(), delay);
    }
}

fn time_process(delay: u64) -> JoinHandle<()> {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(delay)).await;

        let mut next_delay: Option<u64> = None;

        {
            let mut events = PENDING_TIME_EVENTS.lock().await;
            if events.is_empty() {
                let mut sleeper = TIME_SLEEPER.lock().await;
                *sleeper = None;
                return;
            }

            // Récupère l'évènement qui vient de finir
            let current_event = events.remove(0);

            // Exécute l'annonce
            let http_opt = {
                let lock = HTTP_CLIENT.lock().await;
                lock.clone()
            };

            if let Some(http) = http_opt {
                let phase = TimePhase::from_index(current_event.phase_index);
                tokio::spawn(process_universe_phase_change(current_event.universe_id, phase, http));
            }

            // Calcule le prochain évènement pour cet univers et le réinsère
            let universe_opt = match crate::database::universe::get_universe_by_id(current_event.universe_id).await {
                Ok(u) => u,
                Err(_) => None,
            };

            if let Some(universe) = universe_opt {
                let modifier = universe.global_time_modifier as f64 / 100.0;
                if modifier > 0.0 {
                    let phase_duration_secs = (21600.0 / modifier) as u64;
                    let origin_secs = (universe.time_origin_timestamp / 1000) as u64;
                    
                    let next_phase_idx = current_event.phase_index + 1;
                    let next_trigger = origin_secs + (next_phase_idx * phase_duration_secs);

                    let new_event = TimeEvent {
                        universe_id: universe.universe_id,
                        trigger_timestamp: next_trigger,
                        phase_index: next_phase_idx,
                    };

                    // Réinsertion triée
                    let i = events.partition_point(|e| e.trigger_timestamp < new_event.trigger_timestamp);
                    events.insert(i, new_event);
                }
            }

            // Prépare le prochain sleep
            if let Some(first) = events.first() {
                let now = Utc::now().timestamp() as u64;
                next_delay = Some(first.trigger_timestamp.saturating_sub(now));
            }
        }

        if let Some(delay) = next_delay {
            let mut sleeper = TIME_SLEEPER.lock().await;
            *sleeper = Some(time_process(delay));
        } else {
            let mut sleeper = TIME_SLEEPER.lock().await;
            *sleeper = None;
        }
    })
}

async fn process_universe_phase_change(universe_id: mongodb::bson::oid::ObjectId, phase: TimePhase, http: Arc<Http>) {
    let db_client = match crate::database::db_client::DB_CLIENT.get().cloned() {
        Some(client) => client,
        None => return,
    };
    let filter = mongodb::bson::doc! { "universe_id": universe_id };

    let mut cursor = match db_client
        .database(crate::database::db_namespace::VERSEENGINE_DB_NAME)
        .collection::<crate::database::server::Server>(crate::database::db_namespace::SERVERS_COLLECTION_NAME)
        .find(filter)
        .await {
            Ok(c) => c,
            Err(_) => return,
        };

    let bot_user_id = match http.get_current_user().await {
        Ok(u) => u.id,
        Err(_) => return,
    };

    use futures::TryStreamExt;
    loop {
        match cursor.try_next().await {
            Ok(Some(server)) => {
                if let Some(channel_id) = server.universal_time_channel_id {
                    let locale = match http.get_guild(server.server_id.into()).await {
                        Ok(g) => g.preferred_locale,
                        Err(_) => "fr".to_string()
                    };
                    let msg = tr_locale!(locale.as_str(), phase.get_message_key());
                    let http_clone = http.clone();
                    let channel = ChannelId::new(channel_id.id);
                    tokio::spawn(async move {
                        // Tenter de modifier le précédent message du bot dans ce salon ou en envoyer un nouveau
                        match channel.messages(&http_clone, serenity::all::GetMessages::new().limit(10)).await {
                            Ok(mut messages) => {
                                // Trouver le message le plus récent du bot
                                let bot_message_index = messages.iter().position(|m| m.author.id == bot_user_id);

                                if let Some(index) = bot_message_index {
                                    let mut bot_message = messages.remove(index);
                                    // Modifier le message existant
                                    if let Err(_) = bot_message.edit(&http_clone, serenity::all::EditMessage::new().content(msg.clone()).suppress_embeds(true)).await {
                                        // Si la modification échoue (ex: message supprimé), on en envoie un nouveau
                                        let _ = channel.send_message(&http_clone, CreateMessage::new().content(msg)).await;
                                    }
                                } else {
                                    // Aucun message trouvé, on en envoie un nouveau
                                    let _ = channel.send_message(&http_clone, CreateMessage::new().content(msg)).await;
                                }
                            },
                            Err(_) => {
                                // Erreur lors de la récupération, on tente l'envoi d'un nouveau message par défaut
                                let _ = channel.send_message(&http_clone, CreateMessage::new().content(msg)).await;
                            }
                        }
                    });
                }
            },
            Ok(None) => break,
            Err(_) => break,
        }
    }
}
