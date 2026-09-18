//! Viewer policy for read-only scoreboard endpoints.
//!
//! Apply this only after checking access to the contest with the original
//! request. Public view removes scoring privileges; it never grants access to
//! a private or inactive contest and must not be used to authorize writes.

use crate::permissions::CONTEST_MANAGE;
use crate::types::PluginHttpRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewer {
    pub user_id: Option<i32>,
    pub can_view_all: bool,
}

impl Viewer {
    /// `view=public` gives even an authenticated organizer the same scoring
    /// visibility as an anonymous spectator, including no own-score exception.
    pub fn from_request(req: &PluginHttpRequest) -> Self {
        if req.query.get("view").is_some_and(|view| view == "public") {
            return Self {
                user_id: None,
                can_view_all: false,
            };
        }
        Self {
            user_id: req.user_id(),
            can_view_all: req.has_permission(CONTEST_MANAGE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn request(
        user_id: Option<i32>,
        permissions: &[&str],
        view: Option<&str>,
    ) -> PluginHttpRequest {
        let auth = user_id.map(|id| {
            json!({ "user_id": id, "username": "viewer", "permissions": permissions })
        });
        let mut req: PluginHttpRequest =
            serde_json::from_value(json!({ "method": "GET", "auth": auth })).unwrap();
        if let Some(view) = view {
            req.query.insert("view".into(), view.into());
        }
        req
    }

    #[test]
    fn public_view_removes_organizer_privileges_and_identity() {
        let req = request(Some(7), &[CONTEST_MANAGE], Some("public"));
        assert_eq!(
            Viewer::from_request(&req),
            Viewer {
                user_id: None,
                can_view_all: false,
            },
        );
    }

    #[test]
    fn public_view_removes_contestants_own_score_exception() {
        let req = request(Some(7), &[], Some("public"));
        assert_eq!(
            Viewer::from_request(&req),
            Viewer {
                user_id: None,
                can_view_all: false,
            },
        );
    }

    #[test]
    fn anonymous_views_remain_unprivileged() {
        for view in [None, Some("public"), Some("admin")] {
            assert_eq!(
                Viewer::from_request(&request(None, &[], view)),
                Viewer {
                    user_id: None,
                    can_view_all: false,
                },
            );
        }
    }

    #[test]
    fn normal_view_preserves_existing_permissions() {
        assert_eq!(
            Viewer::from_request(&request(Some(7), &[CONTEST_MANAGE], None)),
            Viewer {
                user_id: Some(7),
                can_view_all: true,
            },
        );
        assert_eq!(
            Viewer::from_request(&request(Some(8), &[], None)),
            Viewer {
                user_id: Some(8),
                can_view_all: false,
            },
        );
    }

    #[test]
    fn unknown_view_does_not_grant_permissions() {
        for view in ["admin", "true", "PUBLIC", ""] {
            let viewer = Viewer::from_request(&request(Some(8), &[], Some(view)));
            assert!(!viewer.can_view_all);
            assert_eq!(viewer.user_id, Some(8));
        }
    }

    #[test]
    fn public_view_does_not_mutate_auth_used_by_other_endpoints() {
        let req = request(Some(7), &[CONTEST_MANAGE], Some("public"));
        let _ = Viewer::from_request(&req);
        assert_eq!(req.user_id(), Some(7));
        assert!(req.has_permission(CONTEST_MANAGE));
    }
}
