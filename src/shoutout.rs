from datetime import timedelta

from boto3.dynamodb.conditions import Attr

from utils import current_time

class Shoutout:
    def __init__(self, sender, recipients, message, timestamp=None):
        self.sender = sender
        self.recipients = recipients
        self.message = message
        self.timestamp = timestamp or current_time().timestamp()

    @classmethod
    def from_body(cls, body):
        publicity, rest = body['text'][0].strip().split(' ', 1)
        if publicity == 'public':
            user_id = body['user_id'][0]
            sender = f'<@{user_id}>'
        elif publicity == 'anonymous':
            sender = ''
        else:
            raise Exception('Publicity must be "public" or "anonymous"')

        recipient, rest = rest.split(' ', 1)
        if not recipient.startswith('<@'):
            raise Exception('recipients must be users')

        recipients = [recipient]
        while rest.startswith('<@'):
            recipient, rest = rest.split(' ', 1)
            recipients.append(recipient)

        if rest.startswith('"') or rest.startswith('\''):
            rest = rest[1:]

        if rest.endswith('"') or rest.endswith('\''):
            rest = rest[:-1]

        return Shoutout(sender, recipients, rest)

    def to_json(self):
        return {
            'timestamp': int(self.timestamp),
            'sender': self.sender,
            'recipients': ','.join(self.recipients),
            'message': self.message,
        }

    def insert_into(self, table):
        return table.put_item(Item=self.to_json())

    @classmethod
    def for_last_week(cls, table):
        one_week_ago = current_time() - timedelta(days = 7)
        timestamp = int(one_week_ago.timestamp())
        response = table.scan(
            FilterExpression=Attr('timestamp').gt(timestamp))

        return [Shoutout(i['sender'], i['recipients'].split(','), i['message'], i['timestamp'])
            for i in response['Items']]
            
    def publicity(self):
        return 'public' if self.sender else 'anonymous'
            
    def recipient_list(self):
        if len(self.recipients) == 1:
            return self.recipients[0]
        elif len(self.recipients) == 2:
            return f'{self.recipients[0]} and {self.recipients[1]}'
        else:
            all_but_last = ', '.join(self.recipients[:-1])
            return f'{all_but_last}, and {self.recipients[-1]}'

    def __str__(self):
        from_sender = (' from ' + self.sender) if self.sender else ''
        return f'Shoutout to {self.recipient_list()} for _"{self.message}"_{from_sender}'

