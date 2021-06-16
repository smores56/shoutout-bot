import json
import boto3
import http.client

from dateutil.tz import gettz
from datetime import datetime


def current_time():
    return datetime.now(tz=gettz('US/Eastern'))


def get_table():
    dynamodb = boto3.resource('dynamodb')
    return dynamodb.Table('shoutouts')


def respond(err, res=None):
    if err:
        body = {
            'response_type': 'ephemeral',
            'text': str(err),
        }
    else:
        body = res
    
    return {
        'statusCode': '200',
        'body': json.dumps(body),
        'headers': {
            'Content-Type': 'application/json',
        },
    }


def markdown_section(text):
    return {
        "type": "section",
        "text": {
            "type": "mrkdwn",
            "text": text
        }
    }


def slack_message(sections):
    return {
        "blocks": sections
    }


def send_messages(messages):
    conn = http.client.HTTPSConnection('hooks.slack.com')
    for message in messages:
        conn.request('POST', '/services/THY9C1C8H/B0187UYSFRB/7uYJQzGRaPtoijDP7HiIB1dl',
            json.dumps(slack_message(message)))
        conn.getresponse().read()
