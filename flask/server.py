from flask import Flask, request

from kafka import KafkaProducer
import json
from waitress import serve

app = Flask(__name__)



@app.route('/send_to_kafka', methods=['POST'])
def send_to_kafka():
    producer = KafkaProducer(bootstrap_servers=['localhost:29092'])
    data = request.get_json()
    producer.send('test', json.dumps(data).encode('utf-8'))
    return json.dumps({"status": "success"})

if __name__ == '__main__':
    serve(app, port=5001)


