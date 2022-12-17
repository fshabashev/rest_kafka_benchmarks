from fastapi import FastAPI
from fastapi import HTTPException
from kafka import KafkaProducer
from pydantic import BaseModel

app = FastAPI()

producer = KafkaProducer(bootstrap_servers='localhost:29092')


class Item(BaseModel):
    text: str


@app.post("/send_to_topic")
def send_to_topic(data: Item):
    try:
        producer.send('test', data.text.encode())
        return {"message": "Successfully sent message to topic"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

