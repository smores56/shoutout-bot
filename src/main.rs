use anyhow::Context as _;
use lambda_runtime::{handler_fn, Context};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn func(event: Value, _: Context) -> anyhow::Result<Value> {
    let body = event
        .get("body")
        .context("No body")?
        .as_str()
        .context("Body was not a string")?;
    let params: Params = serde_url_params::from_str(body)?;

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}

struct Params {
    pub user_id: String,
    pub command: String,
}

// "body": "
//     token=gIkuvaNzQIHg97ATvDxqgjtO&\
//     team_id=T0001&\
//     team_domain=example&\
//     enterprise_id=E0001&\
//     enterprise_name=Globular%20Construct%20Inc&\
//     channel_id=C2147483705&\
//     channel_name=test&\
//     user_id=U2147483697&\
//     user_name=Steve&\
//     command=/shoutout&text=help%20me&\
//     response_url=https://hooks.slack.com/commands/1234/5678&\
//     trigger_id=13345224609.738474920.8088930838d88f008e0",

// from urllib.parse import parse_qs

// from shoutout import Shoutout
// from utils import respond, slack_message, markdown_section, get_table, current_time, send_messages

// def handler(event, context):
//     try:
//         table = get_table()

//         if event.get('runBatch', False):
//             return send_batch_of_shoutouts(table)

//         if event.get('sendReminder', False):
//             return send_reminder()

//         body = parse_qs(event['body'])

//         if body['text'][0].split(' ')[0] == 'help':
//             return respond(None, help_message())
//         else:
//             return save_shoutout(body, table)

//     except Exception as e:
//         return respond(e)

// def save_shoutout(body, table):
//     shoutout = Shoutout.from_body(body)
//     shoutout.insert_into(table)

//     now = current_time()
//     # If it's Friday
//     if now.weekday() == 4:
//         if now.hour < 12:
//             next_batch = 'later today'
//         else:
//             next_batch = 'next Friday'
//     else:
//         next_batch = 'this coming Friday'

//     return respond(None, slack_message([
//         markdown_section(
//             f'Thanks for the {shoutout.publicity()} shoutout! ' +
//             f'I\'m sure {shoutout.recipient_list()} will appreciate it.'
//         ),
//         markdown_section(
//             f'Look out for your shoutout (_among others_) *{next_batch}*.'
//         )
//     ]))

// def send_batch_of_shoutouts(table):
//     shoutouts = Shoutout.for_last_week(table)

//     if len(shoutouts) == 0:
//         messages = [
//             [
//                 markdown_section(
//                     'No shoutouts were made this week, folks. _Reminder: if you forgot how, checkout ' +
//                     'the_ `/shoutout help` _message to remember how to shoutout your friends._'
//                 ),
//                 markdown_section('Anyway, see you next Friday!')
//             ]
//         ]
//     else:
//         shoutout_count = '1 shoutout' if len(shoutouts) == 1 else f'{len(shoutouts)} shoutouts'
//         shoutout_messages = [[markdown_section(f'• {s}')] for s in shoutouts]
//         messages = [
//             [
//                 markdown_section(
//                     f'Shoutout time! We\'ve got {shoutout_count} this week.'
//                 )
//             ],
//             *shoutout_messages,
//             [
//                 markdown_section(
//                     '_Reminder: the_ `/shoutout help` _command knows how to make shoutouts._'
//                 ),
//                 markdown_section(
//                     'See you next Friday for more shoutouts!'
//                 )
//             ]
//         ]

//     send_messages(messages)

//     return respond(None, { 'success': True })

// def help_message():
//     return slack_message([
//         markdown_section(
//             'You can send public/anonymous shoutouts with the `/shoutout` command.'
//         ),
//         markdown_section(
//             'Use `/shoutout public <user(s)> <message>` to tie your username to a shoutout, ' +
//             'and `/shoutout anonymous <user(s)> <message>` to not include your username.'
//         ),
//         markdown_section(
//             'The `<user(s)>` argument is a space-separated list of usernames. Once the ' +
//             'usernames stop, the message begins. You don\'t need to wrap your `<message>` ' +
//             'in quotes (single or double), but if you do they will be trimmed.'
//         ),
//         markdown_section(
//             'If you have any more questions, feel free to DM me (_read: Sam Mohr_)!'
//         )
//     ])

// def send_reminder():
//     send_messages([
//         [
//             markdown_section(
//                 '_psst!_ It\'s almost time for `/shoutout`s. Will you take the charge?'
//             ),
//             markdown_section(
//                 '_Reminder: the_ `/shoutout help` _command knows how to make shoutouts._'
//             )
//         ]
//     ])

//     return respond(None, { 'success': True })
