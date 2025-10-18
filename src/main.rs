use rand::{Rng, SeedableRng, rngs::StdRng, thread_rng};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum GoblinMessage {
    Report {
        id: usize,
        pos: Position,
        ore: u32,
        fatigue: f64,
    },
    Deposit {
        id: usize,
        ore: u32,
    },
}

async fn goblin_task(id: usize, tx: mpsc::Sender<GoblinMessage>) {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(id as u64);
    let mut pos = Position { x: 0, y: 0 };
    let mut ore = 0;
    let mut fatigue = 0.0;

    loop {
        pos.x += rng.random_range(-1..=1);
        pos.y += rng.random_range(-1..=1);
        fatigue += rng.random_range(1.0..5.0);

        if rng.random_bool(0.3) {
            let found = rng.random_range(1..=5);
            ore += found;
            println!(
                "🪓 Goblin #{id} encontró {found} de oro en ({}, {})!",
                pos.x, pos.y
            );
        }

        if fatigue > 30.0 {
            println!("🏃 Goblin #{id} vuelve a la fortaleza con {ore} de oro!");
            tx.send(GoblinMessage::Deposit { id, ore }).await.ok();
            ore = 0;
            fatigue = 0.0;
        } else {
            tx.send(GoblinMessage::Report {
                id,
                pos,
                ore,
                fatigue,
            })
            .await
            .ok();
        }

        tokio::time::sleep(Duration::from_millis(rng.random_range(100..500))).await;
    }
}

async fn fortress_task(mut rx: mpsc::Receiver<GoblinMessage>) {
    let mut total_ore = 0u32;
    let mut reports = Vec::new();

    while let Some(msg) = rx.recv().await {
        match msg {
            GoblinMessage::Report {
                id,
                pos,
                ore,
                fatigue,
            } => {
                if reports.len() <= id {
                    reports.resize(id + 1, None)
                }
                reports[id] = Some((pos, ore, fatigue));
            }
            GoblinMessage::Deposit { id, ore } => {
                total_ore += ore;
                println!("💰 Goblin #{id} depositó {ore} de oro. Total: {total_ore}")
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    // Spawn la fortaleza
    let fortress = tokio::spawn(fortress_task(rx));

    // Spawn los goblins
    let mut goblin_handles = vec![];
    for id in 0..5 {
        let tx_clone = tx.clone();
        goblin_handles.push(tokio::spawn(goblin_task(id, tx_clone)));
    }

    // Soltar el tx original
    drop(tx);

    // Esperar a que termine la fortaleza (cuando todos los tx se cierren)
    fortress.await.unwrap();

    // O si quieres esperar a los goblins específicamente:
    // for handle in goblin_handles {
    //     handle.await.unwrap();
    // }
}
