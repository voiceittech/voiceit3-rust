use std::env;
use voiceit3::client::VoiceIt3;

fn main() {
    let ak = env::var("VOICEIT_API_KEY").expect("Set VOICEIT_API_KEY");
    let at = env::var("VOICEIT_API_TOKEN").expect("Set VOICEIT_API_TOKEN");
    let vi = VoiceIt3::new(ak, at);
    let phrase = "never forget tomorrow is a new day";
    let td = "test-data";

    let r = vi.create_user().unwrap();
    let user_id: String = {
        let pos = r.find("usr_").unwrap();
        r[pos..pos+36].to_string()
    };
    println!("CreateUser: {}", if r.contains("SUCC") { "PASS" } else { "FAIL" });

    for i in 1..=3 {
        let r = vi.create_video_enrollment(&user_id, "en-US", phrase, &format!("{}/videoEnrollmentA{}.mov", td, i)).unwrap();
        println!("VideoEnrollment{}: {}", i, if r.contains("SUCC") { "PASS" } else { "FAIL" });
    }

    let r = vi.video_verification(&user_id, "en-US", phrase, &format!("{}/videoVerificationA1.mov", td)).unwrap();
    println!("VideoVerification: {}", if r.contains("SUCC") { "PASS" } else { "FAIL" });

    let _ = vi.delete_all_enrollments(&user_id);
    let _ = vi.delete_user(&user_id);
    println!("\nAll tests passed!");
}
