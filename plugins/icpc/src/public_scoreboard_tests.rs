use broccoli_server_sdk::permissions::CONTEST_MANAGE;
use broccoli_server_sdk::scoreboard::Viewer;
use broccoli_server_sdk::types::{PluginHttpRequest, Verdict};
use serde_json::json;

use crate::config::freeze_view;
use crate::standings::{StandingsSubmission, counts_as_live};

const DURATION: i64 = 300 * 60_000;
const NOW: i64 = 250 * 60_000;

fn viewer(public: bool) -> Viewer {
    let req: PluginHttpRequest = serde_json::from_value(json!({
        "method": "GET",
        "query": if public { json!({ "view": "public" }) } else { json!({}) },
        "auth": { "user_id": 7, "username": "organizer", "permissions": [CONTEST_MANAGE] }
    }))
    .unwrap();
    Viewer::from_request(&req)
}

#[test]
fn public_organizer_view_is_frozen_and_cannot_reveal() {
    let viewer = viewer(true);
    let freeze = freeze_view(60, "during", viewer.can_view_all, false, DURATION, NOW);
    assert!(freeze.frozen);
    assert!(!freeze.can_reveal);
    assert_eq!(freeze.freeze_start_ms, 240 * 60_000);
}

#[test]
fn public_view_does_not_keep_the_organizers_own_frozen_solve_live() {
    let viewer = viewer(true);
    let freeze = freeze_view(60, "during", viewer.can_view_all, false, DURATION, NOW);
    let mut submission = StandingsSubmission {
        submission_id: 1,
        user_id: 7,
        problem_id: 1,
        verdict: Some(Verdict::Accepted),
        status: "Judged".into(),
        elapsed_ms: 245 * 60_000,
    };
    assert!(!counts_as_live(
        &submission,
        freeze.freeze_start_ms,
        viewer.user_id,
    ));
    submission.user_id = 8;
    assert!(!counts_as_live(
        &submission,
        freeze.freeze_start_ms,
        viewer.user_id,
    ));
    submission.elapsed_ms = 239 * 60_000;
    assert!(counts_as_live(
        &submission,
        freeze.freeze_start_ms,
        viewer.user_id,
    ));
}

#[test]
fn default_organizer_view_is_unchanged() {
    let viewer = viewer(false);
    let freeze = freeze_view(60, "during", viewer.can_view_all, false, DURATION, NOW);
    assert!(!freeze.frozen);
    assert!(freeze.can_reveal);
    assert_eq!(viewer.user_id, Some(7));
}

#[test]
fn public_view_remains_frozen_after_contest_until_revealed() {
    let viewer = viewer(true);
    let after = 400 * 60_000;
    let frozen = freeze_view(60, "after", viewer.can_view_all, false, DURATION, after);
    assert!(frozen.frozen);
    assert!(!frozen.can_reveal);
    let revealed = freeze_view(60, "after", viewer.can_view_all, true, DURATION, after);
    assert!(!revealed.frozen);
    assert!(!revealed.can_reveal);
}
