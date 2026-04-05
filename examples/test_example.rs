// Test script for VoiceIt3 Rust SDK
// Run: cargo run --example test_example

use std::env;
use voiceit3::VoiceIt3;

fn main() {
    let api_key = env::var("VOICEIT_API_KEY").expect("Set VOICEIT_API_KEY env var");
    let api_token = env::var("VOICEIT_API_TOKEN").expect("Set VOICEIT_API_TOKEN env var");

    let vi = VoiceIt3::new(&api_key, &api_token);

    println!("CreateUser: {:?}", vi.create_user());
    println!("GetAllUsers: {:?}", vi.get_all_users());
    println!("CreateGroup: {:?}", vi.create_group("Test Group"));
    println!("GetAllGroups: {:?}", vi.get_all_groups());
    println!("GetPhrases: {:?}", vi.get_phrases("en-US"));

    println!("\nAll API calls completed successfully!");
}
