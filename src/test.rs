use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use github_flows::{
    event_handler, get_octo, listen_to_event,
    octocrab::models::{events::payload::EventPayload, reactions::ReactionContent},
    GithubLogin,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    // `some_login` must be authed in flows.network
    listen_to_event(&GithubLogin::Default, "alabulei1", "a-test", vec!["issue_comment"]).await;
    listen_to_event(&GithubLogin::Default, "flows-network", "docs", vec!["issue_comment"]).await;
}

#[event_handler]
async fn handler(payload: EventPayload) {
    if let EventPayload::IssueCommentEvent(e) = payload {
        let comment_id = e.comment.id.0;
        logger::init();
    log::debug!("Running test");

        // installed app login
        let octo = get_octo(&GithubLogin::Default;

            log::debug!("login");

        let _reaction = octo
            .issues("some_owner", "some_repo")
            .create_comment_reaction(comment_id, ReactionContent::Rocket)
            .await
            .unwrap();
           log::debug!("success");
    };
}
