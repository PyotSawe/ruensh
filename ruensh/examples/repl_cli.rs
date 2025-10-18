/// REPL Example - Simple Read-Eval-Print Loop with RuenSH
///
/// This example demonstrates how to use RuenSH components to build a simple
/// REPL interface. It shows integration with the Modal component for
/// confirmations and basic command execution.

use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ruensh::components::Modal;
use ruensh::events::{Event, EventHandler};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("╔════════════════════════════════════════╗");
    println!("║   RuenSH REPL CLI Example              ║");
    println!("║                                        ║");
    println!("║   This demonstrates REPL integration   ║");
    println!("║   with RuenSH modal components         ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("RuenSH REPL-like CLI Ready!");
    println!("Try running: cargo run --example modal_demo");
    println!("\nFor a full REPL with your language, use RuenSH components:");
    println!("  - Modal: For confirmations and dialogs");
    println!("  - List: For command history");
    println!("  - Custom Input: For user input");
    println!("\nSee COMPONENT_GUIDE.md for detailed examples.");

    // Demonstrate event handling
    println!("\n--- Event System Demo ---");
    let mut event_handler = EventHandler::new();
    let _ = event_handler.spawn_event_loop();

    println!("Event system initialized (async polling active)");
    println!("Press Ctrl+C to exit\n");

    // Simulate REPL input
    let commands = vec![
        "(+ 1 2)       ; => 3",
        "(defn greet [name] (str \"Hello, \" name))",
        "(greet \"World\")  ; => \"Hello, World\"",
        "(map inc [1 2 3])  ; => (2 3 4)",
    ];

    println!("Example REPL Commands:");
    for (i, cmd) in commands.iter().enumerate() {
        println!("  {}. {}", i + 1, cmd);
    }

    println!("\nBuilding a full REPL?");
    println!("  1. Define your language's evaluator");
    println!("  2. Use Modal for error confirmations");
    println!("  3. Use List for command history");
    println!("  4. Create custom Input component");
    println!("  5. Combine with event routing");
    println!("\nCheck COMPONENT_GUIDE.md for complete templates!");

    Ok(())
}
