const express = require('express'); // import express
const app = express(); // create an express app
const kafka = require('kafka-node'); // import the kafka-node module

// create a new Kafka Producer
const producer = new kafka.Producer(
    new kafka.KafkaClient({
        kafkaHost: 'localhost:29092', // provide the connection string for your Apache Kafka cluster here
    })
);

app.use(express.json()); // enable parsing of JSON bodies in requests

// create a POST endpoint at the path '/message'
app.post('/kafka_endpoint', (req, res) => {
    const { body } = req; // get the request body

    // send the message to the Kafka topic 'messages'
    producer.send(
        [
            {
                topic: 'test',
                messages: "Hello from NodeJS",
            },
        ],
        (error, data) => {
            if (error) {
                // if there was an error, return a 500 status code and the error message
                return res.status(500).send(error.message);
            }

            // if the message was sent successfully, return a 200 status code
            return res.sendStatus(200);
        }
    );
});

// start the server on port 3000
app.listen(8082, () => {
    console.log('Listening on port 8082');
});
